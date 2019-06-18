#![deny(warnings)]
#![no_main]
#![no_std]

use core::{
    mem,
    sync::atomic::{AtomicBool, Ordering},
};

use cortex_m::peripheral::NVIC;
use lpc55s6x::{Interrupt, BLUE, RED};
use panic_halt as _;

static BARRIER: AtomicBool = AtomicBool::new(false);

#[no_mangle]
unsafe extern "C" fn main_0() -> ! {
    // unmask CTIMER0
    mem::transmute::<(), NVIC>(()).enable(Interrupt::CTIMER0);

    // unblock core #1
    BARRIER.store(true, Ordering::Release);

    loop {}
}

#[no_mangle]
extern "C" fn CTIMER0_0() {
    BLUE.on();

    // trigger core #1 CTIMER0 interrupt
    lpc55s6x::xpend(1, Interrupt::CTIMER0);
}

#[no_mangle]
unsafe extern "C" fn main_1() -> ! {
    while !BARRIER.load(Ordering::Acquire) {}

    // unmask CTIMER0
    mem::transmute::<(), NVIC>(()).enable(Interrupt::CTIMER0);

    // trigger core #0 CTIMER0 interrupt
    lpc55s6x::xpend(0, Interrupt::CTIMER0);

    loop {}
}

#[no_mangle]
extern "C" fn CTIMER0_1() {
    RED.on();
}
