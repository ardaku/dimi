/// A note and octave of the Western scale.
///
///  In this library, C4 is middle C and octaves range from -1 to 9
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Note {
    /// The C Note in the Western Scale
    C(i8),
    /// The D-Flat Note in the Western Scale
    Db(i8),
    /// The D Note in the Western Scale
    D(i8),
    /// The E-Flat Note in the Western Scale
    Eb(i8),
    /// The E Note in the Western Scale
    E(i8),
    /// The F Note in the Western Scale
    F(i8),
    /// The G-Flat Note in the Western Scale
    Gb(i8),
    /// The G Note in the Western Scale
    G(i8),
    /// The A-Flat Note in the Western Scale
    Ab(i8),
    /// The A Note in the Western Scale
    A(i8),
    /// The B-Flat Note in the Western Scale
    Bb(i8),
    /// The B Note in the Western Scale
    B(i8),
}

impl TryFrom<u8> for Note {
    type Error = std::num::TryFromIntError;

    fn try_from(note: u8) -> Result<Self, Self::Error> {
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
