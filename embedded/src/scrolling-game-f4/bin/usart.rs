#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32f4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();
let rcc = dp.RCC.constrain();

let gpioa = dp.GPIOA.split();
let clocks = rcc.cfgr.freeze();
let mut serial = dp.USART2.serial((gpioa.pa2, gpioa.pa3), 9600.bps(), &clocks).unwrap();

loop {
if let Ok(c) = nb::block!(serial.read()) {
if c != 10 {
for i in "You Typed: ".as_bytes() {
nb::block!(serial.write(*i)).unwrap_or_default();
}
// Printing character
nb::block!(serial.write(c)).unwrap_or_default();
// Printing new line
nb::block!(serial.write(10)).unwrap_or_default();
// Printing carriage return line
nb::block!(serial.write(13)).unwrap_or_default();
};
};
}
}
