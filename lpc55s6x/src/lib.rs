#![allow(non_camel_case_types)]
#![deny(warnings)]
#![no_std]

use core::{
    cmp,
    convert::{Infallible, TryInto},
    fmt, ops,
    sync::atomic::{self, Ordering},
};

use bare_metal::Nr;
use rtfm::Monotonic;

// the vector table of both cores have the exact same layout
pub use Interrupt_0 as Interrupt_1;

pub const NVIC_PRIO_BITS: u8 = 3;

/* Mailbox: forward interrupts */
const MAILBOX_BASE: usize = 0x5008_B000;
const MAILBOX_IRQ0: *mut u32 = (MAILBOX_BASE + 0x000) as *mut u32;
const MAILBOX_IRQ0SET: *mut u32 = (MAILBOX_BASE + 0x004) as *mut u32;
const MAILBOX_IRQ0CLR: *mut u32 = (MAILBOX_BASE + 0x008) as *mut u32;
const MAILBOX_IRQ1: *mut u32 = (MAILBOX_BASE + 0x010) as *mut u32;
const MAILBOX_IRQ1SET: *mut u32 = (MAILBOX_BASE + 0x014) as *mut u32;
const MAILBOX_IRQ1CLR: *mut u32 = (MAILBOX_BASE + 0x018) as *mut u32;

pub fn xpend(core: u8, interrupt: impl Nr) {
    let nr = interrupt.nr();

    if nr < 32 {
        let mask = 1 << nr;

        if core == 0 {
            unsafe {
                MAILBOX_IRQ1SET.write_volatile(mask);
            }
        } else if core == 1 {
            unsafe {
                MAILBOX_IRQ0SET.write_volatile(mask);
            }
        }
    }
}

const NVIC_ISER: *mut u32 = 0xE000_E100 as *mut u32;
const NVIC_ISPR: *mut u32 = 0xE000_E200 as *mut u32;

#[no_mangle]
unsafe extern "C" fn MAILBOX_0() {
    let mask = MAILBOX_IRQ1.read_volatile();
    NVIC_ISPR.write_volatile(mask);
    MAILBOX_IRQ1CLR.write_volatile(mask);
}

#[no_mangle]
unsafe extern "C" fn MAILBOX_1() {
    let mask = MAILBOX_IRQ0.read_volatile();
    NVIC_ISPR.write_volatile(mask);
    MAILBOX_IRQ0CLR.write_volatile(mask);
}

/* LEDs */
const GPIO_BASE: usize = 0x4008_C000;
const GPIO_SET1: *mut u32 = (GPIO_BASE + 0x2204) as *mut u32;
const GPIO_CLR1: *mut u32 = (GPIO_BASE + 0x2284) as *mut u32;
const GPIO_NOT1: *mut u32 = (GPIO_BASE + 0x2304) as *mut u32;

pub struct RED;

impl RED {
    const OFFSET: u8 = 6;

    pub fn on(&self) {
        unsafe {
            GPIO_CLR1.write_volatile(1 << Self::OFFSET);
        }
    }

    pub fn off(&self) {
        unsafe {
            GPIO_SET1.write_volatile(1 << Self::OFFSET);
        }
    }

    pub fn toggle(&self) {
        unsafe {
            GPIO_NOT1.write_volatile(1 << Self::OFFSET);
        }
    }
}

pub struct BLUE;

impl BLUE {
    const OFFSET: u8 = 4;

    pub fn on(&self) {
        unsafe {
            GPIO_CLR1.write_volatile(1 << Self::OFFSET);
        }
    }

    pub fn off(&self) {
        unsafe {
            GPIO_SET1.write_volatile(1 << Self::OFFSET);
        }
    }

    pub fn toggle(&self) {
        unsafe {
            GPIO_NOT1.write_volatile(1 << Self::OFFSET);
        }
    }
}

pub struct GREEN;

impl GREEN {
    const OFFSET: u8 = 7;

    pub fn on(&self) {
        unsafe {
            GPIO_CLR1.write_volatile(1 << Self::OFFSET);
        }
    }

    pub fn off(&self) {
        unsafe {
            GPIO_SET1.write_volatile(1 << Self::OFFSET);
        }
    }

    pub fn toggle(&self) {
        unsafe {
            GPIO_NOT1.write_volatile(1 << Self::OFFSET);
        }
    }
}

