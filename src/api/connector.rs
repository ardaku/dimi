use lookit::Searcher;
use pasts::prelude::*;

use crate::Instrument;

/// [`Notify`] for when MIDI [`Instrument`](crate::Instrument)s are connected.
#[derive(Debug)]
pub struct Connector(Searcher);

impl Default for Connector {
    fn default() -> Self {
        Self::new()
    }
}

impl Connector {
    /// Create a new MIDI instrument connector
    pub fn new() -> Self {
        Self(Searcher::with_midi())
    }
}

impl Notify for Connector {
    type Event = Instrument;

    fn poll_next(
        mut self: Pin<&mut Self>,
        task: &mut Task<'_>,
    ) -> Poll<Self::Event> {
        while let Ready(inst) =
            Pin::new(&mut self.0).poll_next(task).map(Instrument::new)
        {
            let Some(inst) = inst else { continue };
            return Ready(inst);
        }

        Pending
    }
}
