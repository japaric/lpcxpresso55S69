#![deny(warnings)]
#![no_main]
#![no_std]

use lpc55s6x::{Duration, BLUE, RED};
use panic_halt as _;

const PERIOD: u32 = 96_000_000; // about one second

#[rtfm::app(device = lpc55s6x, cores = 2, monotonic = lpc55s6x::CTIMER0)]
const APP: () = {
    #[init(core = 0, spawn = [ping])]
    fn init0(c: init0::Context) {
        c.spawn.ping().ok();
    }

    #[task(core = 0, schedule = [ping])]
    fn pong(c: pong::Context) {
        RED.toggle();

        c.schedule
            .ping(c.scheduled + Duration::from_cycles(PERIOD))
            .ok();
    }

    #[task(core = 1, schedule = [pong])]
    fn ping(c: ping::Context) {
        BLUE.toggle();

        c.schedule
            .pong(c.scheduled + Duration::from_cycles(PERIOD))
            .ok();
    }

    extern "C" {
        #[core = 0]
        fn GPIO_GLOBALINT0();
        #[core = 0]
        fn GPIO_GLOBALINT1();

        #[core = 1]
        fn GPIO_GLOBALINT0();
        #[core = 1]
        fn GPIO_GLOBALINT1();
    }
};
