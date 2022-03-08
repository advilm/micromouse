#![no_std]
#![no_main]

mod systick;

use teensy4_panic as _;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use imxrt_hal::gpio::GPIO;
use teensy4_bsp::pins::t40;

#[entry]
fn main() -> ! {
    let teensy_peripherals = teensy4_bsp::Peripherals::take().unwrap();
    let pins = t40::from_pads(teensy_peripherals.iomuxc);

    let cortex_m_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut systick = systick::new(cortex_m_peripherals.SYST);

    let mut pin14 = GPIO::new(pins.p14).output();

    loop {
        pin14.set_high().unwrap();
        systick.delay_ms(1_000);
        pin14.set_low().unwrap();
        systick.delay_ms(1_000);
    }
}