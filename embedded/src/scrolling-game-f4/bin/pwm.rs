#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use defmt_rtt as _;
use panic_halt as _;

use stm32f4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();

    // TIM2
    let c1 = gpioa.pa0;
    let c2 = gpioa.pa1;

    let channels = (
        stm32f4xx_hal::timer::Channel1::new(c1),
        stm32f4xx_hal::timer::Channel2::new(c2),
    );
    let pwm = dp.TIM2.pwm_hz(channels, 1_u32.kHz(), &clocks);

    let max = pwm.get_max_duty();
    let (mut ch1, _) = pwm.split();

    ch1.set_duty(max / 2);
    ch1.enable();

    loop {
        // Tells CPU to sleep
        cortex_m::asm::nop();
    }
}
