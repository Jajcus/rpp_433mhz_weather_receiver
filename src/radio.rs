
use defmt::{info, debug};

use embassy_rp::adc;
use embassy_rp::adc::AdcPin;
use embassy_rp::gpio;
use embassy_rp::pio;
use embassy_rp::peripherals;
use embassy_rp::peripherals::PIO0;
use embassy_rp::Peripheral;
use embassy_time::{Instant, Timer, Duration};
use embassy_rp::bind_interrupts;
use embassy_sync::channel::{Channel, Sender, Receiver};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_futures::join::join;
use fixed::traits::ToFixed;
use fixed_macro::types::U56F8;
use core::cmp::max;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => adc::InterruptHandler;
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

const MIN_PULSE_LENGTH : u32 = 50;

#[derive(Copy, Clone,Debug, PartialEq, Eq)]
pub enum PulseKind {
    Reset,
    Low,
    High
}

#[derive(Copy, Clone, Debug)]
pub struct Pulse {
    pub kind: PulseKind,
    pub length: u32,
}

const PULSE_RESET : Pulse = Pulse { kind: PulseKind::Reset, length: 0 };

#[derive(Copy, Clone, Debug)]
pub struct Level {
    pub current: u16,
    pub second_avg: f32,
    pub minute_avg: f32,
    pub hour_avg: f32,
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Pulse(Pulse),
    Level(Level),
}

pub type MessageChannel = Channel<NoopRawMutex, Message, 16>;
pub type MessageSender<'a> = Sender<'a, NoopRawMutex, Message, 16>;
pub type MessageReceiver<'a> = Receiver<'a, NoopRawMutex, Message, 16>;

struct LvlAverages {
    s_samples: [u16;10],
    s_pos: usize,
    s_sum: i32,
    s_full: bool,

    m_samples: [f32;60],
    m_pos: usize,
    m_sum: f32,
    m_full: bool,

    h_samples: [f32;60],
    h_pos: usize,
    h_sum: f32,
    h_full: bool,
}

pub struct Radio<'d, > {
    rssi_adc: adc::Adc<'d, adc::Async>,
    rssi_adc_pin: adc::Pin<'d>,
    data_pio: pio::Pio<'d, PIO0>,
    msg_sender: MessageSender<'d>,

    lvl_averages: LvlAverages,
}

impl<'d> Radio<'d> {

