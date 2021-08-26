use std::io::{Result, Read};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::convert::{TryInto, TryFrom};
use std::num::NonZeroI8;

use inst::midi::{Event};

/// Encoded MIDI event (to be sent through flume channel).
///
/// If the 4th byte has bit 0x80 set, it's part of variable length data.
#[derive(Debug)]
#[repr(transparent)]
struct Midi([u8; 4]);

impl Midi {
    fn new(stream: &mut BufReader<File>) -> Result<Self> {
        let mut midi = Midi([0; 4]);
        let mut cmd = 0;

        while let Some(command) = stream.fill_buf()?.iter().next() {
            if command & 0x80 != 0 {
                cmd = *command;
                break;
            }
            stream.consume(1);
        };

        // Get the number of bytes.
        let bytes = match cmd {
            0x80..=0xCF => 3,
            0xD0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xFF => 1,
            _ => unreachable!(),
        };

        stream.read_exact(&mut midi.0[..bytes])?;
        
        Ok(midi)
    }
}

impl From<Midi> for Event {
    fn from(other: Midi) -> Event {
        let chan = other.0[0] & 0x0F;
        let id = other.0[1].try_into().unwrap();
        let note = Note::try_from(other.0[1]).unwrap();
        let value = other.0[2].try_into().unwrap();

        match other.0[0] & 0xF0 {
            0x80 => Event::NoteOff {
                chan,
                note,
                value,
            },
            0x90 => Event::NoteOn {
                chan,
                note,
                value,
            },
            0xA0 => Event::NoteTouch {
                chan,
                note,
                value,
            },
            0xB0 => Event::Control {
                chan,
                message: Control::new(id, value),
            },
            0xC0 => Event::Instrument {
                chan,
                patch: [id, value],
            },
            0xD0 => Event::Pressure {
                chan,
                value: id,
            },
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
                }
            },
            a => { panic!("FIXME: Unknown MIDI event {:X}", a) },
        }
    }
}

//

fn main() {
    // Open file in read-only mode.
    let file = File::open("/dev/snd/midiC1D0").expect("No MIDI");
    // Buffer the reader.
    let mut reader = BufReader::new(file);

    // FIXME: Set non-blocking.

    // Read
    loop {
        let midi = Midi::new(&mut reader);
        dbg!(midi.map(Event::from));
    }
}
