#![no_std]
#![no_main]

pub mod hal;
pub mod app;

use panic_halt as _;

use hal::arduino::ArduinoDelay;
use crate::hal::arduino::OutputPinWrapper;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = OutputPinWrapper::new(pins.d13.into_output());
    let mut delay = ArduinoDelay;

    loop {
        app::blink(&mut led, &mut delay, 3);
    }
}
