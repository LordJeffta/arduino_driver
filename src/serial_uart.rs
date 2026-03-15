use arduino_hal::{Peripherals, Pins, Usart};
use arduino_hal::hal::port::{PD0, PD1};
use arduino_hal::pac::USART0;
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;

pub struct SerialUart {
    serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>,
}

impl SerialUart {
    // pub fn new(baud: u32) -> Self {
    //     let peripheral: Peripherals = Peripherals::take().unwrap();
    //     let pins: Pins = arduino_hal::pins!(peripheral);
    //     let serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>> = arduino_hal::default_serial!(peripheral, pins, baud);
    //     SerialUart { serial }
    // }

    pub fn new(peripherals: &Peripherals, pins: &Pins, baud: u32) -> Self {
        let serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>> = arduino_hal::default_serial!(peripherals, pins, baud);
        SerialUart { serial }
    }

    pub fn writeln(&mut self, string: &str){
        ufmt::uwriteln!(self.serial, "{}", string).unwrap_infallible();
    }
}