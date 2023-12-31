# Raspberry Pi Pico based receiver for 433 MHz weather sensors

## Overview

This repository contains code for a simple Raspberry-Pi based 433 Mhz receiver for weather data from remote sensors of cheap 'weather stations' intended to provide data for Home Assistant via serial over USB.

## State of the project

This is one-off project for my own needs, which I would like to share for anyone interested. It is one of my first Rust projects, so the code is probably quite bad.

The code uses a great, but highly experimental [Embassy](https://github.com/embassy-rs/embassy) framework, which, in turn, depends on nightly features of the Rust compiler. It may stop working with future official Rust releases.

Use on your own risk.

The code currently support only one specific communication protocol (called 'Prologue' in [rtl_433 code](https://github.com/merbanan/rtl_433/blob/master/src/devices/prologue.c)) implemented in a few different devices. My sensors are for 'Meteo SP90' weather station.

It should be easy to modify the code to support most (if not any) devices supported by [rtl_422](https://github.com/merbanan/rtl_433/blob/master/src/devices/prologue.c).

Also, the cheap receiver hardware used is not very good, so reception range is quite bad.

## Hardware

![hardware - inside view](images/hw_inside.jpg) ![hardware - outside view](images/hw_outside.jpg)

My device consists of [Raspberry Pi Pico](https://www.raspberrypi.com/products/raspberry-pi-pico/) and a cheap 433Mhz receiver module 'RXB6'. No other components, except the antenna, are really required, but I have also added three LEDs with current limiting resistors and a decoupling capacitor (I am not sure it is needed, but it shouldn't hurt).

'+5V' pin of RXB6 should be connected to +3.3V from Raspberry Pi Pico (**it must not be connected to +5V**, despite the label, or the Pi might get damaged).
GND pins of Pi and RXB6 should be connected, of course. And the 'DATA' pin of RXB6 should be connected to any GPIO pin of Pi (code assumes GPIO #19 – pin 25).

For RSSI (signal level) monitoring the DER pin (available on newr RXB6 versions) is used. For this to work the RXB6 modules needs to be slightly modified – the 'R6' 0Ω resistor must be removed and moved to the 'R7' place (a wire or solder bridge may be used instead). Then the DER pin should be connected to and ADC pin of the Pi (code assumes ADC #0 – GPIO #26 – pin 31).

The antenna is a straight wire 173mm long. Actually a slightly shorter one works even better for me (part of the circuit works as effective antenna length).

The three LEDs are:
* 'power' (red) – always on when the device is on and code running – GPIO #2 (pin 4)
* 'signal' (yellow) – lights up when RSSI becomes larger than last minute average – GPIO #6 (pin 9)
* 'data' (green) – lights up when signal coding is recognized and data packet decoded – GPIO #10 (pin 14)

All LEDS connected to ground with current-limiting resistors of 1kΩ.

![schematic](images/schematic.svg)

## How it works

RXB6 converts 433Mhz radio signal to stream of 'ones' and 'zeros'. A PIO program measures those pulses and returns number of microseconds for each low and high state. Some obvious noise and errors are filtered out in the PIO-interfacing code and remaining pulse data is passed to a decoder.

A decoder based on comments from [rtl_433 code](https://github.com/merbanan/rtl_433/blob/master/src/devices/prologue.c) finds pules that make sense and extracts temperature and humidity data from received packets (provided three identical ones are received in a row).

The decoded data is then sent as JSON strings via USB using the CDC ACM class.

## Building and running

### Building

Check out the codei and submodules:

```bash
git clone https://github.com/Jajcus/rpp_433mhz_weather_receiver.git
cd rpp_433mhz_weather_receiver
git submodule init
git submodule update --recursive
```

Install Rust 'nightly' toolkit and the 'thumbv6m-none-eabi' target, make
nightly the default for this projectk:

```bash
rustup install nightly
rustup target add --toolchain nightly thumbv6m-none-eabi
rustup override set nightly
```

It should be possible to compile the project now:
```bash
cargo build --release
```

Note: use ``--release``, as a debug build might be too slow to keep up with some noise pulses which makes it unreliable.

### Flashing and running

You will need a [probe-rs](https://probe.rs/) compatible probe to program the Raspberry Pi Pico. The simplest way to get one is to use another Raspberry Pi Pico with [Picoprobe](https://github.com/raspberrypi/picoprobe) firmware with CMSIS support (recent versions).

Make sure you have ``probe-run`` installed (``probe-rs-cli`` will also work, but ``.cargo/config.toml`` needs to be updated.):

```bash
cargo install probe-run
```

Connect the probe to the target device (SWD connector is enough – +5V can be provided via USB and serial console is not used) and run:

```bash
cargo run --release
```

The device should be flashed, restarted and debug output will be displayed.
If the device fails to start after subsequent flashing it might be because of https://github.com/embassy-rs/embassy/issues/1736 Power cycling should help (eventually).

## Receiving data

The device sends received data as JSON lines over serial over USB. In Linux they can be read from a /dev/ttyACM0 (or whatever the kernel assigns it), like this:

```bash
cat /dev/ttyACM0

```

Which will result is output like this:
```json
{"channel": 2, "id": 48, "temperature": 22.6, "humidity": 54, "rssi": 1024}
{"channel": 1, "id": 155, "temperature": 22.3, "humidity": 54, "rssi": 1081}
{"channel": 2, "id": 48, "temperature": 22.6, "humidity": 54, "rssi": 1021}
{"channel": 1, "id": 155, "temperature": 22.3, "humidity": 54, "rssi": 1079}
{"channel": 2, "id": 48, "temperature": 22.6, "humidity": 54, "rssi": 1026}
{"channel": 1, "id": 155, "temperature": 22.3, "humidity": 54, "rssi": 1083}
{"channel": 2, "id": 48, "temperature": 22.6, "humidity": 54, "rssi": 1025}
```

## Feeding data to MQTT for Home Assistant

Home Assistant cannot use that output directly (as far as I know) and easiest
way to make it usable to Home Assistant is to publish it to MQTT.

There are various solutions for converting serial transmition into MQTT. I have
chosen [2mqtt](https://github.com/mycontroller-org/2mqtt) which can be easily
configured to handle such JSON strings.

See the configuration example in the ``etc/`` directory –
``etc/2mqtt/config.yaml`` configuration file for 2mqtt and
``etc/systemd/rpp_433mhz_weather2mqtt.service`` systemd unit file to start the
2qmtt container bound to the serial device.
