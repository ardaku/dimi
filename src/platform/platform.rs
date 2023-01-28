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
