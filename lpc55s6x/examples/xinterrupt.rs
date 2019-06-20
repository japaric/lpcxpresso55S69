#![deny(warnings)]
#![no_main]
#![no_std]

use lpc55s6x::{Interrupt_0, Interrupt_1, BLUE, RED};
use panic_halt as _;

#[rtfm::app(device = lpc55s6x, cores = 2)]
const APP: () = {
    #[init(core = 0)]
    fn init0(_: init0::Context) {
        lpc55s6x::xpend(1, Interrupt_1::CTIMER0)
    }

    #[task(core = 0, binds = CTIMER0)]
    fn t0(_: t0::Context) {
        RED.on();
    }

    #[init(core = 1)]
    fn init1(_: init1::Context) {
        lpc55s6x::xpend(0, Interrupt_0::CTIMER0)
    }

    #[task(core = 1, binds = CTIMER0)]
    fn t1(_: t1::Context) {
        BLUE.on();
    }
};
