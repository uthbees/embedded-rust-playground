#![no_std]

use crate::driver::{enable_gpio_ports, GpioPort, GpioPortVariant};

mod driver;

const BUTTON_PIN: i32 = 13; // PC13
const LED_PIN: i32 = 5; // PA5

pub fn run() {
    enable_gpio_ports();

    let mut gpioa = GpioPort::new(GpioPortVariant::A);
    let a_moder = gpioa.moder();

    // Configure the LED pin as an output.
    *a_moder &= !(0b11 << (LED_PIN * 2));
    *a_moder |= 0b01 << (LED_PIN * 2);

    // Configure the button pin as an input.
    *a_moder &= !(0b11 << (BUTTON_PIN * 2));

    let a_odr = gpioa.odr();
    *a_odr |= 1 << LED_PIN;
    // loop {
    //     let button_pressed = gpioa.idr() & (1 << BUTTON_PIN) == 0;
    //
    //     let a_odr = gpioa.odr();
    //     if button_pressed {
    //         // Turn the LED on.
    //         *a_odr |= 1 << LED_PIN;
    //     } else {
    //         // Turn the LED off.
    //         *a_odr &= !(1 << LED_PIN);
    //     }
    // }
}
