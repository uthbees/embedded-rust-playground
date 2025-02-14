pub fn enable_gpio_ports() {
    const AHB2ENR: *mut u32 = 0x4002_104C as *mut u32;

    unsafe {
        // Turn on GPIO clocks (Ports A, B, and C)
        *AHB2ENR |= 0b111;
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
    bsrr: u32,
    lckr: u32,
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

    // TODO: Write wrapper functions for reading from and writing to the pins
    pub fn moder(&mut self) -> &mut u32 {
        &mut self.ptr.moder
    }

    pub fn pupdr(&mut self) -> &mut u32 {
        &mut self.ptr.pupdr
    }

    pub fn idr(&self) -> u32 {
        self.ptr.idr
    }

    pub fn odr(&mut self) -> &mut u32 {
        &mut self.ptr.odr
    }
}
