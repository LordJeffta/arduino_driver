#![no_std]
#![no_main]

pub mod serial_uart;
pub mod arduino;

use arduino_hal::{Peripherals, Pins};
use arduino_hal::port::{Pin, D13, D9};
use arduino_hal::port::mode::{Output};
use panic_halt as _;
use crate::serial_uart::SerialUart;

#[arduino_hal::entry]
fn main() -> ! {
    let dp: Peripherals = Peripherals::take().unwrap();
    let pins: Pins = arduino_hal::pins!(dp);

    let mut led: Pin<Output, D9> = pins.d9.into_output();
    led.set_high();
    arduino_hal::delay_ms(1000);

    let singleton_ref = &*dp.unwrap();
    let mut serial = SerialUart::new(&dp, &pins, 57600);
    led.set_low();
    // let mut serial2 = SerialUart::new(960);

    // let mut serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>> = arduino_hal::default_serial!(dp, pins, 57600);
    // ufmt::uwriteln!(&mut serial, "Hello World!").unwrap_infallible();

    serial.writeln("Hello World!".into());
    let mut led: Pin<Output, D13> = pins.d13.into_output();

    loop {
        led.toggle();
        // serial2.writeln("toggle".into());
        arduino_hal::delay_ms(1000);
    }
}
