#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32l4xx_hal::{
    delay::Delay,
    gpio::{Output, Pin, PushPull, L8},
    pac,
    prelude::*,
};

/// Takes ownership of D13 and sets it to high.
pub fn takes_ownership(mut d13: Pin<Output<PushPull>, L8, 'B', 2>) {
    d13.set_high();
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    // Setup pin
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let led = gpiob
        .pb2
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // Setup delay
    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    let mut delay = Delay::new(cp.SYST, clocks);

    defmt::println!("Moving Variable");
    // Moving 1
    let mut led2 = led;
    led2.set_high();
    delay.delay_ms(1000_u32);
    // Does not compile
    // led.set_low();
    led2.set_low();

    delay.delay_ms(1000_u32);
    defmt::println!("Moving Functions");
    // Moving 2
    takes_ownership(led2);
    // Does not compile
    // led2.set_low();

    loop {}
}
