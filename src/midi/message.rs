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
