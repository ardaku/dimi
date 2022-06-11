// Copyright © 2021-2022 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use lookit::Lookit;
use pasts::prelude::*;

use crate::Instrument;

/// [`Notifier`] for when MIDI [`Instrument`](crate::Instrument)s are connected.
#[derive(Debug)]
pub struct Connector(Lookit);

impl Default for Connector {
    fn default() -> Self {
        Self::new()
    }
}

impl Connector {
    /// Create a new MIDI instrument connector
    pub fn new() -> Self {
        Self(Lookit::with_midi())
    }
}

impl Notifier for Connector {
    type Event = Instrument;

    fn poll_next(
        mut self: Pin<&mut Self>,
        exec: &mut Exec<'_>,
    ) -> Poll<Self::Event> {
        let a = Pin::new(&mut self.as_mut().0)
            .poll(exec)
            .map(Instrument::new);
        match a {
            Ready(Some(x)) => Ready(x),
            Ready(None) => self.poll_next(exec),
            Pending => Pending,
        }
    }
}
