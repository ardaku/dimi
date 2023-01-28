use lookit::It;
use pasts::prelude::*;

use crate::{
    midi::Event,
    platform::{connect, Device, Midi},
};

/// [`Notifier`] for when MIDI [`Event`](crate::midi::Event)s are generated.
#[derive(Debug)]
pub struct Instrument(Device<Midi>);

impl Instrument {
    pub(crate) fn new(which: It) -> Option<Self> {
        Some(Self(connect(which)?))
    }
}

impl Notifier for Instrument {
    type Event = Option<Event>;

    fn poll_next(
        self: Pin<&mut Self>,
        exec: &mut Exec<'_>,
    ) -> Poll<Self::Event> {
        Pin::new(&mut self.get_mut().0).poll(exec).map(Midi::into)
    }
}
