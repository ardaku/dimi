// use std::io::{Result, Read};
use std::fs::File;
use std::io::{BufReader}; //, BufRead};

use dimi::midi::{Event};

//

fn main() {
    // Open file in read-only mode.
    let file = File::open("/dev/snd/midiC1D0").expect("No MIDI");
    // Buffer the reader.
    let mut reader = BufReader::new(file);

    // FIXME: Set non-blocking.

    // Read
    loop {
        // let midi = Midi::new(&mut reader);
        // dbg!(midi.map(Event::from));
    }
}
