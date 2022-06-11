use dimi::{midi::Event, Connector, Instrument};
use pasts::{prelude::*, Join};

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

    fn event(&mut self, (which, midi): (usize, Event)) -> Poll<()> {
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

        Join::new(&mut app)
            .on(|a| &mut a.connector, Self::connect)
            .on(|a| a.instruments.as_mut_slice(), Self::event)
            .await
    }
}

fn main() {
    pasts::Executor::default().spawn(Box::pin(App::run()))
}
