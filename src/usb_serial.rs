
use embassy_futures::join::join3;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_usb::driver::EndpointError;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State, Sender, Receiver};
use embassy_usb::{Builder, Config, UsbDevice};
use embassy_rp::Peripheral;
use embassy_sync::pipe::{Pipe, TryWriteError};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use defmt::{info, debug};
use core::fmt;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

pub type UsbSerialPipe = Pipe<CriticalSectionRawMutex, 256>;
pub type UsbSerialPipeReader = embassy_sync::pipe::Reader<'static, CriticalSectionRawMutex, 256>;
pub type UsbSerialPipeWriter = embassy_sync::pipe::Writer<'static, CriticalSectionRawMutex, 256>;

pub struct UsbSerial<'d> {
    usb: UsbDevice<'d, Driver<'d, USB>>,
    class_sender: Sender<'d, Driver<'d, USB>>,
    class_receiver: Receiver<'d, Driver<'d, USB>>,
    pipe_reader: UsbSerialPipeReader,
}

static mut DEVICE_DESCRIPTOR: [u8;256] = [0; 256];
static mut CONFIG_DESCRIPTOR: [u8;256] = [0; 256];
static mut BOS_DESCRIPTOR: [u8;256] = [0; 256];
static mut CONTROL_BUF: [u8;64] = [0; 64];
static mut STATE : Option<State> = None;


pub const WRITE_BUFFER_SIZE: usize = 256;


#[derive(Clone)]
pub struct UsbSerialWriter {
    pipe_writer: UsbSerialPipeWriter,
    buf: [u8; WRITE_BUFFER_SIZE],
    written: usize,
}

impl UsbSerialWriter {
    pub fn new(pipe_writer: UsbSerialPipeWriter) -> UsbSerialWriter {
        UsbSerialWriter {
            pipe_writer,
            buf: [0; WRITE_BUFFER_SIZE],
            written: 0,
        }
    }
    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), ()> {
        self.pipe_writer.write(bytes).await;
        Ok(())
    }
    pub async fn send_written(&mut self) -> Result<(), ()> {
        let mut num_bytes: usize;
        let mut start: &[u8] = &self.buf[..];
        (num_bytes, self.written) = (self.written, 0);

        while num_bytes > 0 {
            match self.pipe_writer.try_write(&start[..num_bytes]) {
                Ok(n) => {
                    start = &start[n..num_bytes];
                    num_bytes -= n;
                }
                Err(TryWriteError::Full) => {
                    debug!("USB pipe full!");
                    return Err(());
                },
            }
        }
        Ok(())
    }
}

impl<'a> fmt::Write for UsbSerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        if bytes.len() + self.written > WRITE_BUFFER_SIZE {
            panic!("Usb serial formatting buffer overflow");
        }

        self.buf[self.written..(self.written + bytes.len())].clone_from_slice(bytes);
        self.written += bytes.len();

        Ok(())
    }
}

impl<'d> UsbSerial<'d> {
    pub fn new(usb_p: impl Peripheral<P = USB> + 'd + 'static, pipe_reader: UsbSerialPipeReader) -> UsbSerial<'static> {

        debug!("creating USB driver");
        let driver = Driver::new(usb_p, Irqs);

        debug!("creating USB config");

        let mut config = Config::new(0x1209, 0x001);
        config.manufacturer = Some("Jajcus");
        config.product = Some("rpp_433mhz_weather_receiver");
        config.serial_number = Some("12345678");
        config.max_power = 100;
        config.max_packet_size_0 = 64;

        // Required for windows compatibility.
        // https://developer.nordicsemi.com/nRF_Connect_SDK/doc/1.9.1/kconfig/CONFIG_CDC_ACM_IAD.html#help
        config.device_class = 0xEF;
        config.device_sub_class = 0x02;
        config.device_protocol = 0x01;
        config.composite_with_iads = true;

        let device_descriptor;
        let config_descriptor;
        let bos_descriptor;
        let control_buf;
        unsafe {
            // mutable refereces to static buffers – ok, as used only once only here
            device_descriptor = &mut DEVICE_DESCRIPTOR;
            config_descriptor = &mut CONFIG_DESCRIPTOR;
            bos_descriptor = &mut BOS_DESCRIPTOR;
            control_buf = &mut CONTROL_BUF;
        }

        let mut builder = Builder::new(
            driver,
            config,
            device_descriptor,
            config_descriptor,
            bos_descriptor,
            control_buf
        );

        // Create classes on the builder.
        let state_r;
        unsafe {
            STATE = Some(State::new());
            state_r = STATE.as_mut().unwrap();
        }


        debug!("creating USB class");
        let class = CdcAcmClass::new(&mut builder, state_r, 64);
        let (class_sender, class_receiver) = class.split();

        debug!("building USB");

        // Build the builder.
        let usb = builder.build();

        info!("USB built");
        UsbSerial {
            usb,
            class_sender,
            class_receiver,
            pipe_reader,
            }
    }
    pub async fn run(&mut self) {
        debug!("UsbSerial.run()");
        let usb_fut = self.usb.run();

        let read_fut = async {
            debug!(" read_fut()");
            let mut buf: [u8;64] = [0;64];
            loop {
                self.class_receiver.wait_connection().await;
                debug!("Connected (recv)");
                loop {
                    match self.class_receiver.read_packet(&mut buf).await {
                        Ok(n) => debug!("{} bytes received", n),
                        Err(EndpointError::Disabled) => break,
                        Err(EndpointError::BufferOverflow) => panic!("Buffer overflow"),
                    }
                }
                debug!("Disconnected (recv)");
            }
        };

        let write_fut = async {
            debug!(" write_fut()");
            let mut buf: [u8;64] = [0;64];
            loop {
                self.class_sender.wait_connection().await;
                debug!("Connected (send)");
                loop {
                    let number = self.pipe_reader.read(&mut buf).await;
                    debug!("Got {} bytes", number);
                    match self.class_sender.write_packet(&buf[0..number]).await {
                        Ok(()) => debug!("sent!"),
                        Err(EndpointError::Disabled) => break,
                        Err(EndpointError::BufferOverflow) => panic!("Buffer overflow"),
                    }
                }
                debug!("Disconnected (send)");
            }
        };
        join3(usb_fut, read_fut, write_fut).await;
        info!("UsbSerial.run() end");
    }
}
