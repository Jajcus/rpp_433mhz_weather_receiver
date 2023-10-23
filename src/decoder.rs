

use embassy_sync::channel::{Channel, Sender, Receiver};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use crate::radio::PulseKind;

use crate::radio::Pulse;

use defmt::debug;

pub type DecoderInputChannel = Channel<NoopRawMutex, Pulse, 10>;
pub type DecoderInputTx<'a> = Sender<'a, NoopRawMutex, Pulse, 10>;
pub type DecoderInputRx<'a> = Receiver<'a, NoopRawMutex, Pulse, 10>;

pub type DecoderOutputChannel = Channel<NoopRawMutex, DecodedData, 10>;
pub type DecoderOutputTx<'a> = Sender<'a, NoopRawMutex, DecodedData, 10>;
pub type DecoderOutputRx<'a> = Receiver<'a, NoopRawMutex, DecodedData, 10>;

#[derive(Copy, Clone, Debug)]
pub struct DecodedData {
    pub temperature: f32,
    pub humidity: f32,
    pub id: u32,
    pub channel: u32,
}

async fn decode_bit<'a>(input: &'a DecoderInputRx<'a>) -> Result<bool, ()> {

    let pulse1 = input.receive().await;
    if pulse1.kind != PulseKind::High || pulse1.length < 500 || pulse1.length > 600 { return Err(()); }

    let pulse2 = input.receive().await;

    if pulse2.kind != PulseKind::Low { return Err(()); }

    if pulse2.length < 2000 { Err(()) }
    else if pulse2.length < 2500 { Ok(false) }
    else if pulse2.length < 4000 { Err(()) }
    else if pulse2.length < 4500 { Ok(true) }
    else { Err(()) }
}

async fn decode_bits<'a, const S: usize, const N: usize>(input: &'a DecoderInputRx<'a>) -> Result<[u8;S], ()>  {

    if S < N / 8 { panic!("S < N / 8") }

    let mut result: [u8;S] = [0;S];

    for i in 0..N {
        let bit = decode_bit(input).await?;
        result[i / 8] |= (bit as u8) << (7 - (i % 8));
    }

    Ok(result)
}

pub async fn run_decoder<'a>(input: DecoderInputRx<'a>, output: DecoderOutputTx<'a>) {

    let mut got_packets = 0;
    let mut last_packet: [u8;5] = [0;5];
    'main : loop {
        if got_packets > 0 {
            last_packet = [0;5];
            got_packets = 0;
        }
        loop {
            let pulse = input.receive().await;

            // wait for the 9000ms low pulse
            if pulse.kind != PulseKind::Low || pulse.length < 9000 || pulse.length > 9500 {
                continue;
            }

            let packet: [u8;5];

            match decode_bits::<5, 36>(&input).await {
                Err(_) => { debug!("decode bits error"); continue 'main; },
                Ok(p) => packet = p,
            }

            let type_v = packet[0] >> 4;
            if type_v != 9 && type_v != 5 {
                debug!("Unexpected type: {}", type_v);
                continue 'main;
            }

            if packet == last_packet {
                got_packets += 1;
            }
            else {
                got_packets = 1;
            }

            last_packet = packet;

            if got_packets == 3 {
                // 3x the same packet – got good data

                let id = (((packet[0] &0x0f) as u32) << 4) | ((packet[1] as u32) >> 4);
                let channel = ((packet[1] as u32) & 0x03) + 1;
                let temperature = (((packet[2] as u32) << 4) | ((packet[3] as u32) >> 4)) as f32 / 10.0;
                let humidity = ((((packet[3] & 0x0f) as u32) << 4) | ((packet[4] as u32) >> 4)) as f32;
                output.send(DecodedData{temperature, humidity, id, channel}).await;
            }
        }
    }
}

