#![no_main]
#![no_std]

mod time;
use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin, StatefulOutputPin},
};
use microbit::{board::Board, hal::timer::Timer};
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let (mut col, mut row) = board.display_pins.degrade();
    let mut button_l = board.buttons.button_a.degrade();
    let mut button_r = board.buttons.button_b.degrade();

    row[0].set_high().ok();
    let active_col: usize = 0;

    loop {
        col[active_col].toggle().ok();
        timer.delay_ms(300);
        if button_l.is_low().unwrap() {}
        if button_r.is_low().unwrap() {}
    }
}
