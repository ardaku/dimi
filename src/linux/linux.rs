// Copyright © 2021-2022 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

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
