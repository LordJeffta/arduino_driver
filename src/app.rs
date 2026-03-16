use crate::hal::{DigitalOutput, DigitalInput, DelayMs};

pub fn blink<Led, Delay>(led: &mut Led, delay: &mut Delay, times: u8)
where
    Led: DigitalOutput,
    Delay: DelayMs,
{
    for _ in 0..times {
        led.set_high();
        delay.delay_ms(1000);
        led.set_low();
        delay.delay_ms(1000);
    }
}

pub fn read_and_echo<In, Out>(input: &In, output: &mut Out)
where
    In: DigitalInput,
    Out: DigitalOutput,
{
    if input.is_high() {
        output.set_high();
    } else {
        output.set_low();
    }
}