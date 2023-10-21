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

let gpioa = dp.GPIOA.split();
let mut led = gpioa.pa5.into_push_pull_output();

let clocks = rcc.cfgr.freeze();
let mut delay = cp.SYST.delay(&clocks);

loop {
led.set_high();
delay.delay_ms(1000_u32);
led.set_low();
delay.delay_us(1_000_000_u32);
}
}
