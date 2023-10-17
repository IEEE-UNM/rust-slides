#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32l4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = stm32l4xx_hal::pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb2);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    let tx = gpiod
        .pd5
        .into_alternate(&mut gpiod.moder, &mut gpiod.otyper, &mut gpiod.afrl);
    let rx = gpiod
        .pd6
        .into_alternate(&mut gpiod.moder, &mut gpiod.otyper, &mut gpiod.afrl);
    let mut serial = stm32l4xx_hal::serial::Serial::usart2(
        dp.USART2,
        (tx, rx),
        9_600.bps(),
        clocks,
        &mut rcc.apb1r1,
    );

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
