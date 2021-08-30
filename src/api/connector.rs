// Copyright © 2021 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use crate::Instrument;
use lookit::Lookit;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Future that you can `.await` to connect to MIDI
/// [`Instrument`](crate::Instrument)s
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

impl Future for Connector {
    type Output = Instrument;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let a = Pin::new(&mut self.get_mut().0)
            .poll(cx)
            .map(Instrument::new);
        match a {
            Poll::Ready(Some(x)) => Poll::Ready(x),
            Poll::Ready(None) => Poll::Pending,
            Poll::Pending => Poll::Pending,
        }
    }
}
