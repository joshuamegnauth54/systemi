// SPDX-License-Identifier: BSD-2-Clause

use core::{
    ffi::CStr,
    fmt::{self, Write},
    hint::cold_path,
    mem::MaybeUninit,
    slice,
};

use rustix::{
    fd::BorrowedFd,
    io::{Errno, read},
};

use crate::error::{Action, Error};

// TODO: const_format_args
#[macro_export]
macro_rules! debug_buf_pos {
    ($pos:expr, $len:expr) => {{
        let pos: usize = $pos;
        let len: usize = $len;
        debug_assert!(
            pos <= len,
            "Buffer position is past the end of the slice: position = {}, length = {}",
            pos,
            len
        );
    }};
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReadRes {
    EOF(usize),
    Full,
}

/// Read until EOF or buf is full.
fn read_buf<'b>(
    fd: BorrowedFd<'_>,
    buf: &mut [MaybeUninit<u8>],
    msg: &'static str,
) -> Result<ReadRes, Error> {
    let mut nread = 0;

    loop {
        match read(fd, &mut buf[nread..]) {
            Ok((val, _)) => {
                nread += val.len();
                if val.len() == 0 {
                    return Ok(ReadRes::EOF(nread));
                } else if nread == buf.len() {
                    return Ok(ReadRes::Full);
                }
            }
            Err(Errno::INTR) => (),
            Err(err) => {
                cold_path();
                return Err(Error {
                    act: Action::Read,
                    err,
                    msg,
                });
            }
        }
    }
}

pub struct Buffer<'slice> {
    buf: &'slice mut [MaybeUninit<u8>],
    pos: usize,
}

