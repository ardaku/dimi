// Copyright © 2021 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

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
    type Event = Event;

    fn poll_next(
        self: Pin<&mut Self>,
        exec: &mut Exec<'_>,
    ) -> Poll<Self::Event> {
        Pin::new(&mut self.get_mut().0).poll(exec).map(Event::from)
    }
}
