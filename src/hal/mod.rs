pub mod arduino;

pub trait DigitalOutput {
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn toggle(&mut self);
}

pub trait DigitalInput {
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}

pub trait DelayMs {
    fn delay_ms(&mut self, ms: u16);
}

pub trait Serial {
    fn write_byte(&mut self, byte: u8);
    fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            self.write_byte(b);
        }
    }
}