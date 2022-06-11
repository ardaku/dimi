// Copyright © 2021-2022 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use std::io::{BufReader, Result, Read, BufRead, ErrorKind};
use std::fs::File;
use crate::platform::Midi;
use flume::Sender;
use lookit::It;
use std::os::unix::io::AsRawFd;
use smelling_salts::linux::{Device, Watcher, RawDevice};

struct Instrument {
    listen: BufReader<File>,
    sender: Sender<Midi>,
    device: RawDevice,
}

pub(crate) fn connect(it: It) -> Option<Device<Midi>> {
    let file = it.file_open_r().ok()?;
    let device = file.as_raw_fd();
    let listen = BufReader::new(file);
    let constructor = |sender| Instrument { listen, sender, device };
    let watcher = Watcher::new().input();
    Some(super::platform().driver.device(constructor, device, callback, watcher))
}

unsafe fn callback(inst: &mut Instrument) -> Option<()> {
    let should_discard = match midi(&mut inst.listen) {
        Ok(midi) => inst.sender.send(midi).is_err(),
        Err(e) => e.kind() != ErrorKind::WouldBlock,
    };
    if should_discard {
        let _ = inst.sender.send(Midi([0xFF; 4]));
        super::platform().driver.discard(inst.device);
        drop(std::ptr::read(inst));
        return None;
    }
    Some(())
}

fn midi(stream: &mut BufReader<File>) -> Result<Midi> {
    let mut midi = Midi([0; 4]);
    let mut cmd = 0;

    while let Some(command) = stream.fill_buf()?.iter().next() {
        if command & 0x80 != 0 {
            cmd = *command;
            break;
        }
        stream.consume(1);
    };

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
