#![deny(warnings)]
#![no_main]
#![no_std]

use core::sync::atomic::{self, Ordering};

use lpc55s6x::{BLUE, GREEN};
use panic_halt as _;

#[no_mangle]
unsafe extern "C" fn main_0() -> ! {
    GREEN.on();

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[no_mangle]
unsafe extern "C" fn main_1() -> ! {
    BLUE.on();

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
