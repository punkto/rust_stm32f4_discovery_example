#![no_std]
#![no_main]

use panic_halt as _;
extern crate cortex_m_semihosting as sh;
use crate::hal::{
    dwt::{ClockDuration, DwtExt},
    pac,
    prelude::*,
};
use stm32f4xx_hal as hal;

use crate::sh::hio;
use cortex_m_rt::entry;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();

    writeln!(hstdout, "Hello, world!").unwrap();

    let mut dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut gpiod = dp.GPIOD.split();

    let mut leda = gpiod
        .pd14
        .into_push_pull_output();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, &clocks);
    let mut delay = dwt.delay();

    loop {
        // led.set_low();
        writeln!(hstdout, "LED OFF").unwrap();
        leda.set_low();
        delay.delay_ms(1000_u32);
        writeln!(hstdout, "LED ON").unwrap();
        leda.set_high();
        delay.delay_ms(1000_u32);
    }
}
