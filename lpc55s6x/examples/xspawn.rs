#![deny(warnings)]
#![no_main]
#![no_std]

use lpc55s6x::{BLUE, RED};
use panic_halt as _;

#[rtfm::app(device = lpc55s6x, cores = 2)]
const APP: () = {
    #[init(core = 0, spawn = [ping])]
    fn init0(c: init0::Context) {
        c.spawn.ping().ok();
    }

    #[task(core = 0)]
    fn pong(_: pong::Context) {
        RED.on();
    }

    #[task(core = 1, spawn = [pong])]
    fn ping(c: ping::Context) {
        BLUE.on();

        c.spawn.pong().ok();
    }

    extern "C" {
        #[core = 0]
        fn CTIMER0();

        #[core = 1]
        fn CTIMER0();
    }
};
