pub mod arduino {
    use arduino_hal::{Peripherals, Pins, Usart};
    use arduino_hal::hal::port::{PD0, PD1};
    use arduino_hal::pac::USART0;
    use arduino_hal::port::mode::{Input, Output};
    use arduino_hal::port::{Pin, D13};

    pub struct Arduino {
        peripheral: Peripherals,
        pins: Pins,
        serial: Option<Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>>
    }

    impl Arduino{
        pub fn new(&mut self) -> &Self {
            self.peripheral = Peripherals::take().unwrap();
            self.pins = arduino_hal::pins!(self.peripheral);
            self.serial = Option::None;
            self
        }

        pub fn get_pin(&mut self, pin: u16) -> Pin<Output, D13> {
            let pin: Pin<Output, D13>;
            if let Some(value) = self.pins.as_mut() {
                // `value` is `&mut i32`, so you can modify it
                pin = value.d13.into_output();
            }
            return pin;
            // match self.pins {
            //     Some(p) => self.pins.unwrap().d13.into_output(),
            //     None => {
            //         self.pins = arduino_hal::pins!(self.peripheral).into();
            //         self.get_pin(pin)
            //     }
            // }
        }
    }
}