#![deny(warnings)]
#![no_main]
#![no_std]

use lpc55s6x::{BLUE, RED};
use panic_halt as _;

#[rtfm::app(device = lpc55s6x, cores = 2)]
const APP: () = {
    #[init(core = 0)]
    fn init0(_: init0::Context) {
        RED.on();
    }

    #[idle(core = 0)]
    fn idle0(_: idle0::Context) -> ! {
        loop {
            // dim LED
            BLUE.toggle();
        }
    }

    #[init(core = 1)]
    fn init1(_: init1::Context) {
        BLUE.on();
    }

    #[idle(core = 1)]
    fn idle1(_: idle1::Context) -> ! {
        loop {
            // dim LED
            RED.toggle();
        }
    }
};
