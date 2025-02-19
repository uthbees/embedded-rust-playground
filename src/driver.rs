use core::ptr::read_volatile;
use core::ptr::write_volatile;
use raw_gpio_port::RawGpioPort;

// Enclose the raw GPIO port struct in a module so that we can't
// accidentally use the struct fields directly.
mod raw_gpio_port {
    use super::GpioPortVariant;
    use core::ptr::{read_volatile, write_volatile};

    #[repr(C)]
    pub struct RawGpioPort {
        moder: u32,
        otyper: u32,
        ospeedr: u32,
        pupdr: u32,
        idr: u32,
        odr: u32,
    }

    impl RawGpioPort {
        pub fn new(variant: GpioPortVariant) -> &'static mut RawGpioPort {
            let address = match variant {
                GpioPortVariant::A => 0x4800_0000,
                GpioPortVariant::B => 0x4800_0400,
                GpioPortVariant::C => 0x4800_0800,
            };

            unsafe { &mut *(address as *mut RawGpioPort) }
        }

        pub fn moder(&self) -> u32 {
            unsafe { read_volatile(&raw const self.moder) }
        }

        pub fn write_moder(&mut self, val: u32) {
            unsafe { write_volatile(&raw mut self.moder, val) }
        }

        pub fn pupdr(&self) -> u32 {
            unsafe { read_volatile(&raw const self.pupdr) }
        }

        pub fn write_pupdr(&mut self, val: u32) {
            unsafe { write_volatile(&raw mut self.pupdr, val) }
        }

        pub fn idr(&self) -> u32 {
            unsafe { read_volatile(&raw const self.idr) }
        }

        pub fn odr(&self) -> u32 {
            unsafe { read_volatile(&raw const self.odr) }
        }

        pub fn write_odr(&mut self, val: u32) {
            unsafe { write_volatile(&raw mut self.odr, val) }
        }
    }
}

pub struct GpioPort {
    ptr: &'static mut RawGpioPort,
}

#[allow(dead_code)]
impl GpioPort {
    pub fn new(variant: GpioPortVariant) -> GpioPort {
        GpioPort::enable(variant);

        GpioPort {
            ptr: RawGpioPort::new(variant),
        }
    }

    fn enable(variant: GpioPortVariant) {
        const AHB2ENR: *mut u32 = 0x4002_104C as *mut u32;

        let bitmask = match variant {
            GpioPortVariant::A => 0b001,
            GpioPortVariant::B => 0b010,
            GpioPortVariant::C => 0b100,
        };

        unsafe {
            // Turn on the GPIO clock for the port.
            write_volatile(AHB2ENR, read_volatile(AHB2ENR) | bitmask);
        }
    }

    pub fn set_pin_mode(&mut self, pin: u8, mode: PinMode) {
        debug_assert!(pin < 16);

        self.ptr.write_moder(GpioPort::set_register_pin(
            self.ptr.moder(),
            pin,
            mode.get_binary_repr(),
        ));
    }

    pub fn set_pin_pull(&mut self, pin: u8, pull: PinPullStatus) {
        debug_assert!(pin < 16);

        self.ptr.write_pupdr(GpioPort::set_register_pin(
            self.ptr.pupdr(),
            pin,
            pull.get_binary_repr(),
        ));
    }

    pub fn pin_read(&self, pin: u8) -> bool {
        debug_assert!(pin < 16);

        self.ptr.idr() & (1 << pin) != 0
    }

    pub fn pin_write(&mut self, pin: u8, value: bool) {
        debug_assert!(pin < 16);

        self.ptr.write_odr(GpioPort::set_register_pin(
            self.ptr.odr(),
            pin,
            if value { PinValue::b1 } else { PinValue::b0 },
        ));
    }

    pub fn pin_toggle(&mut self, pin: u8) {
        debug_assert!(pin < 16);

        self.ptr.write_odr(self.ptr.odr() ^ (1 << pin));
    }

    /// Given the value of a register, a pin number, and a value for the pin, sets the specified
    /// pin to the specified value in the register value. The new register value is returned.
    ///
    /// Notes:
    /// - Does not actually update any registers - the variable passed to the `register_val` parameter
    ///   will need to be written to the register manually if you want the register to be updated.
    /// - Be sure to use the correct width of pin value (double width or single width).
    #[must_use]
    fn set_register_pin(register_val: u32, pin: u8, new_pin_val: PinValue) -> u32 {
        debug_assert!(pin < 16);

        match new_pin_val {
            PinValue::b0 => register_val & !(1 << pin),
            PinValue::b1 => register_val | (1 << pin),
            PinValue::b00 => register_val & !(0b11 << (pin * 2)),
            PinValue::b01 => register_val & !(0b10 << (pin * 2)) | (0b01 << (pin * 2)),
            PinValue::b10 => register_val & !(0b01 << (pin * 2)) | (0b10 << (pin * 2)),
            PinValue::b11 => register_val | (0b11 << (pin * 2)),
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum GpioPortVariant {
    A,
    B,
    C,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum PinMode {
    Input,
    Output,
    Alternate,
    Analog,
}
impl PinMode {
    fn get_binary_repr(self) -> PinValue {
        match self {
            PinMode::Input => PinValue::b00,
            PinMode::Output => PinValue::b01,
            PinMode::Alternate => PinValue::b10,
            PinMode::Analog => PinValue::b11,
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum PinPullStatus {
    None,
    PullUp,
    PullDown,
}
impl PinPullStatus {
    fn get_binary_repr(self) -> PinValue {
        match self {
            PinPullStatus::None => PinValue::b00,
            PinPullStatus::PullUp => PinValue::b01,
            PinPullStatus::PullDown => PinValue::b10,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum PinValue {
    b0,
    b1,
    b00,
    b01,
    b10,
    b11,
}
