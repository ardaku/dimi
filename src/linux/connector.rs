// Copyright © 2021 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

use crate::platform::packet::Midi;
use std::os::unix::io::RawFd;
use std::os::raw::{c_int, c_char, c_uint, c_void};
use std::ffi::CString;
use std::mem::{self, MaybeUninit};
use flume::Sender;
use smelling_salts::linux::{Device, Watcher};

/// Device node paths in which to check for devices.  Preferred listed first.
const INOTIFY_PATH: &[&str] = &[
    "/dev/snd/",
    "/dev/",
];

////////////////////////////////////////////////////////////////////////////////
//////////////////////////// FIXME: Turn Into Crate ////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
struct InotifyEv {
    // struct inotify_event, from C.
    wd: c_int, /* Watch descriptor */
    mask: u32, /* Mask describing event */
    cookie: u32, /* Unique cookie associating related
               events (for rename(2)) */
    len: u32,        /* Size of name field */
    name: [u8; 256], /* Optional null-terminated name */
}

extern "C" {
    fn inotify_init1(flags: c_int) -> RawFd;
    fn inotify_add_watch(fd: RawFd, path: *const c_char, mask: u32) -> c_int;
    fn read(fd: RawFd, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: RawFd) -> c_int;
    fn strlen(s: *const u8) -> usize;
}

struct Connector {
    path: &'static str,
    listen: RawFd,
    sender: Sender<Device<Midi>>,
}

impl Connector {
    unsafe fn callback(&mut self) -> Option<()> {
        let mut ev = MaybeUninit::<InotifyEv>::zeroed();
        if read(
                self.listen,
                ev.as_mut_ptr().cast(),
                mem::size_of::<InotifyEv>(),
            ) > 0 {
            let ev = ev.assume_init();
            let len = strlen(&ev.name[0]);
            let filename = String::from_utf8_lossy(&ev.name[..len]);
            let path = format!("{}{}", self.path, filename);
            dbg!(path);
        }

        /*if self.sender.send().is_err() {
            super::platform().driver.discard(self.listen);
            let ret = close(self.listen);
            assert_eq!(0, ret);
            return None;
        }*/
        Some(())
    }
}

unsafe fn add_watch(inotify_fd: RawFd, dir: &str) -> Result<(), ()> {
    const ATTRIB: c_uint = 0x00000004;

    let dir = CString::new(dir).unwrap();
    if inotify_add_watch(inotify_fd, dir.into_raw(), ATTRIB) == -1 {
        Err(())
    } else {
        Ok(())
    }
}

pub(crate) fn connector() -> Device<Device<Midi>> {
    const CLOEXEC: c_int = 0o2000000;
    const NONBLOCK: c_int = 0o0004000;

    let platform = super::platform();

    let listen = unsafe { inotify_init1(NONBLOCK | CLOEXEC) };
    assert_ne!(-1, listen); // The only way this fails is some kind of OOM

    let mut paths = INOTIFY_PATH.iter();
    let mut path = paths.next().unwrap();
    while let Err(()) = unsafe { add_watch(listen, path) } {
        path = paths.next().unwrap();
    }

    platform.driver.device(|sender| Connector { sender, listen, path }, listen, Connector::callback, Watcher::new().input())
}
