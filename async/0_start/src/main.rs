#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::{
    delay::DelayNs,
    digital::{OutputPin, StatefulOutputPin},
};
use microbit::{board::Board, hal::timer::Timer};
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let _ = board.display_pins.col1.set_low();
    let mut row1 = board.display_pins.row1;

    loop {
        row1.toggle().ok();
        timer.delay_ms(300);
    }
}
