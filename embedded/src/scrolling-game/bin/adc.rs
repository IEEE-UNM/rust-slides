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

    // Setup Analog Pin
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut pin = gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    let mut delay = stm32l4xx_hal::delay::Delay::new(cp.SYST, clocks);

    // Setup ADC
    let adc_common = stm32l4xx_hal::adc::AdcCommon::new(dp.ADC_COMMON, &mut rcc.ahb2);
    let mut adc =
        stm32l4xx_hal::adc::Adc::adc1(dp.ADC1, adc_common.clone(), &mut rcc.ccipr, &mut delay);
    let mut temperature = adc.enable_temperature(&mut delay).unwrap();

    loop {
        let value = adc.read(&mut pin).unwrap_or_default();
        let voltage = adc.to_millivolts(value);
        defmt::println!("Value: {}, {}mV", value, voltage);
        let temp = adc.read(&mut temperature).unwrap_or_default();
        defmt::println!("Temperature: {}", temp);
        delay.delay_ms(500_u32);
    }
}
