#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32f4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
let cp = cortex_m::Peripherals::take().unwrap();
let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();

let rcc = dp.RCC.constrain();

// Setup Analog Pin
let gpioa = dp.GPIOA.split();
let mut pin = gpioa.pa0.into_analog();

let clocks = rcc.cfgr.freeze();
let mut delay = cp.SYST.delay(&clocks);

// Setup ADC
let config =  stm32f4xx_hal::adc::config::AdcConfig::default();
let mut adc = stm32f4xx_hal::adc::Adc::adc1(dp.ADC1, true, config);
adc.enable_temperature_and_vref();

loop {
let value = adc.read(&mut pin).unwrap_or_default();
let voltage = adc.sample_to_millivolts(value);
defmt::println!("Value: {}, {}mV", value, voltage);
let temp = adc.read(&mut stm32f4xx_hal::adc::Temperature).unwrap_or_default();
defmt::println!("Temperature: {}", temp);
delay.delay_ms(500_u32);
}
}
