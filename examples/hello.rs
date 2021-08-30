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
        println!("{}: Connected", self.instruments.len());
        self.instruments.push(instrument);
        Pending
    }

    fn event(&mut self, which: usize, midi: Event) -> Poll<()> {
        if midi == Event::Disconnect {
            self.instruments.swap_remove(which);
            println!("{}: Disconnected", which);
        } else {
            println!("{}: {:?}", which, midi);
        }
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
