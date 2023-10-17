#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32l4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let button = gpioa
        .pa0
        .into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    let mut delay = stm32l4xx_hal::delay::Delay::new(cp.SYST, clocks);

    loop {
        let is_high = button.is_high();
        let is_low = button.is_low();
        defmt::println!("{}, {}", is_high, is_low);
        delay.delay_ms(100_u32);
    }
}
