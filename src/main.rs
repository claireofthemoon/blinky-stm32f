#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::{self as _, interrupt::Mutex};
use cortex_m_rt::{entry, exception};
use panic_halt as _;
use stm32f0::stm32f0x2::{
    self, Peripherals,
    gpioc::moder::MODE,
    tim3::{ccer::CC1E, ccmr1_output::OC1M},
};

static P: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

#[exception]
fn SysTick() {
    static mut STATE: u16 = 0;

    let north = tri(*STATE, 0);
    let east = tri(*STATE, 512);
    let south = tri(*STATE, 1024);
    let west = tri(*STATE, 1536);

    cortex_m::interrupt::free(|cs| {
        if let Some(p) = P.borrow(cs).borrow().as_ref() {
            // Set PWM values
            p.TIM3.ccr1().write(|w| w.ccr().set(north));
            p.TIM3.ccr4().write(|w| w.ccr().set(west));
            p.TIM3.ccr2().write(|w| w.ccr().set(south));
            p.TIM3.ccr3().write(|w| w.ccr().set(east));
        }
    });

    if *STATE == 2047 {
        *STATE = 0;
    } else {
        *STATE += 1;
    }
}

#[inline(always)]
fn tri(phase: u16, offset: u16) -> u16 {
    let p = (phase + offset) & 2047; // Modulo 2048
    if p < 1024 { p >> 2 } else { (2047 - p) >> 2 }
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f0x2::Peripherals::take().unwrap();

    // GPIO setup for LEDs
    p.RCC.ahbenr().modify(|_, w| w.iopcen().set_bit());
    p.GPIOC.moder().modify(|_, w| {
        w.moder6()
            .variant(MODE::Alternate)
            .moder8()
            .variant(MODE::Alternate)
            .moder9()
            .variant(MODE::Alternate)
            .moder7()
            .variant(MODE::Alternate)
    });

    // TIM3 setup
    p.RCC.apb1enr().modify(|_, w| w.tim3en().set_bit());
    // Prescaler is PSC + 1; we want 32 so set 31 here
    p.TIM3.psc().write(|w| w.psc().set(31));
    p.TIM3.arr().write(|w| w.arr().set(255));
    p.TIM3.ccmr1_output().modify(|_, w| {
        w.oc1m()
            .variant(OC1M::PwmMode1)
            .oc2m()
            .variant(OC1M::PwmMode1)
    });
    p.TIM3.ccmr2_output().modify(|_, w| {
        w.oc3m()
            .variant(OC1M::PwmMode1)
            .oc4m()
            .variant(OC1M::PwmMode1)
    });
    p.TIM3.ccer().modify(|_, w| {
        w.cc1e()
            .variant(CC1E::Enabled)
            .cc2e()
            .variant(CC1E::Enabled)
            .cc3e()
            .variant(CC1E::Enabled)
            .cc4e()
            .variant(CC1E::Enabled)
    });

    p.TIM3.cr1().modify(|_, w| w.cen().enabled());

    cortex_m::interrupt::free(|cs| P.borrow(cs).replace(Some(p)));

    // SysTick setup
    let mut syst = cp.SYST;
    // Set to around 1 ms (1000 ticks at 8 MHz / 8, which is SysTick prescaler default)
    syst.set_reload(1_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
    panic!()
}
