//! This example shows how to use USB (Universal Serial Bus) in the RP2040 chip.
//!
//! This creates a USB serial port that echos.

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_projections)]

pub mod usb_serial;
pub mod radio;
pub mod test_data;
pub mod decoder;

use defmt::{info,debug,unwrap};

use crate::usb_serial::{UsbSerial, UsbSerialPipe, UsbSerialWriter};
use crate::radio::Radio;
use crate::decoder::{DecoderInputChannel, DecoderOutputChannel, run_decoder};

use static_cell::StaticCell;
use embassy_executor::Executor;
use embassy_futures::join::join3;
use embassy_sync::pubsub::WaitResult;
use embassy_time::{Duration, Timer};
use embassy_rp::peripherals;

use embassy_rp::gpio::{Level, Output, AnyPin};
use embassy_rp::multicore::{spawn_core1, Stack};

use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: Stack<81920> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

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

    //cortex_m::asm::delay(125_000_000 * 5); // ~5 second
    cortex_m::asm::delay(10);

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
    let data_pin =  periferials.PIN_19;

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

    let core0pers = Core0Peripherals {
        usb: usb_per,
    };

    static USB_PIPE: UsbSerialPipe = UsbSerialPipe::new();
    let usb_writer = UsbSerialWriter::new(&USB_PIPE);

    spawn_core1(periferials.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let executor1 = EXECUTOR1.init(Executor::new());
        executor1.run(|spawner| unwrap!(spawner.spawn(core1_task(core1pers, usb_writer))));
    });

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| unwrap!(spawner.spawn(core0_task(core0pers, &USB_PIPE))));
}



#[embassy_executor::task]
async fn core1_task(pers: Core1Peripherals, mut usb_writer: UsbSerialWriter<'static>) {
    
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

    let level_channel = radio::LevelChannel::new();
    let mut l_sub = level_channel.subscriber().unwrap();
    let pulse_channel = radio::PulseChannel::new();
    let mut p_sub = pulse_channel.subscriber().unwrap();

    let decoder_in_channel = DecoderInputChannel::new();
    let decoder_tx = decoder_in_channel.sender();
    let decoder_out_channel = DecoderOutputChannel::new();
    let decoder_rx = decoder_out_channel.receiver();

    let mut radio = Radio::new(pers.adc, pers.adc_pin, pers.pio, pers.data_pin,
                               level_channel.publisher().unwrap(), pulse_channel.publisher().unwrap());


    info!("Got it!");

    let decoder_fut = run_decoder(decoder_in_channel.receiver(), decoder_out_channel.sender());

    let my_fut = async {
        info!("Writting 'abc'");
        let _ = usb_writer.write(b"abc").await;
        info!("Writting 'def'");
        let _ = usb_writer.write(b"def").await;

        use core::fmt::Write;
        _ = write!(usb_writer, "btest");

        _ = usb_writer.send_written().await;

        //use crate::test_data::*;
        //for pulse in PROLOGUE_TEST_STREAM {
        //    decoder_tx.send(pulse).await;
        //}

        use radio::PulseKind;

        loop {
            match decoder_rx.try_recv() {
                Err(_) => (),
                Ok(x) => {
                    info!("Ch: {} Id: {} Temperature: {} Humidity: {}",
                          x.channel, x.id, x.temperature, x.humidity);
                    _ = write!(usb_writer, "Ch: {} Id: {} Temperature: {} Humidity: {}\r\n",
                                x.channel, x.id, x.temperature, x.humidity);
                    _ = usb_writer.send_written().await;
                }
            }
            match l_sub.try_next_message() {
                None => (),
                Some(WaitResult::Lagged(x)) => { info!("Missed {} signal level messages", x); },
                Some(WaitResult::Message(x)) => { info!("Signal level: {}", x); },
            }
            match p_sub.next_message().await {
                WaitResult::Lagged(x) => { debug!("Missed {} pulse messages", x); },
                WaitResult::Message(pulse) => { 
                    match pulse.kind {
                        PulseKind::Reset => { debug!("Pulse stream reset!"); },
                        PulseKind::Low => { debug!("LOW for {} us", pulse.length); },
                        PulseKind::High => { debug!("HIGH for {} us", pulse.length); },
                    }
                    decoder_tx.send(pulse).await;
                },
            }
        }
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join3(radio.run(), decoder_fut, my_fut).await;

    loop {};
}

#[embassy_executor::task]
async fn core0_task(pers: Core0Peripherals, usb_pipe: &'static UsbSerialPipe) -> ! {

    info!("Core 0 running");
    
    let mut usb_serial = UsbSerial::new(pers.usb, usb_pipe);
    
    info!("usb_serial created, running it");

    usb_serial.run().await;
    loop {};
}