impl<'slice> Buffer<'slice> {
    #[inline]
    pub const fn new(buf: &'slice mut [MaybeUninit<u8>]) -> Buffer<'slice> {
        Self { buf, pos: 0 }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        // TODO: const_format_args
        debug_buf_pos!(self.pos, self.buf.len());
        unsafe { slice::from_raw_parts(self.buf.as_ptr().cast(), self.pos) }
    }

    #[inline]
    pub fn spare_capacity(&mut self) -> &mut [MaybeUninit<u8>] {
        // TODO: const_format_args
        debug_buf_pos!(self.pos, self.buf.len());
        unsafe { self.buf.split_at_mut_unchecked(self.pos).1 }
    }

    /// Read from a file descriptor until EOF or buffer is full.
    pub fn read(&mut self, fd: BorrowedFd<'_>, msg: &'static str) -> Result<(), Error> {
        match read_buf(fd, &mut self.buf[self.pos..], msg)? {
            ReadRes::EOF(n) => {
                self.pos += n;
                debug_buf_pos!(self.pos, self.buf.len());
                Ok(())
            }
            ReadRes::Full => {
                self.pos = self.buf.len();
                Ok(())
            }
        }
    }

    /// Read until EOF but fail on full buffer.
    pub fn read_eof(&mut self, fd: BorrowedFd<'_>, msg: &'static str) -> Result<(), Error> {
        match read_buf(fd, &mut self.buf[self.pos..], msg)? {
            ReadRes::EOF(n) => {
                self.pos += n;
                debug_buf_pos!(self.pos, self.buf.len());
                Ok(())
            }
            ReadRes::Full => {
                self.pos = self.buf.len();
                Err(Error {
                    act: Action::Read,
                    err: Errno::NOSPC,
                    msg,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use core::{
        mem::MaybeUninit,
        ptr::{self, NonNull},
        slice,
    };

    use rustix::{
        fd::{AsFd, OwnedFd},
        io::{Errno, write},
        mm::{MapFlags, ProtFlags, mmap_anonymous, munmap},
        pipe::pipe,
    };

    use crate::{
        error::{Action, Error},
        utils::buffer::{Buffer, ReadRes, read_buf},
    };

    const TEXT: &[u8] = b"Hank Hill";
    const LONG_TEXT: &[u8] = b"That boy ain't right";
    const READ_ERR: &str = "Testing read";
    const READ_BUF_ERR: &str = "Testing read_buf";
    const READ_EOF_ERR: &str = "Testing read_eof";
    const INPUT_FD_ERR: &str = "Writing bytes to pipe to create input FD";
    const MAPPED_BUFFER_ERR: &str = "Memory mapping buffer";

    struct MappedBuffer {
        map: NonNull<MaybeUninit<u8>>,
        len: usize,
    }

    impl MappedBuffer {
        fn new(len: usize) -> Result<Self, Error> {
            let map = unsafe {
                mmap_anonymous(
                    ptr::null_mut(),
                    len,
                    ProtFlags::READ | ProtFlags::WRITE,
                    MapFlags::PRIVATE,
                )
            }
            .map_err(|err| Error {
                act: Action::Mmap,
                err,
                msg: MAPPED_BUFFER_ERR,
            })?;
            let ptr = NonNull::new(map.cast()).ok_or(Error {
                act: Action::Mmap,
                err: Errno::INVAL,
                msg: MAPPED_BUFFER_ERR,
            })?;

            Ok(Self { map: ptr, len })
        }

        const fn as_mut_slice(&mut self) -> &mut [MaybeUninit<u8>] {
            unsafe { slice::from_raw_parts_mut(self.map.as_ptr(), self.len) }
        }
    }

    impl Drop for MappedBuffer {
        fn drop(&mut self) {
            // Panicking in Drop is ill-advised, but this is test-only cleanup.
            let result =
                unsafe { munmap(self.map.as_ptr().cast(), self.len) }.map_err(|err| Error {
                    act: Action::Munmap,
                    err,
                    msg: MAPPED_BUFFER_ERR,
                });
            assert!(result.is_ok(), "munmap failed: {:?}", result.err());
        }
    }

    fn input_fd(input: &[u8]) -> Result<OwnedFd, Error> {
        let (read_fd, write_fd) = pipe().map_err(|err| Error {
            act: Action::Pipe,
            err,
            msg: INPUT_FD_ERR,
        })?;
        let mut written = 0;
        // TODO: Add a helper for this in super
        while written < input.len() {
            match write(&write_fd, &input[written..]) {
                Ok(n) => {
                    assert!(n > 0);
                    written += n;
                }
                Err(Errno::INTR) => (),
                Err(err) => {
                    return Err(Error {
                        act: Action::Write,
                        err,
                        msg: INPUT_FD_ERR,
                    });
                }
            }
        }

        Ok(read_fd)
    }

    fn assert_buffer(buffer: &mut Buffer<'_>, expected: &[u8]) {
        assert_eq!(buffer.as_slice(), expected);
        assert_eq!(
            buffer.spare_capacity().len(),
            buffer.buf.len() - expected.len()
        );
    }

    #[test]
    fn read_eof() -> Result<(), Error> {
        let fd = input_fd(TEXT)?;
        let mut storage = MappedBuffer::new(LONG_TEXT.len())?;
        let mut buffer = Buffer::new(storage.as_mut_slice());

        buffer.read(fd.as_fd(), READ_ERR)?;
        assert_buffer(&mut buffer, TEXT);
        Ok(())
    }

    #[test]
    fn read_full() -> Result<(), Error> {
        let fd = input_fd(LONG_TEXT)?;
        let mut storage = MappedBuffer::new(TEXT.len())?;
        let mut buffer = Buffer::new(storage.as_mut_slice());

        buffer.read(fd.as_fd(), READ_ERR)?;
        assert_buffer(&mut buffer, TEXT);
        Ok(())
    }

    #[test]
    fn read_buf_eof() -> Result<(), Error> {
        let fd = input_fd(TEXT)?;
        let mut storage = MappedBuffer::new(LONG_TEXT.len())?;
        let mut buffer = Buffer::new(storage.as_mut_slice());

        let result = read_buf(fd.as_fd(), buffer.spare_capacity(), READ_BUF_ERR)?;
        assert_eq!(result, ReadRes::EOF(TEXT.len()));
        buffer.pos = TEXT.len();
        assert_buffer(&mut buffer, TEXT);
        Ok(())
    }

    #[test]
    fn read_buf_full() -> Result<(), Error> {
        let fd = input_fd(LONG_TEXT)?;
        let mut storage = MappedBuffer::new(TEXT.len())?;
        let mut buffer = Buffer::new(storage.as_mut_slice());

        let result = read_buf(fd.as_fd(), buffer.spare_capacity(), READ_BUF_ERR)?;
        assert_eq!(result, ReadRes::Full);
        buffer.pos = TEXT.len();
        assert_buffer(&mut buffer, TEXT);
        Ok(())
    }

    #[test]
    fn read_eof_requires_eof() -> Result<(), Error> {
        let fd = input_fd(TEXT)?;
        let mut storage = MappedBuffer::new(LONG_TEXT.len())?;
        let mut buffer = Buffer::new(storage.as_mut_slice());

        buffer.read_eof(fd.as_fd(), READ_EOF_ERR)?;
        assert_buffer(&mut buffer, TEXT);
        Ok(())
    }

    #[test]
    fn read_eof_rejects_full() -> Result<(), Error> {
        let fd = input_fd(LONG_TEXT)?;
        let mut storage = MappedBuffer::new(TEXT.len())?;
        let mut buffer = Buffer::new(storage.as_mut_slice());

        let error = match buffer.read_eof(fd.as_fd(), READ_EOF_ERR) {
            Err(error) => error,
            Ok(()) => {
                return Err(Error {
                    act: Action::Sanity,
                    err: Errno::INVAL,
                    msg: "read_eof unexpectedly accepted a full buffer",
                });
            }
        };
        assert_eq!(error.err, Errno::NOSPC);
        assert_buffer(&mut buffer, TEXT);
        Ok(())
    }
}
