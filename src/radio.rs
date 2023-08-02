
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
use embassy_sync::pubsub::{PubSubChannel, Publisher, Subscriber};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_futures::join::join;
use fixed::traits::ToFixed;
use fixed_macro::types::U56F8;
use core::cmp::max;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => adc::InterruptHandler;
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

const MAX_PULSES : usize = 256;
const MIN_PULSE_LENGTH : u32 = 50;

#[derive(Copy, Clone,Debug, PartialEq, Eq)]
pub enum PulseKind {
    Reset,
    Low,
    High
}

pub type LevelChannel = PubSubChannel<NoopRawMutex, u16, 10, 2, 1>;
pub type LevelPublisher<'a> = Publisher<'a, NoopRawMutex, u16, 10, 2, 1>;
pub type LevelSubscriber<'a> = Subscriber<'a, NoopRawMutex, u16, 10, 2, 1>;


#[derive(Copy, Clone, Debug)]
pub struct Pulse {
    pub kind: PulseKind,
    pub length: u32,
}

const PULSE_RESET : Pulse = Pulse { kind: PulseKind::Reset, length: 0 };

pub type PulseChannel = PubSubChannel::<NoopRawMutex, Pulse, MAX_PULSES, 2, 1>;
pub type PulsePublisher<'a> = Publisher::<'a, NoopRawMutex, Pulse, MAX_PULSES, 2, 1>;
pub type PulseSubscriber<'a> = Subscriber::<'a, NoopRawMutex, Pulse, MAX_PULSES, 2, 1>;

pub struct Radio<'d, > {
    rssi_adc: adc::Adc<'d, adc::Async>,
    rssi_adc_pin: adc::Pin<'d>,
    data_pio: pio::Pio<'d, PIO0>,
    level_pub: LevelPublisher<'d>,
    pulse_pub: PulsePublisher<'d>,
}

impl<'d> Radio<'d> {

    pub fn new(adc_per: impl Peripheral<P = peripherals::ADC> + 'd,
               adc_pin: impl Peripheral<P = impl AdcPin> + 'd,
               pio_per: impl Peripheral<P = peripherals::PIO0> + 'd,
               data_pin: impl pio::PioPin + 'd,
               level_pub: LevelPublisher<'d>,
               pulse_pub: PulsePublisher<'d>) -> Self {
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
            level_pub,
            pulse_pub,
        }
    }
    async fn monitor_level<'a>(l_pub: &mut LevelPublisher<'a>, rssi_adc: &mut adc::Adc<'d, adc::Async>, adc_pin: &mut adc::Pin<'d>) {
        let mut last_report = Instant::MIN;
        let mut level;

        loop {
            let now = Instant::now();
            level = rssi_adc.read(adc_pin).await.unwrap();
            if (now - last_report).as_secs() >= 1 {
                debug!("RSSI: {}", level);
                last_report = now;
            }
            l_pub.publish_immediate(level);

            Timer::at(now + Duration::from_millis(100)).await;
        }
    }
    async fn read_pulses<'a>(p_pub: &mut PulsePublisher<'a>, data_pio: &mut pio::Pio<'d, PIO0>) {

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

        p_pub.publish(PULSE_RESET).await;
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
                        match p_pub.try_publish(pending_pulse) {
                            Ok(_) => {
                                last_sent = pulse;
                                pulses_forwarded += 1;
                            },
                            Err(_) => {
                                overruns += 1;
                                p_pub.publish_immediate(PULSE_RESET);
                                last_sent = PULSE_RESET;
                            }
                        }
                    }
                    if current_streak > 1 {
                        match p_pub.try_publish(pulse) {
                            Ok(_) => {
                                last_sent = pulse;
                                pulses_forwarded += 1;
                            },
                            Err(_) => {
                                overruns += 1;
                                p_pub.publish_immediate(PULSE_RESET);
                                last_sent = PULSE_RESET;
                            }
                        }
                    }
                    else {
                        // single pulse is useless when it is followed by unusable value, postpone
                        pending_pulse = pulse;
                    }
                }
                else if last_sent.kind != pulse.kind { // no point in sending stream of resets
                    match p_pub.try_publish(pulse) {
                        Ok(_) => {
                            last_sent = pulse;
                        },
                        Err(_) => {
                            overruns += 1;
                            p_pub.publish_immediate(PULSE_RESET);
                            last_sent = PULSE_RESET;
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

        let level_fut = Self::monitor_level(&mut self.level_pub,
                                           &mut self.rssi_adc, &mut self.rssi_adc_pin);
        let pulses_fut = Self::read_pulses(&mut self.pulse_pub,
                                            &mut self.data_pio);


        join(level_fut, pulses_fut).await;
    }
}
