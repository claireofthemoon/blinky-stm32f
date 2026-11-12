#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f0::{
    W,
    stm32f0x2::{
        self, Peripherals,
        gpioc::{
            bsrr::{BS_W, BSRRrs},
            moder::{MODE::Output, MODER_W, MODERrs},
        },
    },
};

#[entry]
fn main() -> ! {
    let p = stm32f0x2::Peripherals::take().unwrap();
    p.RCC.ahbenr().write(|w| w.iopcen().set_bit());
    LD3::output(&p);
    LD3::set(&p);

    LD4::output(&p);
    LD4::set(&p);

    LD5::output(&p);
    LD5::set(&p);

    LD6::output(&p);
    LD6::set(&p);

    panic!();
}

trait IOPin {
    fn moder(w: &mut W<MODERrs>) -> MODER_W<'_, MODERrs>;
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
}
