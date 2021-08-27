// use std::io::{Result, Read};
use std::fs::File;
use std::io::{BufReader}; //, BufRead};
use std::task::Poll::{self, Pending};

use dimi::midi::{Event};
use dimi::{Connector, Instrument};

use pasts::Loop;

//

struct App {
    // MIDI instrument connector
    connector: Connector,
}

impl App {
    fn connect(&mut self, instrument: Instrument) -> Poll<()> {
        Pending
    }

    async fn run() {
        let mut app = App {
            connector: Connector::new(),
        };

        Loop::new(&mut app)
            .when(|a| &mut a.connector, Self::connect)
            .await
    }
}

fn main() {
    pasts::block_on(App::run())

    /*// Open file in read-only mode.
    let file = File::open("/dev/snd/midiC1D0").expect("No MIDI");
    // Buffer the reader.
    let mut reader = BufReader::new(file);

    // FIXME: Set non-blocking.

    // Read
    loop {
        // let midi = Midi::new(&mut reader);
        // dbg!(midi.map(Event::from));
    }*/
}
