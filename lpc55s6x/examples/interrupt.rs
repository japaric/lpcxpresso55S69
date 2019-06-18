//! Sanity check that vector tables are working (and are different)

#![deny(warnings)]
#![no_main]
#![no_std]

use core::mem;

use cortex_m::peripheral::NVIC;
use lpc55s6x::{Interrupt, BLUE, GREEN};
use panic_halt as _;

#[no_mangle]
unsafe extern "C" fn main_0() -> ! {
    // unmask CTIMER0
    mem::transmute::<(), NVIC>(()).enable(Interrupt::CTIMER0);

    // pend CTIMER0
    NVIC::pend(Interrupt::CTIMER0);

    loop {}
}

#[no_mangle]
extern "C" fn CTIMER0_0() {
    BLUE.on();
}

#[no_mangle]
unsafe extern "C" fn main_1() -> ! {
    // unmask CTIMER0
    mem::transmute::<(), NVIC>(()).enable(Interrupt::CTIMER0);

    // pend CTIMER0
    NVIC::pend(Interrupt::CTIMER0);

    loop {}
}

#[no_mangle]
extern "C" fn CTIMER0_1() {
    GREEN.on();
}
