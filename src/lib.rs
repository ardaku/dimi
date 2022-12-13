// Copyright © 2021-2022 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.
//
//! Asynchronous library for interfacing with MIDI devices.

#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg",
    html_root_url = "https://docs.rs/dimi"
)]
#![deny(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

pub use api::*;

// Platform-specific implementation
mod platform {
    #![allow(clippy::module_inception)]

    mod packet;
    mod platform;

    pub(crate) use packet::Midi;
    pub(crate) use platform::{connect, Device};
}

// Public root-level API.
mod api {
    mod connector;
    mod instrument;

    pub use connector::Connector;
    pub use instrument::Instrument;
}

// Public `midi` module API.
pub mod midi {
    //! Type-safe MIDI event types.

    mod control;
    mod event;
    mod message;
    mod note;

    pub use control::Control;
    pub use event::Event;
    pub use message::Message;
    pub use note::Note;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_send_sync<T: Send + Sync>() {}

    #[test]
    fn require_send_sync() {
        // Guarantee all exported `Notifiers` are `Send` + `Sync`
        test_send_sync::<Connector>();
        test_send_sync::<Instrument>();
    }
}
