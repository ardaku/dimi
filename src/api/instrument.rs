use lookit::Found;
use pasts::prelude::*;

use crate::{instrument::Instrument as Inst, midi::Event, parse::Midi};

/// [`Notify`] for when MIDI [`Event`](crate::midi::Event)s are generated.
#[derive(Debug)]
pub struct Instrument(Inst);

impl Instrument {
    pub(crate) fn new(found: Found) -> Option<Self> {
        let device = found.connect_input().ok()?;
        Some(Self(Inst::new(device)))
    }
}

impl Notify for Instrument {
    type Event = Option<Event>;

    fn poll_next(
        self: Pin<&mut Self>,
        task: &mut Task<'_>,
    ) -> Poll<Self::Event> {
        Pin::new(&mut self.get_mut().0)
            .poll_next(task)
            .map(Midi::into)
    }
}
