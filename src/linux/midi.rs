// Copyright © 2021 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

impl Midi {
    fn new(stream: &mut BufReader<File>) -> Result<Self> {
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
}
