use embedded_hal::digital::{OutputPin, InputPin};
use super::{DigitalOutput, DigitalInput, DelayMs};

impl<P: OutputPin> DigitalOutput for OutputPinWrapper<P> {
    fn set_high(&mut self) {
        self.set_high();
    }
    fn set_low(&mut self) {
        self.set_low();
    }
    fn toggle(&mut self) {
        if self.state { self.set_low() } else { self.set_high() }
    }
}

pub struct OutputPinWrapper<P: OutputPin> {
    pin: P,
    state: bool,
}

impl<P: OutputPin> OutputPinWrapper<P> {
    pub fn new(pin: P) -> Self {
        Self { pin, state: false }
    }
}

impl<P: InputPin> DigitalInput for P {
    fn is_high(&self) -> bool {
        self.is_high()
    }
    fn is_low(&self) -> bool {
        self.is_low()
    }
}

pub struct ArduinoDelay;

impl DelayMs for ArduinoDelay {
    fn delay_ms(&mut self, ms: u16) {
        arduino_hal::delay_ms(ms as u32);
    }
}