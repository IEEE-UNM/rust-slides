#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use rand::SeedableRng;
use rand::{rngs::SmallRng, Rng};

use stm32l4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    let mut delay = stm32l4xx_hal::delay::Delay::new(cp.SYST, clocks);

    // Setup ADC
    let adc_common = stm32l4xx_hal::adc::AdcCommon::new(dp.ADC_COMMON, &mut rcc.ahb2);
    let mut adc =
        stm32l4xx_hal::adc::Adc::adc1(dp.ADC1, adc_common.clone(), &mut rcc.ccipr, &mut delay);
    let mut temperature = adc.enable_temperature(&mut delay).unwrap();

    let seed = adc.read(&mut temperature).unwrap_or_default();
    let mut rng = SmallRng::seed_from_u64(seed as u64);

    loop {
        let number: u32 = rng.gen();
        defmt::println!("RNG: {}", number);
        delay.delay_ms(500_u32);
    }
}
