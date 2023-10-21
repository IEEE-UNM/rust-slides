#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use rand::SeedableRng;
use rand::{rngs::SmallRng, Rng};

use stm32f4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze();
    let mut delay = cp.SYST.delay(&clocks);

    // Setup ADC
    let mut adc = stm32f4xx_hal::adc::Adc::adc1(
        dp.ADC1,
        true,
        stm32f4xx_hal::adc::config::AdcConfig::default(),
    );
    adc.enable_temperature_and_vref();

    let seed = adc
        .read(&mut stm32f4xx_hal::adc::Temperature)
        .unwrap_or_default();
    let mut rng = SmallRng::seed_from_u64(seed as u64);

    loop {
        let number: u32 = rng.gen();
        defmt::println!("RNG: {}", number);
        delay.delay_ms(500_u32);
    }
}
