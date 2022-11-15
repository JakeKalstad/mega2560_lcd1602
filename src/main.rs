#![no_std]
#![no_main] 
use ag_lcd::{Display, LcdDisplay};

use panic_halt as _;
#[arduino_hal::entry]
fn main() -> ! {

    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let rs = pins.d2.into_output().downgrade();
    let en = pins.d3.into_output().downgrade();
    let d4 = pins.d4.into_output().downgrade();
    let d5 = pins.d5.into_output().downgrade();
    let d6 = pins.d6.into_output().downgrade();
    let d7 = pins.d7.into_output().downgrade();

    let delay = arduino_hal::Delay::new();
    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new(rs, en, delay)
        .with_half_bus(d4, d5, d6, d7)
        .with_display(Display::On)
        .build();
    lcd.clear();
    lcd.print("Hola San Juan!");
    loop {}
}