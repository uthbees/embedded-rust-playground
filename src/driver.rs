use core::ptr::read_volatile;
use core::ptr::write_volatile;

pub fn enable_gpio_ports() {
    const AHB2ENR: *mut u32 = 0x4002_104C as *mut u32;

    unsafe {
        // Turn on GPIO clocks (Ports A, B, and C)
        write_volatile(AHB2ENR, read_volatile(AHB2ENR) | 0b111);
    }
}

#[repr(C)]
struct RawGpioPort {
    moder: u32,
    otyper: u32,
    ospeedr: u32,
    pupdr: u32,
    idr: u32,
    odr: u32,
}

impl RawGpioPort {
    fn new(variant: GpioPortVariant) -> &'static mut RawGpioPort {
        let address = match variant {
            GpioPortVariant::A => 0x4800_0000,
            GpioPortVariant::B => 0x4800_0400,
            GpioPortVariant::C => 0x4800_0800,
        };

        unsafe { &mut *(address as *mut RawGpioPort) }
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

pub struct GpioPort {
    ptr: &'static mut RawGpioPort,
}

#[allow(dead_code)]
impl GpioPort {
    pub fn new(variant: GpioPortVariant) -> GpioPort {
        GpioPort {
            ptr: RawGpioPort::new(variant),
        }
    }

    pub fn set_pin_mode(&mut self, pin: u8, mode: PinMode) {
        assert!(pin < 16);

        let bitmask = match mode {
            PinMode::Input => 0b00,
            PinMode::Output => 0b01,
            PinMode::Alternate => 0b10,
            PinMode::Analog => 0b11,
        };

        let moder = &raw mut self.ptr.moder;

        unsafe {
            let mut val = read_volatile(moder);
            val &= !(0b11 << (pin * 2));
            val |= bitmask << (pin * 2);
            write_volatile(moder, val);
        }
    }

    pub fn pin_read(&self, pin: u8) -> bool {
        assert!(pin < 16);

        let idr = &raw const self.ptr.idr;

        unsafe {
            let idr_val = read_volatile(idr);

            idr_val & (1 << pin) != 0
        }
    }

    pub fn pin_write(&mut self, pin: u8, value: bool) {
        assert!(pin < 16);

        let odr = &raw mut self.ptr.odr;

        unsafe {
            let mut odr_val = read_volatile(odr);

            if value {
                odr_val |= 1 << pin;
            } else {
                odr_val &= !(1 << pin);
            }

            write_volatile(odr, odr_val);
        }
    }
}
