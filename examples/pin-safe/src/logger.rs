use log::debug;

use std::io::{Read, Write};

/// Throw away printer
/// Takes input and writes it to wherever only if you set the path to some
/// Like if condition for logging but its decided at runtime
pub struct Logger<T: Write, const sz: usize> {
    path: Option<T>,
    buf: [u8; sz],
}
impl<T: Write, const Sz: usize> Logger<T, Sz> {
    pub fn new() -> Self {
        Logger {
            path: None,
            buf: [0; Sz],
        }
    }
    // turn logger on or off
    pub fn update(&mut self, path: Option<T>) {
        self.path = path;
    }
    pub fn is_on(&self) -> bool {
        self.path.is_some()
    }
    pub fn from(path: T) -> Self {
        Logger {
            path: Some(path),
            buf: [0; Sz],
        }
    }
    // really a write all
    pub fn write<S: Read>(&mut self, source: &mut S) -> bool {
        // did we write
        match self.path.as_mut() {
            None => return false,
            Some(path) => {
                while let Ok(bw) = source.read(&mut self.buf) {
                    if bw == 0 {
                        break;
                    }
                    let check = path.write_all(&self.buf[..bw]);
                    debug!(target:"Logger","write={}",check.is_ok());
                }
            }
        }
        return true;
    }
    pub fn write_message<S: AsRef<[u8]>>(&mut self, source: S) -> bool {
        match self.path.as_mut() {
            None => false,
            Some(path) => path.write_all(source.as_ref()).is_ok(),
        }
    }
}
