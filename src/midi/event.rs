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
//! Type-safe MIDI event types.

use crate::midi::{Control, Message, Note};

/// A decoded MIDI Event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// Note stopped
    NoteOff {
        /// Channel 0-15
        chan: u8,
        /// Which note was stopped
        note: Note,
        /// Velocity of the note 0-127
        value: i8,
    },
    /// Note playing
    NoteOn {
        /// Channel 0-15
        chan: u8,
        /// Which note was played
        note: Note,
        /// Velocity of the note 0-127
        value: i8,
    },
    /// Note aftertouch parameter change
    NoteTouch {
        /// Channel 0-15
        chan: u8,
        /// Which note was played
        note: Note,
        /// Touch parameter value 0-127.
        value: i8,
    },
    /// Control Change (Continous Controller)
    Control {
        /// Channel 0-15
        chan: u8,
        /// Which control change message.
        message: Control,
    },
    /// Patch Change
    Instrument {
        /// Channel 0-15
        chan: u8,
        /// Instrument Patch ID (`[0x00-0x7F, 0x00-0x7F]`)
        patch: [i8; 2],
    },
    /// Channel Pressure
    Pressure {
        /// Channel 0-15
        chan: u8,
        /// Pressure parameter value 0-127.
        value: i8,
    },
    /// Pitch-Bend
    Bend {
        /// Channel 0-15
        chan: u8,
        /// FIXME: what is LSB
        lsb: i8,
        /// FIXME: what is MSB
        msb: i8,
    },
    /// System Message
    System {
        /// One of the MIDI system messages.
        message: Message,
    },
}
