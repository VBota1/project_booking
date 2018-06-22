extern crate todo;
extern crate task;
extern crate formaters;
extern crate chrono;

use todo::ToDo;
use task::Task;
use formaters::{AsHHMMSS, TimeAsString, dmy_format};
use std::collections::HashMap;
use std::time::Duration;
use chrono::{NaiveDate, Datelike};

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


#[derive(Serialize, Deserialize, Debug)]
pub struct DayReport {
    date: String,
    tasks: Vec<TaskReport>,
}

pub trait ToDoReportMothJurnal {
    fn to_month_jurnal_report(&self, month_to_report: u32) -> Vec<DayReport>;
    fn to_daily_activity_report(&self) -> HashMap<String, Vec<TaskReport>>;
}

impl ToDoReportMothJurnal for ToDo {
    fn to_month_jurnal_report(&self, month_to_report: u32) -> Vec<DayReport> {
        let mut report = self.to_daily_activity_report();
        report.retain(|date, _|
            match NaiveDate::parse_from_str(date, dmy_format().as_str()) {
                Ok(date) => { date.month() == month_to_report }
                Err(_) => { false }
            }
        );
        let mut report: Vec<DayReport> = report.iter().map(|(day, task_vector)| DayReport { date: day.to_string(), tasks: task_vector.to_vec() }).collect();
        report.sort_by(|a, b| a.date.cmp(&b.date));
        report
    }

    fn to_daily_activity_report(&self) -> HashMap<String, Vec<TaskReport>> {
        let mut task_durations_on_day: HashMap<String, Vec<TaskReport>> = HashMap::new();

        let tasklist = self.list.as_slice();
        for t in tasklist {
            let time_records = t.time_spent();
            for (date, _) in time_records {
                let task_info = task_durations_on_day.entry(date).or_insert(Vec::new());
                task_info.push(t.complete_report());
            }
        }

        task_durations_on_day
    }
}

impl std::clone::Clone for TaskReport {
    fn clone(&self) -> TaskReport {
        TaskReport { id: self.id.to_string(), name: self.name.to_string(), time_spent: self.time_spent.to_string(), labels: self.labels.clone(), clock_in_timestamp: self.clock_in_timestamp.to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelReport {
    pub label: String,
    pub time: String,
}

pub trait ToDoReportTimeOnLabels {
    fn to_time_on_labels_report(&self) -> Vec<LabelReport>;
}

impl ToDoReportTimeOnLabels for ToDo {
    fn to_time_on_labels_report(&self) -> Vec<LabelReport> {
        let mut time_on_labels = HashMap::new();

        let tasklist = self.list.as_slice();
        for t in tasklist {
            let time_per_label = t.total_time_spent().checked_div(t.labels().len() as u32).unwrap_or(t.total_time_spent());

            let labels = t.labels();
            for l in labels {
                let value = time_on_labels.entry(l).or_insert(Duration::new(0, 0));
                *value += time_per_label;
            }
        }

        time_on_labels.iter().map(|(label_name, duration)| LabelReport { label: label_name.to_string(), time: duration.as_hhmmss() }).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskReport {
    id: String,
    name: String,
    time_spent: String,
    labels: Vec<String>,
    clock_in_timestamp: String,
}

pub trait ToDoCompleteReport {
    fn to_complete_report(&self) -> Vec<TaskReport>;
}

impl ToDoCompleteReport for ToDo {
    fn to_complete_report(&self) -> Vec<TaskReport> {
        let mut output: Vec<TaskReport> = Vec::new();
        let tasklist = self.list.as_slice();
        for t in tasklist {
            output.push(t.complete_report());
        }
        output
    }
}

trait TaskCompleteReport {
    fn complete_report(&self) -> TaskReport;
}

impl TaskCompleteReport for Task {
    fn complete_report(&self) -> TaskReport {
        let output_id = format!("{}", self.id());

        let output_name = format!("{}", self.name());

        let output_time_spent = format!("{}", self.total_time_spent().as_hhmmss());


        let output_clock_in_timestamp = match self.clock_in_timestamp() {
            Some(timestamp) => {
                format!("{}", timestamp.as_string())
            }
            None => {
                format!("None")
            }
        };

        let mut output_labels: Vec<String> = Vec::new();
        if self.labels().len() <= 0 {
            output_labels.push(format!("None"));
        } else {
            let labels = self.labels().clone();
            for l in labels {
                output_labels.push(format!("{}", l));
            }
        }

        TaskReport { id: output_id, name: output_name, time_spent: output_time_spent, labels: output_labels, clock_in_timestamp: output_clock_in_timestamp }
    }
}