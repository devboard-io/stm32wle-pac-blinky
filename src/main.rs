#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32wl::stm32wle5 as pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC;
    rcc.ahb2enr.modify(|_, w| w.gpioaen().set_bit());

    let gpioa = dp.GPIOA;

    gpioa.moder.modify(|_,w| unsafe {
        w.moder4().bits(0b01)
    });

    gpioa.bsrr.write(|w| w.bs4().set_bit());

    loop {
        for _x in 0..10_000 {
            asm::nop();
        }
        gpioa.bsrr.write(|w| w.bs4().set_bit());

        for _x in 0..5_000 {
            asm::nop();
        }
        gpioa.bsrr.write(|w| w.br4().set_bit());
    }
}
