// Set up the entrypoint and some exception handlers. Business logic should go in other files.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m_rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    embedded_rust_sandbox::run();

    #[allow(clippy::empty_loop)]
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

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
