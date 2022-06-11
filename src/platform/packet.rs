// Copyright © 2021-2022 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use std::convert::{TryFrom, TryInto};

use crate::midi::{Control, Event, Message, Note};

/// Encoded MIDI event (to be sent through flume channel).
///
/// If the 4th byte has bit 0x80 set, it's part of variable length data.
#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct Midi(pub(crate) [u8; 4]);

impl From<Midi> for Option<Event> {
    fn from(other: Midi) -> Self {
        if other.0 == [0xFF; 4] {
            return None;
        }

        let chan = other.0[0] & 0x0F;
        let id = other.0[1].try_into().unwrap();
        let note = Note::try_from(other.0[1]).unwrap();
        let value = other.0[2].try_into().unwrap();

        let event = match other.0[0] & 0xF0 {
            0x80 => Event::NoteOff { chan, note, value },
            0x90 => Event::NoteOn { chan, note, value },
            0xA0 => Event::NoteTouch { chan, note, value },
            0xB0 => Event::Control {
                chan,
                message: Control::new(id, value),
            },
            0xC0 => Event::Instrument {
                chan,
                patch: [id, value],
            },
            0xD0 => Event::Pressure { chan, value: id },
            0xE0 => Event::Bend {
                chan,
                lsb: id,
                msb: value,
            },
            0xF0 => Event::System {
                message: match chan {
                    0x0 => Message::ExStart,
                    0x1 => Message::TimeCode,
                    0x2 => Message::SongPosition,
                    0x3 => Message::SongSelect,
                    // 0x4..=0x5 unknown
                    0x6 => Message::TuneRequest,
                    0x7 => Message::ExEnd,
                    0x8 => Message::TimingClock,
                    // 0x9 unknown
                    0xA => Message::Start,
                    0xB => Message::Continue,
                    0xC => Message::Stop,
                    // 0xD unknown
                    0xE => Message::ActiveSensing,
                    0xF => Message::SystemReset,
                    _ => Message::Unknown(other.0[0]),
                },
            },
            a => {
                panic!("FIXME: Unknown MIDI event {:X}", a)
            }
        };

        Some(event)
    }
}
