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

let gpioc = dp.GPIOC.split();
let button = gpioc.pc13.into_pull_down_input();

let clocks = rcc.cfgr.freeze();
let mut delay = cp.SYST.delay(&clocks);

loop {
let is_high = button.is_high();
let is_low = button.is_low();
defmt::println!("{}, {}", is_high, is_low);
delay.delay_ms(100_u32);
}
}
