mod instrument;

use smelling_salts::linux::Driver;
use std::sync::Once;
use std::mem::MaybeUninit;

pub(crate) use instrument::connect;
pub(crate) use smelling_salts::linux::Device;

struct Platform {
    driver: Driver,
}

impl Platform {
    fn new() -> Platform {
        let driver = Driver::new();
        Self { driver }
    }
}

fn platform() -> &'static Platform {
    static mut PLATFORM: MaybeUninit<Platform> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();
    unsafe {
        ONCE.call_once(|| PLATFORM = MaybeUninit::new(Platform::new()));
        &*PLATFORM.as_ptr()
    }
}
