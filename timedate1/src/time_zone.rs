// SPDX-License-Identifier: BSD-2-Clause

use core::{cell::OnceCell, ffi::CStr, hint::cold_path};

use rustix::{
    fd::{AsFd, BorrowedFd},
    fs::{Mode, OFlags, fstat},
    io::Errno,
};
use systemi_platform::{
    error::{Action, Error},
    read_buf, resolve_beneath,
};

const TZ_STR: usize = 40;
const TZ_AMT: usize = 450;

/// Flat buffer of time zone names as NUL terminated strings.
static TZ_BUF: OnceCell<[u8; TIME_ZONE_LEN * TZ_AMT]> = OnceCell::new();

/// All time zones as parsed from zone1970.tab.
pub static TZ_ALL: OnceCell<[TimeZone<'static>; TZ_AMT]> = OnceCell::new();

pub struct TimeZone<'t>(pub &'t CStr);

impl TimeZone<'_> {
    pub fn generate_time_zones(dfd: BorrowedFd<'_>) -> Result<(), Error> {
        let zfd = resolve_beneath(
            dfd,
            c"zone1970.tab",
            OFlags::RDONLY,
            Mode::empty(),
            "Opening zone1970.tab to read time zones",
        )?;

        // zone1970.tab is only like, 18K so this buffer has ample room.
        // TODO: No zero init
        let mut buf = [0u8; 32 * 1024];
        let buf = read_buf(
            zfd.as_fd(),
            &mut buf,
            "Reading /usr/share/zoneinfo/zone1970.tab",
        )?;

        let stat = fstat(zfd.as_fd()).map_err(|err| Error {
            act: Action::Stat,
            err,
            msg: "fstat zone1970.tab descriptor",
        })?;
        // This should never happen since zone1970 is stable.
        if buf.len() as i64 != stat.st_size {
            cold_path();
            return Err(Error {
                act: Action::Sanity,
                err: Errno::RANGE,
                msg: "zone1970.tab too large to read into stack buffer",
            });
        }
    }
}
