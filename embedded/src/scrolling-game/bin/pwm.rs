#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use defmt_rtt as _;
use panic_halt as _;

use stm32l4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = stm32l4xx_hal::pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    // TIM2
    let c1 = gpioa
        .pa0
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let c2 = gpioa
        .pa1
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let c3 = gpioa
        .pa2
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let c4 = gpioa
        .pa3
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    let mut pwm = dp
        .TIM2
        .pwm((c1, c2, c3, c4), 1.kHz(), clocks, &mut rcc.apb1r1)
        .3;

    let max = pwm.get_max_duty();

    pwm.enable();
    pwm.set_duty(max / 2);

    loop {
        // Tells CPU to sleep
        cortex_m::asm::nop();
    }
}
