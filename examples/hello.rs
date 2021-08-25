use std::io::{Result, Read};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::convert::{TryInto, TryFrom};

/// A note and octave of the Western scale.
#[derive(Debug, Copy, Clone)]
enum Note {
    C(i8),
    Db(i8),
    D(i8),
    Eb(i8),
    E(i8),
    F(i8),
    Gb(i8),
    G(i8),
    Ab(i8),
    A(i8),
    Bb(i8),
    B(i8),
}

impl TryFrom<u8> for Note {
    type Error = std::num::TryFromIntError;

    fn try_from(note: u8) -> std::result::Result<Self, Self::Error> {
        let note = i8::try_from(note)?;
        let octave = note / 12;
        let note = match note % 12 {
            0 => Note::C,
            1 => Note::Db,
            2 => Note::D,
            3 => Note::Eb,
            4 => Note::E,
            5 => Note::F,
            6 => Note::Gb,
            7 => Note::G,
            8 => Note::Ab,
            9 => Note::A,
            10 => Note::Bb,
            11 => Note::B,
            _ => unreachable!(),
        };
        Ok(note(octave - 1))
    }
}

/// A decoded MIDI Event
#[derive(Debug, Clone, Copy)]
enum Event {
    /// Note stopped
    NoteOff {
        /// Channel 0-15
        channel: u8,
        /// Which note was stopped
        note: Note,
        /// Velocity of the note 0-127
        velocity: i8,
    },
    /// Note playing
    NoteOn {
        /// Channel 0-15
        channel: u8,
        /// Which note was played
        note: Note,
        /// Velocity of the note 0-127
        velocity: i8,
    },

}

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
        match other.0[0] & 0xF0 {
            0x80 => Event::NoteOff {
                channel: other.0[0] & 0x0F,
                note: Note::try_from(other.0[1]).unwrap(),
                velocity: other.0[2].try_into().unwrap(),
            },
            0x90 => Event::NoteOn {
                channel: other.0[0] & 0x0F,
                note: Note::try_from(other.0[1]).unwrap(),
                velocity: other.0[2].try_into().unwrap(),
            },
            0xA0 => panic!("FIXME: Aftertouch"),
            0xB0 => panic!("FIXME: Continous Controller"),
            0xC0 => panic!("FIXME: Patch Change"),
            0xD0 => panic!("FIXME: Channel Pressure"),
            0xE0 => panic!("FIXME: Pitch Bend"),
            0xF0 => panic!("FIXME: System Exclusive Message Start"),
            0xF1 => panic!("FIXME: MIDI Timecode Quarter Frame"),
            0xF2 => panic!("FIXME: Song Position Pointer"),
            0xF3 => panic!("FIXME: Song Select"),
            0xF6 => panic!("FIXME: Tune Request"),
            0xF7 => panic!("FIXME: System Exclusive Message End"),
            0xF8 => panic!("FIXME: Timing Clock"),
            0xFA => panic!("FIXME: Start"),
            0xFB => panic!("FIXME: Continue"),
            0xFC => panic!("FIXME: Stop"),
            0xFE => panic!("FIXME: Active Sensing"),
            0xFF => panic!("FIXME: System Reset"),
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
