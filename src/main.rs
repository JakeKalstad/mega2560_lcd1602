#![no_std]
#![no_main] 
use core::pin::Pin;
use core::time::Duration;

use ag_lcd::{Display, LcdDisplay};
use panic_halt as _;
use embedded_time;
 
fn calibration_calc(temp: f32) -> (f32, Duration) {
    /// Speed of sound at 0C in m/s.
    const SOUND_SPEED_0C: f32 = 331.3;
    /// Increase speed of sound over temperature factor m/[sC].
    const SOUND_SPEED_INC_OVER_TEMP: f32 = 0.606;
    /// Maximum measuring range for HC-SR04 sensor in m.
    const MAX_RANGE: f32 = 4.0;

    // Speed of sound, depending on ambient temperature (if `temp` is `None`, default to 20C).
    let sound_speed = SOUND_SPEED_0C + (SOUND_SPEED_INC_OVER_TEMP * temp);

    // Polling timeout for **ECHO** pin: since max range for HC-SR04 is 4m, it doesn't make
    // sense to wait longer than the time required to the ultrasonic sound wave to cover the
    // max range distance. In other words, if the timeout is reached, the measurement was not
    // successfull or the object is located too far away from the sensor in order to be
    // detected.
    let timeout = Duration::from_secs_f32(MAX_RANGE / sound_speed);

    (sound_speed, timeout)
}

#[arduino_hal::entry]
fn main() -> ! {
    // grab all peripherals - pins included
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    // set the rs/en and data pins for the LCD 
    let rs = pins.d2.into_output().downgrade();
    let en = pins.d3.into_output().downgrade();
    let d4 = pins.d4.into_output().downgrade();
    let d5 = pins.d5.into_output().downgrade();
    let d6 = pins.d6.into_output().downgrade();
    let d7 = pins.d7.into_output().downgrade();

    // set the button pins
    let button_io_pin = pins.d12.into_pull_up_input();
    
    // set the led pin
    let mut led_io_pin = pins.d13.into_output().downgrade();
    
    // sonar sensor pins
    let mut trigger_pin = pins.d9.into_output();
    let echo_pin = pins.d8.into_pull_up_input();

    // turn on the led
    led_io_pin.set_high();
    
    // Create the LcdDisplay with pins and options
    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new(rs, en, arduino_hal::Delay::new())
        .with_half_bus(d4, d5, d6, d7)
        .with_lines(ag_lcd::Lines::TwoLines)
        .with_display(Display::On)
        .build();
    
    // clear lcd in case of lingering characters or the arduino didn't restart
    lcd.clear();
    
    // Print to first row of pixels
    lcd.print("Emigrate.. or..");
    // move to second one
    lcd.set_position(0,1);
    // print again
    lcd.print("...Degenerate!");
    // endless loop (this makes return type `!` )
    let triggered = false;
    let timer = Timer::new(hal::pac::TIMER, &mut pac.RESETS);

    let mut first_time: bool = true;
    loop {
        if first_time {
            arduino_hal::delay_ms(1000);   // Initial delay to let transducer settle.
            first_time = false;
        }
        // check if the button input pin is high (pressed)
        if button_io_pin.is_high(){
            lcd.clear();
            lcd.print("Saith my heart!");
            // toggle the led
            led_io_pin.toggle();
        }

        // ultrasonic
        if !triggered {
            trigger_pin.set_low();
            arduino_hal::delay_us(2);
            trigger_pin.set_high();
            arduino_hal::delay_us(10);
            trigger_pin.set_low();
            let ticks = start.elapsed();
                let hz = self.timer.frequency().0;
                let distance_mm = (ticks * 171_605) / hz;
        }
        
    }
}