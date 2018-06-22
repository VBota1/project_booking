use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{NaiveDate, NaiveDateTime};

extern crate chrono;

pub trait AsDMY {
    fn as_dmy(&self) -> String;
}

impl AsDMY for NaiveDate {
    fn as_dmy(&self) -> String {
        self.format(dmy_format().as_str()).to_string()
    }
}

pub fn dmy_format() -> String {
    format!("%d.%m.%Y")
}

pub trait AsHHMMSS {
    fn as_hhmmss(&self) -> String;
}

impl AsHHMMSS for Duration {
    fn as_hhmmss(&self) -> String {
        let seconds = self.as_secs();
        let hh = format!("{:02.*}", 0, (seconds / 3600));
        let mm = format!("{:02.*}", 0, (seconds % 3600) / 60);
        let ss = format!("{:02.*}", 0, (seconds % 3600) % 60);
        format!("{}:{}:{}", hh, mm, ss)
    }
}

pub trait AsString {
    fn as_string(&self) -> String;
}

impl AsString for Result<String, String>
{
    fn as_string(&self) -> String
    {
        match self {
            Ok(message) => message.clone(),
            Err(message) => message.clone(),
        }
    }
}

pub trait TimeAsString {
    fn as_string(&self) -> String;
}

impl TimeAsString for SystemTime {
    fn as_string(&self) -> String {
        system_time_to_date_time(*self).to_string()
    }
}

fn system_time_to_date_time(t: SystemTime) -> NaiveDateTime {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };
    NaiveDateTime::from_timestamp(sec, nsec)
}
