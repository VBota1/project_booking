extern crate todo;
extern crate formaters;
extern crate logger;
extern crate chrono;
extern crate report;

use todo::*;
use logger::*;
use formaters::dmy_format;
use std::slice::Iter;
use std::time::Duration;
use formaters::AsHHMMSS;
use chrono::NaiveDate;
use report::*;

extern crate serde;
extern crate serde_json;

//TODO MEDIUM PRIO GUI (QT)
//TODO LOW PRIO import tasks from Jira
//TODO LOW PRIO export tasks to PTT

pub const NEW: &'static str = "new";
pub const CLOCKIN: &'static str = "clockIn";
pub const CLOCKOUT: &'static str = "clockOut";
pub const REPORT: &'static str = "report";
pub const REPORTBYLABEL: &'static str = "reportByLabel";
pub const REPORTFORMONTH: &'static str = "reportForMonth";
pub const ADDTIME: &'static str = "addTime";
pub const DELETE: &'static str = "delete";
pub const HELP: &'static str = "help";
pub const LICENSE: &'static str = "license";
pub const APPLICATIONMODE: &'static str = "applicationMode";
pub const EXIT: &'static str = "exit";

pub struct Response {
    pub message: String,
    pub should_save: bool,
}

pub fn help() -> String {
    format!("Version 016012000
Service mode usage: ./project_booking_cli command [task][labels][time]
Application mode usage: command [task][labels][time]
  Supported commands:
\t{0: <18} Creates a new task. Can be followed by multiple labels.
\t{1: <18} Starts to monitor time for the indicated task. Must be followed by a task name.
\t{2: <18} Stops to monitor time for the indicated task and adds the duration between this event and clockIn event to the task. Must be followed by a task name.
\t{3: <18} Prints out a report of all the recorded tasks.
\t{4: <18} Prints out a report of all the duration spend on each label.
\t{11: <18} Prints out a report of the time spent on each task of each day of the specified month. Must be followed by a month argument: example 05 for May.
\t{5: <18} Add the specified time to the specified task. Must be followed by a task name. Must be followed by the time to add in the format hh:mm . Can be followed by the date for which to add the time. The date must be in the format dd.mm.yyyy .
\t{6: <18} Removes the specified task from the recordings. Must be followed by a task name.
\t{7: <18} Prints out this help text.
\t{8: <18} Prints out License information.
\t{9: <18} Enters application mode. Changes made in application mode are saved to nonvolatile storage upon calling command {10}.
\t{10: <18} Exits application mode after saving all changes made to nonvolatile storage. Will return an error if called outside application mode.
  Examples:
\t./project_booking_cli {0} task510 Project1 Project2
\t./project_booking_cli {1} task510
\t./project_booking_cli {3}
\t./project_booking_cli {2} task510
\t./project_booking_cli {4}
\t./project_booking_cli {11} 5
\t./project_booking_cli {5} task510 01:01
\t./project_booking_cli {5} task510 01:01 31.05.2021
\t./project_booking_cli {6} task510
\t./project_booking_cli {7}
\t./project_booking_cli {8}
\t./project_booking_cli {9}
\t{10}
    ", NEW, CLOCKIN, CLOCKOUT, REPORT, REPORTBYLABEL, ADDTIME, DELETE, HELP, LICENSE, APPLICATIONMODE, EXIT, REPORTFORMONTH)
}

pub fn delete(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    let task_name = match args.nth(0) {
        Some(task) => { task.clone() },
        None => {
            return Err(warn(format!("No task name received. \"{}\"", recommend_help())));
        },
    };

    to_do.remove_task(task_name).log_result()
}

pub fn report(to_do: &ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    match serde_json::to_string(&to_do.to_complete_report()) {
        Ok(data_as_string) => {
            Ok(data_as_string)
        },
        Err(_) => {
            Err(error(format!("Data could not be serialized")))
        }
    }
}

pub fn report_time_on_labels(to_do: &ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    match serde_json::to_string(&to_do.to_time_on_labels_report()) {
        Ok(data_as_string) => {
            Ok(data_as_string)
        },
        Err(_) => {
            Err(error(format!("Data could not be serialized")))
        }
    }
}

pub fn daily_activity_report(mut args: Iter<String>, to_do: &ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    let month = match args.nth(0) {
        Some(value) => {
            match value.parse::<u32>() {
                Ok(value) => { value },
                Err(_) => {
                    return Err(warn(format!("Month argument \"{}\" could not be parsed as u32", value)));
                }
            }
        },
        None => {
            return Err(warn(format!("Month argument is missing. {}", recommend_help())));
        }
    };

    match serde_json::to_string(&to_do.to_month_jurnal_report(month)) {
        Ok(data_as_string) => {
            Ok(data_as_string)
        },
        Err(_) => {
            Err(error(format!("Data could not be serialized")))
        }
    }
}

fn no_tasks_recorderd_message() -> String {
    format!("No tasks are recorded")
}

pub fn clock_out(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    match args.nth(0) {
        Some(task_name) => {
            to_do.clock_out(task_name.to_string()).log_result()
        },
        None => {
            Err(warn(format!("Please supply a valid task name for the clock out operation. {}", recommend_help())))
        },
    }
}

pub fn clock_in(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    match args.nth(0) {
        Some(task_name) => {
            to_do.clock_in(task_name.to_string()).log_result()
        },
        None => {
            Err(warn(format!("Please supply a valid task name for the clock in operation. {}", recommend_help())))
        },
    }
}

pub fn create_new_task_from_arguments(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    match args.nth(0) {
        Some(task) => {
            let labels: Vec<String> = args.cloned().collect();
            to_do.add(task.clone(), labels).log_result()
        },
        None => {
            Err(warn(format!("No task name received. \"{}\"", recommend_help())))
        },
    }
}

pub fn add_time(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    let task_name = match args.nth(0) {
        Some(task) => { task },
        None => {
            return Err(warn(format!("No task name received. \"{}\"", recommend_help())));
        },
    };

    let (hours, minutes) = match args.nth(0) {
        Some(time_argument) => {
            let time: Vec<String> = time_argument.split(':').map(|s| format!("{}", s)).collect();
            let error_message = format!("Time to be added to the task, \"{}\" ,is not in the expected format. \"{}\"", time_argument, recommend_help());

            let mut h = match time.get(0) {
                Some(h) => { h },
                None => { return Err(warn(error_message)); }
            };
            let mut min = match time.get(1) {
                Some(min) => { min },
                None => { return Err(warn(error_message)); }
            };

            let h = match h.parse::<u32>() {
                Ok(value) => { value },
                Err(_) => { return Err(warn(error_message)); }
            };

            let min = match min.parse::<u32>() {
                Ok(value) => { value },
                Err(_) => { return Err(warn(error_message)); }
            };

            (h, min)
        },
        None => {
            return Err(warn(format!("No time to be added to the task was received. \"{}\"", recommend_help())));
        }
    };

    let date: Option<NaiveDate> = match args.nth(0) {
        Some(date_argument) => {
            match NaiveDate::parse_from_str(date_argument, dmy_format().as_str()) {
                Ok(result) => { Some(result) },
                Err(_) => {
                    return Err(warn(format!("Date for which to add time, \"{}\" ,is not in the expected format. \"{}\"", date_argument, recommend_help())));
                }
            }
        }
        None => {
            warn(format!("No date for which to add time was received. Current date will be used"));
            None
        }
    };

    let secs = (hours * 3600 + minutes * 60) as u64;
    let new_time = to_do.add_time_spent_to_task(task_name.clone(), date, Duration::new(secs, 0))?;
    Ok(trace(format!("Time spent on task \"{}\" is now \"{}\"", task_name, new_time.as_hhmmss())))
}

pub fn store(to_do: ToDo) -> Result<String, String> {
    forced_store(to_do)
}

pub fn forced_store(to_do: ToDo) -> Result<String, String> {
    to_do.save(None).log_result()
}

pub fn get_to_do(load_file: Option<String>) -> ToDo {
    match load(load_file) {
        Ok(todo) => {
            trace(format!("ToDo loaded from database"));
            todo
        },
        Err(error) => {
            warn(error.to_string());
            ToDo::new()
        },
    }
}

pub fn recommend_help() -> String {
    format!("Call \"project_booking help\" for additional information.")
}

pub fn license() -> String {
    format!("
The complete source code is stored at: https://github.com/VBota1/project_booking

the projects: project_booking_backend and project_booking_cli are provided under MIT License
Copyright (c) 2018 V Bota
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


3\'rd party LICENSES:

BSD 3 Clause LICENSE for extern crate simple_logging:
Copyright 2017 Isabela Schulze
Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

MIT LICENSE for extern crates serde, serde_json and serde_derive:
Copyright (c) 2014 The Rust Project Developers
Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the \"Software\"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:
The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

MIT LICENSE for extern crate chrono
The MIT License (MIT)
Copyright (c) 2014, Kang Seonghoon.
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
    ")
}

#[cfg(test)]
mod test_set;
