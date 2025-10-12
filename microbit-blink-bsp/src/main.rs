#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;
use panic_halt as _;
use microbit::{board::Board, hal::timer::Timer};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut board: Board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let _ = board.display_pins.col1.set_low();
    let mut row1 = board.display_pins.row1;

    loop {
        let _ = row1.set_low();
        timer.delay_ms(1_000);
        let _ = row1.set_high();
        timer.delay_ms(1_000);
    }
}
