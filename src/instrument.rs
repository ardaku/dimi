use std::io::{BufRead, BufReader, ErrorKind, Read, Result};

use pasts::prelude::*;
use smelling_salts::Device;

use crate::parse::Midi;

#[derive(Debug)]
pub(crate) struct Instrument(BufReader<Device>);

impl Instrument {
    pub(crate) fn new(device: Device) -> Self {
        Self(BufReader::new(device))
    }
}

impl Notify for Instrument {
    type Event = Midi;

    fn poll_next(mut self: Pin<&mut Self>, task: &mut Task<'_>) -> Poll<Midi> {
        while let Ready(()) = Pin::new(self.0.get_mut()).poll_next(task) {
            match midi(&mut self.0) {
                Ok(midi) => return Ready(midi),
                Err(e) if e.kind() == ErrorKind::WouldBlock => { /* ignore */ }
                Err(_) => return Ready(Midi([0xFF; 4])), // disconnect
            }
        }

        Pending
    }
}

fn midi(stream: &mut BufReader<Device>) -> Result<Midi> {
    let mut midi = Midi([0; 4]);
    let mut cmd = 0;

    while let Some(command) = stream.fill_buf()?.iter().next() {
        if command & 0x80 != 0 {
            cmd = *command;
            break;
        }
        stream.consume(1);
    }

    // Get the number of bytes.
    let bytes = match cmd {
        0x80..=0xCF => 3,
        0xD0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xFF => 1,
        _ => unreachable!(),
    };

    stream.read_exact(&mut midi.0[..bytes])?;

    Ok(midi)
}
