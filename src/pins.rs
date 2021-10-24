use core::convert::Infallible;

use crate::{PeriodicTimer};
use bl602_hal::timer::Preload;
use embedded_hal::digital::blocking::OutputPin;
use embedded_time::duration::*;

// Struct to hold the actual pins.
// All pins must have the OutputPin trait. The OutputPin trait allows
// them to be used with set_low() and set_high() even though they are
// technically different types.
pub struct PinControl<'a> {
    pub timer: PeriodicTimer,
    pub pins: [&'a mut dyn OutputPin<Error=Infallible>; 4],
}

impl<'a> PinControl<'a> {
    
    pub fn set_pin_low_self(&mut self, pin: u8) {
        (&mut self.pins[pin as usize]).set_low().ok();
    }

    pub fn set_pin_high_self(&mut self, pin: u8) {
        (&mut self.pins[pin as usize]).set_high().ok();
    }

    pub fn periodic_start(&mut self, time: impl Into<Nanoseconds<u64>>) {
        let time: Nanoseconds<u64> = time.into();
        let timer = &mut self.timer;
        timer.set_match2(time);
        timer.enable_match2_interrupt();
        timer.set_preload_value(0.nanoseconds());
        timer.set_preload(Preload::PreloadMatchComparator2);
        timer.enable();
    }

    pub fn periodic_wait(&mut self) {
        let timer = &mut self.timer;
        loop {
            if timer.is_match2() {
                timer.clear_match2_interrupt();
                break;
            }
        }
    }
}
