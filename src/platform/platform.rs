// Copyright © 2021 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

#![allow(unsafe_code, unused_attributes)]

use lookit::It;

// Choose platform driver implementation.
#[cfg_attr(target_os = "linux", path = "../linux/linux.rs")]
#[path = "unsupported.rs"]
mod driver;

// Import the device type from the target platform.
pub(crate) use driver::Device;

// Single required method for each platform.
pub(crate) fn connect(it: It) -> Option<Device<crate::platform::packet::Midi>> {
    driver::connect(it)
}
