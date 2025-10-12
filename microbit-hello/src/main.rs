// This is needed for embedded platforms
#![no_std]
#![no_main]

use cortex_m::asm::nop;
// Some panic handler needs to be included. This one halts the processor on panic.
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World!");
    let mut x = 0;
    loop {
        rprintln!("Echo ...");
        x += 1;
        rprintln!("x is equal to {}", x);
        for _ in 0 .. 100_000 {
            nop();
        }
    }
}
