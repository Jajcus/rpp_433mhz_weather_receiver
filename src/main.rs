#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

pub mod decoder;
pub mod radio;
pub mod test_data;
pub mod usb_serial;

use defmt::{debug, info, unwrap};

use crate::decoder::{run_decoder, DecoderInputChannel, DecoderOutputChannel};
use crate::radio::Radio;
use crate::usb_serial::{UsbSerial, UsbSerialPipe, UsbSerialPipeReader, UsbSerialWriter};

use embassy_executor::Executor;
use embassy_futures::join::join3;
use embassy_rp::peripherals;
use embassy_time::{Duration, Instant, Timer};
use static_cell::StaticCell;

use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_rp::multicore::{spawn_core1, Stack};

use core::fmt::Write;

use {defmt_rtt as _, panic_probe as _};

const NEW_SENSOR_DELAY: Duration = Duration::from_secs(15 * 60);

static mut CORE1_STACK: Stack<81920> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static USB_PIPE: StaticCell<UsbSerialPipe> = StaticCell::new();

struct Core1Peripherals {
    pico_led: AnyPin,
    power_led: AnyPin,
    signal_led: AnyPin,
    data_led: AnyPin,
    adc: peripherals::ADC,
    adc_pin: peripherals::PIN_26,
    pio: peripherals::PIO0,
    data_pin: peripherals::PIN_19,
}

struct Core0Peripherals {
    usb: peripherals::USB,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("starting");
    let periferials = embassy_rp::init(Default::default());

    info!("started");

    let pico_led: AnyPin = periferials.PIN_25.into();
    let power_led: AnyPin = periferials.PIN_2.into();
    let signal_led: AnyPin = periferials.PIN_6.into();
    let data_led: AnyPin = periferials.PIN_10.into();
    let adc = periferials.ADC;
    let adc_pin = periferials.PIN_26;
    let pio = periferials.PIO0;
    let data_pin = periferials.PIN_19;

    let core1pers = Core1Peripherals {
        pico_led,
        power_led,
        signal_led,
        data_led,
        adc,
        adc_pin,
        pio,
        data_pin,
    };

    let usb_per = periferials.USB;

    let core0pers = Core0Peripherals { usb: usb_per };

    let pipe = USB_PIPE.init(UsbSerialPipe::new());
    let (reader, writer) = pipe.split();
    let usb_writer = UsbSerialWriter::new(writer);

    spawn_core1(periferials.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| unwrap!(spawner.spawn(core1_task(core1pers, usb_writer))));
    });

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| unwrap!(spawner.spawn(core0_task(core0pers, reader))));
}

#[derive(Copy, Clone, Debug)]
struct SensorInfo {
    id: u32,
    last_heard: Instant,
}

#[embassy_executor::task]
async fn core1_task(pers: Core1Peripherals, mut usb_writer: UsbSerialWriter) {
    info!("Core 1 running");

    let mut pico_led = Output::new(pers.pico_led, Level::Low);
    let mut power_led = Output::new(pers.power_led, Level::Low);
    let mut signal_led = Output::new(pers.signal_led, Level::Low);
    let mut data_led = Output::new(pers.data_led, Level::Low);

    // light it to show the code is running
    pico_led.set_high();

    // flash all LEDs, so we know they are working and startup looks nice
    Timer::after(Duration::from_millis(200)).await;
    power_led.set_high();
    Timer::after(Duration::from_millis(200)).await;
    signal_led.set_high();
    Timer::after(Duration::from_millis(200)).await;
    data_led.set_high();
    Timer::after(Duration::from_millis(500)).await;
    signal_led.set_low();
    data_led.set_low();

    let mut signal_led_on = false;
    let mut data_led_off_after = Instant::MAX;

    let msg_channel = radio::MessageChannel::new();
    let msg_rx = msg_channel.receiver();

    let decoder_in_channel = DecoderInputChannel::new();
    let decoder_tx = decoder_in_channel.sender();
    let decoder_out_channel = DecoderOutputChannel::new();
    let decoder_rx = decoder_out_channel.receiver();

    let mut radio = Radio::new(
        pers.adc,
        pers.adc_pin,
        pers.pio,
        pers.data_pin,
        msg_channel.sender(),
    );

    info!("Got it!");

    let decoder_fut = run_decoder(decoder_in_channel.receiver(), decoder_out_channel.sender());

    let my_fut = async {
        use radio::PulseKind;

        let mut known_sensors: [SensorInfo; 3] = [SensorInfo {
            id: 0,
            last_heard: Instant::MIN,
        }; 3];
        let mut rssi: u16 = 0;

        loop {
            let now = Instant::now();
            match decoder_rx.try_receive() {
                Err(_) => (),
                Ok(x) => {
                    data_led.set_high();
                    data_led_off_after = now + Duration::from_millis(250);

                    if x.channel > 0 && x.channel < 4 {
                        let k = &mut known_sensors[(x.channel - 1) as usize];
                        if k.last_heard == Instant::MIN {
                            info!("New sensor detected!");
                        } else if k.id != x.id {
                            if now - k.last_heard > NEW_SENSOR_DELAY {
                                info!("Switching to a new sensor, long time no hear from the last one.");
                            } else {
                                info!("Unknown sensor (id: {}) heard for channel {}. Ignoring it (for now).", x.id, x.channel);
                                continue;
                            }
                        }
                        k.last_heard = now;
                        k.id = x.id;
                    }

                    info!(
                        "Ch: {} Id: {} Temperature: {} Humidity: {} RSSI: {}",
                        x.channel, x.id, x.temperature, x.humidity, rssi
                    );

                    _ = writeln!(usb_writer, "{{\"channel\": {}, \"id\": {}, \"temperature\": {}, \"humidity\": {}, \"rssi\": {}}}",
                                x.channel, x.id, x.temperature, x.humidity, rssi);
                    _ = usb_writer.send_written().await;
                }
            }
            if now > data_led_off_after {
                data_led.set_low();
                data_led_off_after = Instant::MAX;
            }
            match msg_rx.receive().await {
                radio::Message::Level(l) => {
                    debug!(
                        "Level: current: {}  avg second: {} minute: {} hour: {}",
                        l.current, l.second_avg, l.minute_avg, l.hour_avg
                    );
                    rssi = l.current;
                    if (l.current as f32) > 1.1 * l.minute_avg {
                        if !signal_led_on {
                            signal_led.set_high();
                            signal_led_on = true;
                        }
                    } else if signal_led_on {
                        signal_led.set_low();
                        signal_led_on = false;
                    }
                }
                radio::Message::Pulse(pulse) => {
                    match pulse.kind {
                        PulseKind::Reset => {
                            debug!("Pulse stream reset!");
                        }
                        PulseKind::Low => {
                            debug!("LOW for {} us", pulse.length);
                        }
                        PulseKind::High => {
                            debug!("HIGH for {} us", pulse.length);
                        }
                    }
                    decoder_tx.send(pulse).await;
                }
            }
        }
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join3(radio.run(), decoder_fut, my_fut).await;

    panic!("Unreachable code reached (core 1)!");
}

#[embassy_executor::task]
async fn core0_task(pers: Core0Peripherals, usb_pipe_reader: UsbSerialPipeReader) -> ! {
    info!("Core 0 running");

    let mut usb_serial = UsbSerial::new(pers.usb, usb_pipe_reader);

    info!("usb_serial created, running it");

    usb_serial.run().await;

    panic!("Unreachable code reached (core 0)!");
}
