use std::time::Duration;

pub trait AsHHMMSS {
    fn to_hhmmss(&self) -> String;
}

impl AsHHMMSS for Duration {
    fn to_hhmmss(&self) -> String {
        let seconds = self.as_secs();
        let hh = format!("{:02.*}", 0, (seconds / 3600));
        let mm = format!("{:02.*}", 0, (seconds % 3600) / 60);
        let ss = format!("{:02.*}", 0, (seconds % 3600) % 60);
        format!("{}:{}:{}", hh, mm, ss)
    }
}