    pub fn new(adc_per: impl Peripheral<P = peripherals::ADC> + 'd,
               adc_pin: impl Peripheral<P = impl AdcPin> + 'd,
               pio_per: impl Peripheral<P = peripherals::PIO0> + 'd,
               data_pin: impl pio::PioPin + 'd,
               msg_sender: MessageSender<'d>) -> Self {
        let rssi_adc = adc::Adc::new(adc_per, Irqs, adc::Config::default());
        let rssi_adc_pin = adc::Pin::new(adc_pin, gpio::Pull::None);

        let mut data_pio = pio::Pio::new(pio_per, Irqs);
        let in_pin = data_pio.common.make_pio_pin(data_pin);
        let prg = pio_proc::pio_file!("src/measure_pulses.pio", select_program("measure_pulses"));
        let mut cfg = pio::Config::default();
        cfg.use_program(&data_pio.common.load_program(&prg.program), &[]);
        cfg.set_jmp_pin(&in_pin);
        cfg.set_set_pins(&[&in_pin]);
        cfg.set_in_pins(&[&in_pin]);
        cfg.clock_divider = (U56F8!(125_000_000) / U56F8!(3000000)).to_fixed();
        cfg.shift_in.auto_fill = true;
        cfg.fifo_join = pio::FifoJoin::RxOnly;
        data_pio.sm0.set_config(&cfg);
        Radio {
            rssi_adc,
            rssi_adc_pin,
            data_pio,
            msg_sender,

            lvl_averages: LvlAverages {
                s_samples: [0;10],
                s_pos: 0,
                s_full: false,
                s_sum: 0,

                m_samples: [0.0;60],
                m_pos: 0,
                m_full: false,
                m_sum: 0.0,

                h_samples: [0.0;60],
                h_pos: 0,
                h_full: false,
                h_sum: 0.0,
            }
        }
    }
    async fn monitor_level<'a>(sender: MessageSender<'a>, avgs: &mut LvlAverages,
                               rssi_adc: &mut adc::Adc<'d, adc::Async>, adc_pin: &mut adc::Pin<'d>) {
        let mut last_second = Instant::MIN;
        let mut last_minute = Instant::MIN;
        let mut level;
        let mut s_avg: f32;
        let mut m_avg: f32 = 0.0;
        let mut h_avg: f32 = 0.0;

        loop {
            let now = Instant::now();
            level = rssi_adc.read(adc_pin).await.unwrap();

            avgs.s_sum += (level as i32) - (avgs.s_samples[avgs.s_pos] as i32);
            avgs.s_samples[avgs.s_pos] = level;
            if avgs.s_pos == 9 { avgs.s_full = true };
            avgs.s_pos = (avgs.s_pos + 1) % 10;

            s_avg = avgs.s_sum as f32 / if avgs.s_full { 10.0 } else { avgs.s_pos as f32};

            if (now - last_second).as_secs() >= 1 {
                debug!("RSSI: {} (1s avg: {})", level, s_avg);

                avgs.m_sum += s_avg - avgs.m_samples[avgs.m_pos];
                avgs.m_samples[avgs.m_pos] = s_avg;
                if avgs.m_pos == 59 { avgs.m_full = true };
                avgs.m_pos = (avgs.m_pos + 1) % 60;
                m_avg = avgs.m_sum / if avgs.m_full { 60.0 } else { avgs.m_pos as f32};
                last_second = now;

                if h_avg == 0.0 && m_avg > 0.0 { h_avg = m_avg };

                if (now - last_minute).as_secs() >= 60 {
                    avgs.h_sum += m_avg - avgs.h_samples[avgs.h_pos];
                    avgs.h_samples[avgs.h_pos] = m_avg;
                    if avgs.h_pos == 59 { avgs.h_full = true };
                    avgs.h_pos = (avgs.h_pos + 1) % 60;
                    h_avg = avgs.h_sum / if avgs.h_full { 60.0 } else { avgs.h_pos as f32};
                    last_minute = now;
                }
            }

            let msg = Level {
                current: level,
                second_avg: s_avg,
                minute_avg: m_avg,
                hour_avg: h_avg,
            };
            sender.send(Message::Level(msg)).await;

            Timer::at(now + Duration::from_millis(100)).await;
        }
    }

    async fn read_pulses<'a>(sender: MessageSender<'a>, data_pio: &mut pio::Pio<'d, PIO0>) {

        let mut last_report = Instant::MIN;
        let mut pio_resets = 0;
        let mut shorts = 0;
        let mut misses = 0;
        let mut overruns = 0;
        let mut pulses_total = 0;
        let mut pulses_forwarded = 0;
        let mut current_streak = 0;
        let mut longest_streak = 0;
        let mut last_sent;
        let mut pending_pulse = PULSE_RESET;
        let mut prev_kind = PulseKind::Low;
        let mut prev_value = 0;
        let mut need_reset: bool = false;

        sender.send(Message::Pulse(PULSE_RESET)).await;
        last_sent = PULSE_RESET;

        data_pio.sm0.set_enable(true);

        loop {
            let now = Instant::now();
            if (now - last_report).as_secs() >= 1 {
                debug!("Pulses received: {}, forwarded: {}. Resets: PIO: {}, shorts: {}, misses: {}, overruns: {}",
                      pulses_total, pulses_forwarded, pio_resets, shorts, misses, overruns);
                debug!("Current streak: {}, longest streak: {}", current_streak, longest_streak);
                last_report = now;
            }
            let mut value = data_pio.sm0.rx().wait_pull().await;
            loop {
                let mut pulse : Pulse;
                if value == 0 {
                    pulse = PULSE_RESET;
                    pio_resets += 1;
                    prev_kind = PulseKind::Low;
                }
                else if need_reset {
                    pulse = PULSE_RESET;
                }
                else {
                    if prev_kind == PulseKind::Low {
                        pulse = Pulse {
                            kind: PulseKind::High,
                            length: 0x80000000 - value,
                        };
                        prev_kind = PulseKind::High;
                    }
                    else if value <= prev_value {
                        // value for low pulse should always be lower than previous value...
                        pulse = Pulse {
                            kind: PulseKind::Low,
                            length: prev_value - value,
                        };
                        prev_kind = PulseKind::Low;
                    }
                    else {
                        // ...otherwise some edge must have been missed
                        info!("value = {} after {} when low pulse expected", value, prev_value); 
                        pulse = PULSE_RESET;
                        misses += 1;
                        prev_kind = PulseKind::Low;
                        unsafe {
                            // restart the program
                            data_pio.sm0.exec_instr(0);
                        }
                    }
                }
                prev_value = value;
                
                if pulse.kind == PulseKind::Low || pulse.kind == PulseKind::High {
                    pulses_total += 1;
                    if pulse.length >= MIN_PULSE_LENGTH {
                        current_streak += 1;
                        longest_streak = max(current_streak, longest_streak);
                    }
                    else {
                        shorts += 1;
                        current_streak = 0;
                        pulse = PULSE_RESET;
                    }
                    if current_streak == 2 {
                        // send pending pulse when we get second usable value
                        match sender.try_send(Message::Pulse(pending_pulse)) {
                            Ok(_) => {
                                last_sent = pulse;
                                pulses_forwarded += 1;
                            },
                            Err(_) => {
                                overruns += 1;
                                need_reset = true;
                            }
                        }
                    }
                    if current_streak > 1 {
                        match sender.try_send(Message::Pulse(pulse)) {
                            Ok(_) => {
                                last_sent = pulse;
                                pulses_forwarded += 1;
                            },
                            Err(_) => {
                                overruns += 1;
                                need_reset = true;
                            }
                        }
                    }
                    else {
                        // single pulse is useless when it is followed by unusable value, postpone
                        pending_pulse = pulse;
                    }
                }
                else if pulse.kind != PulseKind::Reset {
                    panic!("Unexpected pulse kind");
                }
                else if last_sent.kind != pulse.kind { // no point in sending stream of resets
                    match sender.try_send(Message::Pulse(pulse)) {
                        Ok(_) => {
                            last_sent = pulse;
                            need_reset = false;
                        },
                        Err(_) => {
                            overruns += 1;
                            need_reset = true;
                        }
                    }
                }

                match data_pio.sm0.rx().try_pull() {
                    None => break,
                    Some(x) => value = x,
                }
            }
        }
    }

    pub async fn run(&mut self) {

        let level_fut = Self::monitor_level(self.msg_sender.clone(), &mut self.lvl_averages,
                                           &mut self.rssi_adc, &mut self.rssi_adc_pin);
        let pulses_fut = Self::read_pulses(self.msg_sender.clone(),
                                            &mut self.data_pio);

        join(level_fut, pulses_fut).await;
    }
}
