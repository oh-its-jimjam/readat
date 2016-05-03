#![feature(zero_one)]
extern crate libc;

use std::io;
use std::fs::{File};
use std::os::unix::io::AsRawFd;
use std::num::One;
use std::ops::Neg;
use libc::{size_t, c_void, off_t};

pub trait ReadAt {
    fn read_at(&self, buf: &mut [u8], off: usize) -> io::Result<usize>;
    fn read_exact_at(&self, buf: &mut [u8], off: usize) -> io::Result<()>;
}

fn cvt<T: One + PartialEq + Neg<Output = T>>(t: T) -> io::Result<T> {
    let one: T = T::one();
    if t == -one {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

#[cfg(unix)]
impl ReadAt for File {
    fn read_at(&self, buf: &mut [u8], off: usize) -> io::Result<usize> {
        let fd = self.as_raw_fd();
        let ret = try!(cvt(unsafe {
            libc::pread(fd,
                        buf.as_mut_ptr() as *mut c_void,
                        buf.len() as size_t,
                        off as off_t)
        }));
        return Ok(ret as usize);
    }

    fn read_exact_at(&self, mut buf: &mut [u8], off: usize) -> io::Result<()> {
        let mut pos = off; 
        while !buf.is_empty() {
            match self.read_at(buf, pos) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                    pos += n;
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {},
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{OpenOptions};
    use std::io::{Seek, SeekFrom, Write};
    use ReadAt;

    #[test]
    fn pos_unchanged() {
        // create file
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("testf")
            .unwrap();

        // write to file
        let input: Vec<u8> = vec![1; 32]; 
        let _ = f.write(&input).unwrap();

        // hold file pos to ensure it is not changed
        let pos = f.seek(SeekFrom::Current(0)).unwrap();

        // test read_at content
        let mut out: Vec<u8> = vec![0; 32];
        let n = f.read_at(&mut out, 0).unwrap();
        let exp: Vec<u8> = vec![1; n];
        assert_eq!(out, exp);

        // test read_at didn't affect file's seek pos
        let new_pos = f.seek(SeekFrom::Current(0)).unwrap();
        assert_eq!(pos, new_pos); 
    }
}
