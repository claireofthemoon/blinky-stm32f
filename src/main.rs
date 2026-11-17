#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::{self as _, interrupt::Mutex};
use cortex_m_rt::{entry, exception};
use panic_halt as _;
use stm32f0::{
    W,
    stm32f0x2::{
        self, Peripherals,
        gpioc::{
            bsrr::{BR_W, BS_W, BSRRrs},
            moder::{MODE::Output, MODER_W, MODERrs},
        },
    },
};

// LED pins are also TIM3_CH{1,2,3,4} (in GPIO pin order) (AF0)
// TIM3 is general purpose, 16-bit

static P: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

#[exception]
fn SysTick() {
    static mut STATE: bool = false;

    cortex_m::interrupt::free(|cs| {
        if let Some(p) = P.borrow(cs).borrow().as_ref() {
            if *STATE {
                LD3::clear(p);
                LD4::clear(p);
                LD5::clear(p);
                LD6::clear(p);
            } else {
                LD3::set(p);
                LD4::set(p);
                LD5::set(p);
                LD6::set(p);
            }

            *STATE = !*STATE;
        }
    });
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f0x2::Peripherals::take().unwrap();

    p.RCC.ahbenr().write(|w| w.iopcen().set_bit());
    LD3::output(&p);
    LD4::output(&p);
    LD5::output(&p);
    LD6::output(&p);
    cortex_m::interrupt::free(|cs| P.borrow(cs).replace(Some(p)));

    let mut syst = cp.SYST;
    syst.set_reload(1_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
    loop {}
}

trait IOPin {
    fn moder(w: &mut W<MODERrs>) -> MODER_W<'_, MODERrs>;
    fn br(w: &mut W<BSRRrs>) -> BR_W<'_, BSRRrs>;
    fn bs(w: &mut W<BSRRrs>) -> BS_W<'_, BSRRrs>;

    #[inline]
    fn output(p: &Peripherals) {
        p.GPIOC
            .moder()
            .modify(|_, w| Self::moder(w).variant(Output));
    }

    #[inline]
    fn set(p: &Peripherals) {
        p.GPIOC.bsrr().write(|w| Self::bs(w).set_bit());
    }

    #[inline]
    fn clear(p: &Peripherals) {
        p.GPIOC.bsrr().write(|w| Self::br(w).set_bit());
    }
}

struct LD3;

impl IOPin for LD3 {
    #[inline]
    fn moder(w: &mut W<MODERrs>) -> MODER_W<'_, MODERrs> {
        w.moder6()
    }

    #[inline]
    fn bs(w: &mut W<BSRRrs>) -> BS_W<'_, BSRRrs> {
        w.bs6()
    }

    #[inline]
    fn br(w: &mut W<BSRRrs>) -> BR_W<'_, BSRRrs> {
        w.br6()
    }
}

struct LD4;

impl IOPin for LD4 {
    #[inline]
    fn moder(w: &mut W<MODERrs>) -> MODER_W<'_, MODERrs> {
        w.moder8()
    }

    #[inline]
    fn bs(w: &mut W<BSRRrs>) -> BS_W<'_, BSRRrs> {
        w.bs8()
    }

    #[inline]
    fn br(w: &mut W<BSRRrs>) -> BR_W<'_, BSRRrs> {
        w.br8()
    }
}

struct LD5;

impl IOPin for LD5 {
    #[inline]
    fn moder(w: &mut W<MODERrs>) -> MODER_W<'_, MODERrs> {
        w.moder9()
    }

    #[inline]
    fn bs(w: &mut W<BSRRrs>) -> BS_W<'_, BSRRrs> {
        w.bs9()
    }

    #[inline]
    fn br(w: &mut W<BSRRrs>) -> BR_W<'_, BSRRrs> {
        w.br9()
    }
}

struct LD6;

impl IOPin for LD6 {
    #[inline]
    fn moder(w: &mut W<MODERrs>) -> MODER_W<'_, MODERrs> {
        w.moder7()
    }

    #[inline]
    fn bs(w: &mut W<BSRRrs>) -> BS_W<'_, BSRRrs> {
        w.bs7()
    }

    #[inline]
    fn br(w: &mut W<BSRRrs>) -> BR_W<'_, BSRRrs> {
        w.br7()
    }
}
