// Copyright © 2021 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use crate::midi::Event;
use crate::platform::{connect, Device, Midi};
use lookit::It;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Future that you can `.await` to get MIDI [`Event`](crate::midi::Event)s
#[derive(Debug)]
pub struct Instrument(Device<Midi>);

impl Instrument {
    pub(crate) fn new(which: It) -> Option<Self> {
        Some(Self(connect(which)?))
    }
}

impl Future for Instrument {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.get_mut().0).poll(cx).map(Event::from)
    }
}
