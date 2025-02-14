#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;

use rt::entry;
use rt::exception;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let x = 1;
    let y = 2;
    let z = 3;

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
