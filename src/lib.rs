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

// For parsing raw MIDI stream bytes
mod parse;
// For parsing raw MIDI stream into byte chunks
mod instrument;

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

    fn test_send<T: Send>() {}

    #[test]
    fn require_send() {
        // Guarantee all exported `Notify`s are `Send`
        test_send::<Connector>();
        test_send::<Instrument>();
    }
}
