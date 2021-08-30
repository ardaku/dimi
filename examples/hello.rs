use dimi::midi::Event;
use dimi::{Connector, Instrument};
use pasts::Loop;
use std::task::Poll::{self, Pending};

struct App {
    // MIDI instrument connector
    connector: Connector,
    // MIDI instruments
    instruments: Vec<Instrument>,
}

impl App {
    fn connect(&mut self, instrument: Instrument) -> Poll<()> {
        self.instruments.push(instrument);
        Pending
    }

    fn event(&mut self, which: usize, midi: Event) -> Poll<()> {
        dbg!(which, midi);
        Pending
    }

    async fn run() {
        let mut app = App {
            connector: Connector::new(),
            instruments: Vec::new(),
        };

        Loop::new(&mut app)
            .when(|a| &mut a.connector, Self::connect)
            .poll(|a| &mut a.instruments, Self::event)
            .await
    }
}

fn main() {
    pasts::block_on(App::run())
}
