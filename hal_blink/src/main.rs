#![no_main]
#![no_std]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use hal::pac;
use nrf52833_hal::{self as hal, gpio::Level};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);

    let mut is_on = false;
    loop {
        let _ = row1.set_state(PinState::from(is_on));
        for _ in 0..100_000 {
            nop();
        }
        is_on = !is_on;
    }
}
