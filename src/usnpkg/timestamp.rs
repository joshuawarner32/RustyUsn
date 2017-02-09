use usnpkg::chrono;                                     //Datetime Handling
use usnpkg::byteorder::{ReadBytesExt, LittleEndian};    //Reading little endian data structs
use std::io::{Error};
use std::fmt;
use std::fmt::{Display,Debug};
use std::io::Read;

pub struct WinTimestamp(
    pub u64
);
impl WinTimestamp {
    pub fn to_datetime(&self) -> chrono::NaiveDateTime {
        // Get nanoseconds (100-nanosecond intervals)
        let t_micro = self.0 / 10;
        // Add microseconds to timestamp via Duration
        (
            chrono::NaiveDate::from_ymd(1601, 1, 1).and_hms_nano(0, 0, 0, 0) + // Win Epoc = 1601-01-01
            chrono::duration::Duration::microseconds(t_micro as i64)
        ) as chrono::NaiveDateTime
    }
}
impl Display for WinTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.to_datetime())
    }
}
impl Debug for WinTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.to_datetime())
    }
}

#[allow(dead_code)]
// For tests
pub fn raw_to_wintimestamp<R: Read>(mut buffer: R)->Result<WinTimestamp,Error>{
    let win_timestamp: WinTimestamp = WinTimestamp(
        buffer.read_u64::<LittleEndian>().unwrap()
    );
    Ok(win_timestamp)
}