// Core #0 entry point
#[no_mangle]
unsafe extern "C" fn _start_0() -> ! {
    const SYSCON_AHBCLKCTRLSET0: *mut u32 = (SYSCON_BASE + 0x220) as *mut u32;
    const SYSCON_AHBCLKCTRLSET1: *mut u32 = (SYSCON_BASE + 0x224) as *mut u32;
    const SYSCON_BASE: usize = 0x5000_0000;
    const SYSCON_CPBOOT: *mut u32 = (SYSCON_BASE + 0x804) as *mut u32;
    const SYSCON_CPUCTRL: *mut u32 = (SYSCON_BASE + 0x800) as *mut u32;
    const SYSCON_CPUCTRL_KEY: u32 = 0xc0c4 << 16;
    const SYSCON_CTIMERCLKSEL0: *mut u32 = (SYSCON_BASE + 0x26C) as *mut u32;

    // enable the MAILBOX peripheral
    SYSCON_AHBCLKCTRLSET0.write_volatile(1 << 26);

    // use the main clock to drive CTIMER0
    SYSCON_CTIMERCLKSEL0.write_volatile(0);

    // enable the CTIMER0 peripheral
    SYSCON_AHBCLKCTRLSET1.write_volatile(1 << 26);

    // hold the CTIMER0 counter in reset
    CTIMER0_TCR.write_volatile(0b11);

    // unmask the MAILBOX interrupt
    NVIC_ISER.write_volatile(1 << Interrupt_0::MAILBOX.nr());

    // configure pins that drive LEDs
    const GPIO_DIRSET1: *mut u32 = (GPIO_BASE + 0x2384) as *mut u32;

    let mask = (1 << RED::OFFSET) | (1 << GREEN::OFFSET) | (1 << BLUE::OFFSET);
    GPIO_SET1.write_volatile(mask); // set high
    GPIO_DIRSET1.write_volatile(mask); // set as outputs

    extern "C" {
        fn main_0() -> !;

        static mut _sbss: u32;
        static mut _ebss: u32;

        static mut _sdata: u32;
        static mut _edata: u32;
        static _sidata: u32;

        static _vectors_1: u32;
    }

    /* initialize static variables */
    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    atomic::compiler_fence(Ordering::SeqCst);

    /* boot core #1 */
    // set core #1 VTOR (vector table offset)
    SYSCON_CPBOOT.write_volatile(&_vectors_1 as *const u32 as usize as u32);

    // NOTE the user manual (UM11126 v1.3) got the meaning of these bits backwards
    // enable core #1 clock and hold it in reset
    SYSCON_CPUCTRL.write_volatile(SYSCON_CPUCTRL_KEY | (1 << 5) | (1 << 3));
    // release the core from the reset state
    SYSCON_CPUCTRL.write_volatile(SYSCON_CPUCTRL_KEY | (0 << 5) | (1 << 3));

    atomic::compiler_fence(Ordering::SeqCst);

    main_0()
}

#[no_mangle]
unsafe extern "C" fn _start_1() -> ! {
    extern "C" {
        fn main_1() -> !;
    }

    // unmask the MAILBOX interrupt
    NVIC_ISER.write_volatile(1 << Interrupt_1::MAILBOX.nr());

    atomic::compiler_fence(Ordering::SeqCst);

    main_1()
}

/* Interrupts */
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Interrupt_0 {
    WDT = 0,
    SDMA0 = 1,
    GPIO_GLOBALINT0 = 2,
    GPIO_GLOBALINT1 = 3,
    GPIO_INT0_IRQ0 = 4,
    GPIO_INT0_IRQ1 = 5,
    GPIO_INT0_IRQ2 = 6,
    GPIO_INT0_IRQ3 = 7,
    UTICK = 8,
    MRT = 9,
    CTIMER0 = 10,
    CTIMER1 = 11,
    SCT = 12,
    CTIMER3 = 13,
    Flexcomm0 = 14,
    Flexcomm1 = 15,
    Flexcomm2 = 16,
    Flexcomm3 = 17,
    Flexcomm4 = 18,
    Flexcomm5 = 19,
    Flexcomm6 = 20,
    Flexcomm7 = 21,
    ADC = 22,
    ACMP = 24,
    USB0_NEEDCLK = 27,
    USB0 = 28,
    RTC = 29,
    MAILBOX = 31,
    PIN_INT4 = 32,
    PIN_INT5 = 33,
    PIN_INT6 = 34,
    PIN_INT7 = 35,
    CTIMER2 = 36,
    CTIMER4 = 37,
    OSEVTIMER = 38,
    SDIO = 42,
    USB1 = 47,
    USB1_NEEDCLK = 48,
    HYPERVISOR = 49,
    SGPIO_INT0_IRQ0 = 50,
    SGPIO_INT0_IRQ1 = 51,
    PLU = 52,
    SEC_VIO = 53,
    HASH = 54,
    CASPER = 55,
    PUF = 56,
    PQ = 57,
    SDMA1 = 58,
    HS_SPI = 59,
}

