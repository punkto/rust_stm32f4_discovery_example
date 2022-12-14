#![no_std]
#![no_main]

use bme280::Measurements;
use panic_halt as _;
extern crate cortex_m_semihosting as sh;
use crate::hal::{
    dwt::DwtExt,
    pac,
    prelude::*,
};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::adc::Adc;
use stm32f4xx_hal::adc::config::AdcConfig;
use stm32f4xx_hal::adc::config::SampleTime;

use stm32f4xx_hal::i2c::Mode;

use crate::sh::hio;
use cortex_m_rt::entry;
use core::fmt::Write;

use bme280::BME280;

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();

    writeln!(hstdout, "Hello, world!").unwrap();

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpiod = dp.GPIOD.split();

    let mut leda = gpiod
        .pd14
        .into_push_pull_output();

    let an_in = gpioa.pa0.into_analog();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, &clocks);
    let mut delay = dwt.delay();

    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());


    let scl = gpiob.pb6;
    let sda = gpiob.pb9;
    let i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );

    let mut bme280 = BME280::new_primary(i2c, delay);
    // bme280.init().unwrap();
    match bme280.init() {
        Err(e) => writeln!(hstdout, "ERROR {:?}", e).unwrap(),
        Ok(()) => writeln!(hstdout, "BME OK").unwrap(),
    }

    loop {
        // led.set_low();
        writeln!(hstdout, "LED OFF").unwrap();
        leda.set_low();
        delay.delay_ms(1000_u32);
        writeln!(hstdout, "LED ON").unwrap();
        leda.set_high();
        delay.delay_ms(1000_u32);

        let sample = adc.convert(&an_in, SampleTime::Cycles_480);
        let millivolts = adc.sample_to_millivolts(sample);
        writeln!(hstdout, "adc: {}mV", millivolts).unwrap();

        // let measurements = bme280.measure().unwrap();
        let measurements = bme280.measure();
        if measurements.is_err() {
            writeln!(hstdout, "ERROR READING {:?}", measurements.unwrap_err());
            continue;
        }
        let measurements_ok = measurements.unwrap();
        // Alternativelly, we could use match like in bme280.init()

        writeln!(hstdout,"Relative Humidity = {}%", measurements_ok.humidity).unwrap();
        writeln!(hstdout,"Temperature = {} deg C", measurements_ok.temperature).unwrap();
        writeln!(hstdout,"Pressure = {} pascals", measurements_ok.pressure).unwrap();
    }
}

