use std::time::Duration;

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

impl AsString for Vec<Vec<String>> {
    fn as_string(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        for task in self {
            lines.push(task.join(" "));
        }

        lines.join("\n")
    }
}