unsafe impl Nr for Interrupt_0 {
    fn nr(&self) -> u8 {
        *self as u8
    }
}

#[no_mangle]
unsafe extern "C" fn DefaultHandler_(_ef: &ExceptionFrame) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[repr(C)]
pub struct ExceptionFrame {
    /// (General purpose) Register 0
    pub r0: u32,

    /// (General purpose) Register 1
    pub r1: u32,

    /// (General purpose) Register 2
    pub r2: u32,

    /// (General purpose) Register 3
    pub r3: u32,

    /// (General purpose) Register 12
    pub r12: u32,

    /// Linker Register
    pub lr: u32,

    /// Program Counter
    pub pc: u32,

    /// Program Status Register
    pub xpsr: u32,
}

extern "C" {
    fn NonMaskableInt_0();
    fn HardFault_0();
    fn MemoryManagement_0();
    fn BusFault_0();
    fn UsageFault_0();
    fn SecureFault_0();
    fn SVCall_0();
    fn DebugMonitor_0();
    fn PendSV_0();
    fn SysTick_0();

    fn WDT_0();
    fn SDMA0_0();
    fn GPIO_GLOBALINT0_0();
    fn GPIO_GLOBALINT1_0();
    fn GPIO_INT0_IRQ0_0();
    fn GPIO_INT0_IRQ1_0();
    fn GPIO_INT0_IRQ2_0();
    fn GPIO_INT0_IRQ3_0();
    fn UTICK_0();
    fn MRT_0();
    fn CTIMER0_0();
    fn CTIMER1_0();
    fn SCT_0();
    fn CTIMER3_0();
    fn Flexcomm0_0();
    fn Flexcomm1_0();
    fn Flexcomm2_0();
    fn Flexcomm3_0();
    fn Flexcomm4_0();
    fn Flexcomm5_0();
    fn Flexcomm6_0();
    fn Flexcomm7_0();
    fn ADC_0();
    fn ACMP_0();
    fn USB0_NEEDCLK_0();
    fn USB0_0();
    fn RTC_0();
    // fn MAILBOX_0();
    fn PIN_INT4_0();
    fn PIN_INT5_0();
    fn PIN_INT6_0();
    fn PIN_INT7_0();
    fn CTIMER2_0();
    fn CTIMER4_0();
    fn OSEVTIMER_0();
    fn SDIO_0();
    fn USB1_0();
    fn USB1_NEEDCLK_0();
    fn HYPERVISOR_0();
    fn SGPIO_INT0_IRQ0_0();
    fn SGPIO_INT0_IRQ1_0();
    fn PLU_0();
    fn SEC_VIO_0();
    fn HASH_0();
    fn CASPER_0();
    fn PUF_0();
    fn PQ_0();
    fn SDMA1_0();
    fn HS_SPI_0();
}

