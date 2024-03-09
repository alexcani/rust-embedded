#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::peripheral::{syst::SystClkSource, Peripherals};
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    let cp = Peripherals::take().unwrap();
    let mut systick = cp.SYST;
    systick.set_clock_source(SystClkSource::Core);
    systick.set_reload(8_000_000); // 1s

    hprintln!("Waiting for 1 second...").unwrap();
    systick.clear_current();
    systick.enable_counter();
    while !systick.has_wrapped() {}
    hprintln!("Done!").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

        loop {}
}
