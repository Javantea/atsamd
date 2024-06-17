//! ADXL343 accelerometer example

#![no_std]
#![no_main]

use bsp::hal;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use trellis_m4 as bsp;
use ws2812_timer_delay as ws2812;

use hal::ehal::digital::v1_compat::OldOutputPin;

use accelerometer::Accelerometer;
use bsp::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::TimerCounter;
use hal::{clock::GenericClockController, delay::Delay};
use bsp::i2c_master;
use adxl343::Adxl343;
use hal::time::MegaHertz;

use smart_leds::{hsv::RGB8, SmartLedsWrite};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let gclk0 = clocks.gclk0();
    let tc2_3 = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&tc2_3, peripherals.TC3, &mut peripherals.MCLK);
    timer.start(MegaHertz(4));

    let mut delay = Delay::new(core_peripherals.SYST, &mut clocks);
    let mut pins = bsp::Pins::new(peripherals.PORT);

    // neopixels
    let neopixel_pin = pins.neopixel.into_push_pull_output();
    let mut neopixels = ws2812::Ws2812::new(timer, neopixel_pin);

    // accelerometer
    let mut adxl343 = Adxl343::new(i2c_master(&mut clocks, 100.khz(), peripherals.SERCOM2, &mut peripherals.MCLK, pins.sda, pins.scl)).unwrap();

    loop {
        let ax3 = adxl343.accel_norm().unwrap();
        //let vv:Accelerometer = adxl343;
        //let ax3 = vv.accel_norm().unwrap();

        // RGB indicators of current accelerometer state
        let colors = [
            RGB8::from(((ax3.x * 13.) as u8, 0, 0)),
            RGB8::from((0, (ax3.y * 13.) as u8, 0)),
            RGB8::from((0, 0, (ax3.x * 13.) as u8)),
        ];

        neopixels.write(colors.iter().cloned()).unwrap();

        delay.delay_ms(10u8);
    }
}