#[link_section = ".vectors.0"]
#[no_mangle]
static VECTORS_0: [Option<unsafe extern "C" fn()>; 74] = [
    // exception 2
    Some(NonMaskableInt_0),
    // exception 3
    Some(HardFault_0),
    // exception 4
    Some(MemoryManagement_0),
    // exception 5
    Some(BusFault_0),
    // exception 6
    Some(UsageFault_0),
    // exception 7
    Some(SecureFault_0),
    // exceptions 8-10
    None,
    None,
    None,
    // exception 11
    Some(SVCall_0),
    // exception 12
    Some(DebugMonitor_0),
    None,
    // exception 14
    Some(PendSV_0),
    // exception 15
    Some(SysTick_0),
    // 0 -- start of interrupts
    Some(WDT_0),
    Some(SDMA0_0),
    Some(GPIO_GLOBALINT0_0),
    Some(GPIO_GLOBALINT1_0),
    Some(GPIO_INT0_IRQ0_0),
    Some(GPIO_INT0_IRQ1_0),
    Some(GPIO_INT0_IRQ2_0),
    Some(GPIO_INT0_IRQ3_0),
    Some(UTICK_0),
    Some(MRT_0),
    // 10
    Some(CTIMER0_0),
    Some(CTIMER1_0),
    Some(SCT_0),
    Some(CTIMER3_0),
    Some(Flexcomm0_0),
    Some(Flexcomm1_0),
    Some(Flexcomm2_0),
    Some(Flexcomm3_0),
    Some(Flexcomm4_0),
    Some(Flexcomm5_0),
    // 20
    Some(Flexcomm6_0),
    Some(Flexcomm7_0),
    Some(ADC_0),
    None,
    Some(ACMP_0),
    None,
    None,
    Some(USB0_NEEDCLK_0),
    Some(USB0_0),
    Some(RTC_0),
    // 30
    None,
    Some(MAILBOX_0),
    Some(PIN_INT4_0),
    Some(PIN_INT5_0),
    Some(PIN_INT6_0),
    Some(PIN_INT7_0),
    Some(CTIMER2_0),
    Some(CTIMER4_0),
    Some(OSEVTIMER_0),
    None,
    // 40
    None,
    None,
    Some(SDIO_0),
    None,
    None,
    None,
    None,
    Some(USB1_0),
    Some(USB1_NEEDCLK_0),
    Some(HYPERVISOR_0),
    // 50
    Some(SGPIO_INT0_IRQ0_0),
    Some(SGPIO_INT0_IRQ1_0),
    Some(PLU_0),
    Some(SEC_VIO_0),
    Some(HASH_0),
    Some(CASPER_0),
    Some(PUF_0),
    Some(PQ_0),
    Some(SDMA1_0),
    Some(HS_SPI_0),
];

extern "C" {
    fn NonMaskableInt_1();
    fn HardFault_1();
    fn MemoryManagement_1();
    fn BusFault_1();
    fn UsageFault_1();
    fn SecureFault_1();
    fn SVCall_1();
    fn DebugMonitor_1();
    fn PendSV_1();
    fn SysTick_1();

    fn WDT_1();
    fn SDMA0_1();
    fn GPIO_GLOBALINT0_1();
    fn GPIO_GLOBALINT1_1();
    fn GPIO_INT0_IRQ0_1();
    fn GPIO_INT0_IRQ1_1();
    fn GPIO_INT0_IRQ2_1();
    fn GPIO_INT0_IRQ3_1();
    fn UTICK_1();
    fn MRT_1();
    fn CTIMER0_1();
    fn CTIMER1_1();
    fn SCT_1();
    fn CTIMER3_1();
    fn Flexcomm0_1();
    fn Flexcomm1_1();
    fn Flexcomm2_1();
    fn Flexcomm3_1();
    fn Flexcomm4_1();
    fn Flexcomm5_1();
    fn Flexcomm6_1();
    fn Flexcomm7_1();
    fn ADC_1();
    fn ACMP_1();
    fn USB0_NEEDCLK_1();
    fn USB0_1();
    fn RTC_1();
    // fn MAILBOX_1();
    fn PIN_INT4_1();
    fn PIN_INT5_1();
    fn PIN_INT6_1();
    fn PIN_INT7_1();
    fn CTIMER2_1();
    fn CTIMER4_1();
    fn OSEVTIMER_1();
    fn SDIO_1();
    fn USB1_1();
    fn USB1_NEEDCLK_1();
    fn HYPERVISOR_1();
    fn SGPIO_INT0_IRQ0_1();
    fn SGPIO_INT0_IRQ1_1();
    fn PLU_1();
    fn SEC_VIO_1();
    fn HASH_1();
    fn CASPER_1();
    fn PUF_1();
    fn PQ_1();
    fn SDMA1_1();
    fn HS_SPI_1();
}

