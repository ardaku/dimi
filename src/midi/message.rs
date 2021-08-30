// Copyright © 2021 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.
//
//! Type-safe MIDI event types.

/// MIDI System Message
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// Start System Exclusive Message
    ExStart,
    /// MIDI Time Code quarter frame
    TimeCode,
    /// Song position pointer
    SongPosition,
    /// Song selection
    SongSelect,
    /// Tune Request
    TuneRequest,
    /// End System Exclusive Message
    ExEnd,
    /// Timing Clock
    TimingClock,
    /// Start
    Start,
    /// Continue
    Continue,
    /// Stop
    Stop,
    /// Active Sensing
    ActiveSensing,
    /// Reset System
    SystemReset,
    /// Unknown System Message
    Unknown(u8),
}
