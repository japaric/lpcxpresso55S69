#![no_main]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

use cortex_m::asm;
use lpc55s6x as _;
use panic_halt as _;

static X: AtomicBool = AtomicBool::new(false);

#[no_mangle]
unsafe extern "C" fn main_0() -> ! {
    while !X.load(Ordering::Relaxed) {}

    asm::bkpt();

    loop {}
}

#[no_mangle]
unsafe extern "C" fn main_1() -> ! {
    X.store(true, Ordering::Relaxed);

    loop {}
}