#[link_section = ".vectors.1"]
#[no_mangle]
static VECTORS_1: [Option<unsafe extern "C" fn()>; 74] = [
    // exception 2
    Some(NonMaskableInt_1),
    // exception 3
    Some(HardFault_1),
    // exception 4
    Some(MemoryManagement_1),
    // exception 5
    Some(BusFault_1),
    // exception 6
    Some(UsageFault_1),
    // exception 7
    Some(SecureFault_1),
    // exceptions 8-10
    None,
    None,
    None,
    // exception 11
    Some(SVCall_1),
    // exception 12
    Some(DebugMonitor_1),
    None,
    // exception 14
    Some(PendSV_1),
    // exception 15
    Some(SysTick_1),
    // 0 -- start of interrupts
    Some(WDT_1),
    Some(SDMA0_1),
    Some(GPIO_GLOBALINT0_1),
    Some(GPIO_GLOBALINT1_1),
    Some(GPIO_INT0_IRQ0_1),
    Some(GPIO_INT0_IRQ1_1),
    Some(GPIO_INT0_IRQ2_1),
    Some(GPIO_INT0_IRQ3_1),
    Some(UTICK_1),
    Some(MRT_1),
    // 10
    Some(CTIMER0_1),
    Some(CTIMER1_1),
    Some(SCT_1),
    Some(CTIMER3_1),
    Some(Flexcomm0_1),
    Some(Flexcomm1_1),
    Some(Flexcomm2_1),
    Some(Flexcomm3_1),
    Some(Flexcomm4_1),
    Some(Flexcomm5_1),
    // 20
    Some(Flexcomm6_1),
    Some(Flexcomm7_1),
    Some(ADC_1),
    None,
    Some(ACMP_1),
    None,
    None,
    Some(USB0_NEEDCLK_1),
    Some(USB0_1),
    Some(RTC_1),
    // 30
    None,
    Some(MAILBOX_1),
    Some(PIN_INT4_1),
    Some(PIN_INT5_1),
    Some(PIN_INT6_1),
    Some(PIN_INT7_1),
    Some(CTIMER2_1),
    Some(CTIMER4_1),
    Some(OSEVTIMER_1),
    None,
    // 40
    None,
    None,
    Some(SDIO_1),
    None,
    None,
    None,
    None,
    Some(USB1_1),
    Some(USB1_NEEDCLK_1),
    Some(HYPERVISOR_1),
    // 50
    Some(SGPIO_INT0_IRQ0_1),
    Some(SGPIO_INT0_IRQ1_1),
    Some(PLU_1),
    Some(SEC_VIO_1),
    Some(HASH_1),
    Some(CASPER_1),
    Some(PUF_1),
    Some(PQ_1),
    Some(SDMA1_1),
    Some(HS_SPI_1),
];

/* Instant API */
const CTIMER0_BASE: usize = 0x4000_8000;
const CTIMER0_TCR: *mut u32 = (CTIMER0_BASE + 0x04) as *mut u32;
const CTIMER0_TC: *mut u32 = (CTIMER0_BASE + 0x08) as *mut u32;

pub struct CTIMER0;

unsafe impl Monotonic for CTIMER0 {
    type Instant = Instant;

    fn ratio() -> u32 {
        1
    }

    fn now() -> Instant {
        Instant::now()
    }

    /// Resets the counter to *zero*
    unsafe fn reset() {
        CTIMER0_TCR.write_volatile(0b01); // release from reset
    }

    fn zero() -> Instant {
        Instant { inner: 0 }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Instant {
    inner: i32,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            inner: unsafe { CTIMER0_TC.read_volatile() } as i32,
        }
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        let diff = self.inner.wrapping_sub(earlier.inner);
        assert!(diff >= 0, "second instant is later than `self`");
        Duration { inner: diff as u32 }
    }
}

impl fmt::Debug for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Instant")
            .field(&(self.inner as u32))
            .finish()
    }
}

impl ops::Add<Duration> for Instant {
    type Output = Self;

    fn add(self, dur: Duration) -> Instant {
        Instant {
            inner: self.inner.wrapping_add(dur.inner as i32),
        }
    }
}

impl ops::Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Duration {
        self.duration_since(rhs)
    }
}

impl Ord for Instant {
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        self.inner.wrapping_sub(rhs.inner).cmp(&0)
    }
}

impl PartialOrd for Instant {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

#[derive(Clone, Copy)]
pub struct Duration {
    inner: u32,
}

impl Duration {
    pub fn from_cycles(cycles: u32) -> Self {
        Self { inner: cycles }
    }

    pub fn as_cycles(&self) -> u32 {
        self.inner
    }
}

impl TryInto<u32> for Duration {
    type Error = Infallible;

    fn try_into(self) -> Result<u32, Infallible> {
        Ok(self.inner)
    }
}
