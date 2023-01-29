use async_main::async_main;
use dimi::{midi::Event, Connector, Instrument};
use pasts::{prelude::*, Join};

struct App {
    // MIDI instrument connector
    connector: Connector,
    // MIDI instruments
    instruments: Vec<Instrument>,
}

impl App {
    fn connect(&mut self, instrument: Instrument) -> Poll {
        println!("{}: Connected", self.instruments.len());
        self.instruments.push(instrument);

        Pending
    }

    fn midi(&mut self, (inst, midi): (usize, Option<Event>)) -> Poll {
        if let Some(midi) = midi {
            println!("{inst}: {midi:?}");
        } else {
            self.instruments.swap_remove(inst);
            println!("{inst}: Disconnected");
        }

        Pending
    }
}

#[async_main]
async fn main(_spawner: impl Spawn) {
    let mut app = App {
        connector: Connector::new(),
        instruments: Vec::new(),
    };

    Join::new(&mut app)
        .on(|a| &mut a.connector, App::connect)
        .on(|a| &mut a.instruments[..], App::midi)
        .await
}
