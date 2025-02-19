#![no_std]

use driver::{GpioPort, GpioPortVariant, PinMode};

mod driver;

const LED_PIN: u8 = 5; // PA5
const BUTTON_PIN: u8 = 13; // PC13

pub fn run() {
    let mut gpioa = GpioPort::new(GpioPortVariant::A);
    let mut gpioc = GpioPort::new(GpioPortVariant::C);

    gpioa.set_pin_mode(LED_PIN, PinMode::Output);
    gpioc.set_pin_mode(BUTTON_PIN, PinMode::Input);

    loop {
        gpioa.pin_write(LED_PIN, !gpioc.pin_read(BUTTON_PIN));
    }
}
