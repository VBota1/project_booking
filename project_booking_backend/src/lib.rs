extern crate todo;
extern crate formaters;
extern crate logger;

use todo::*;
use logger::*;
use formaters::AsString;
use std::slice::Iter;
use std::time::Duration;
use formaters::AsHHMMSS;

//TODO MEDIUM PRIO new database at the start of each month
//TODO MEDIUM PRIO GUI (QT)
//TODO LOW PRIO import tasks from Jira
//TODO LOW PRIO export tasks to PTT

const NEW: &'static str = "new";
const CLOCKIN: &'static str = "clockIn";
const CLOCKOUT: &'static str = "clockOut";
const REPORT: &'static str = "report";
const REPORTBYLABEL: &'static str = "reportByLabel";
const ADDTIME: &'static str = "addTime";
const DELETE: &'static str = "delete";
const HELP: &'static str = "help";
const LICENSE: &'static str = "license";
pub const APPLICATIONMODE: &'static str = "applicationMode";
pub const EXIT: &'static str = "exit";

pub struct Response {
    pub message: String,
    pub should_save: bool,
}

pub fn handle_command_as_service(args: Vec<String>) -> String {
    let mut to_do: ToDo = get_to_do(None);
    let mut args = args.iter();
    args.next();
    let result = handle_command_as_application(args, &mut to_do);
    if true == result.should_save {
        match store(to_do) {
            Ok(_) => {},
            Err(store_error) => { return format!("{}. {}", result.message, store_error); }
        };
    }
    result.message
}

pub fn handle_command_as_application(mut args: Iter<String>, to_do: &mut ToDo) -> Response {
    match args.nth(0) {
        Some(command) => {
            match command.as_str() {
                NEW => {
                    trace(format!("New task request detected"));
                    Response { message: create_new_task_from_arguments(args, to_do).to_string(), should_save: true }
                },
                CLOCKIN => {
                    trace(format!("Clock in request detected"));
                    Response { message: clock_in(args, to_do).to_string(), should_save: true }
                },
                CLOCKOUT => {
                    trace(format!("Clock out request detected"));
                    Response { message: clock_out(args, to_do).to_string(), should_save: true }
                },
                REPORT => {
                    trace(format!("Report request detected"));
                    Response { message: report(to_do).to_string(), should_save: false }
                },
                REPORTBYLABEL => {
                    trace(format!("Report time spent on labels request detected"));
                    Response { message: report_time_on_labels(to_do).to_string(), should_save: false }
                },
                ADDTIME => {
                    trace(format!("Add time request detected"));
                    Response { message: add_time(args, to_do).to_string(), should_save: true }
                }
                DELETE => {
                    trace(format!("Delete request detected"));
                    Response { message: delete(args, to_do).to_string(), should_save: false }
                }
                HELP => {
                    trace(format!("Help request detected"));
                    Response { message: help(), should_save: false }
                },
                LICENSE => {
                    trace(format!("License request detected"));
                    Response { message: license(), should_save: false }
                },
                _ => {
                    Response { message: warn(format!("Unknown command \"{}\". {}", command, recommend_help())), should_save: false }
                },
            }
        },
        None => {
            Response { message: warn(format!("No command received. {}", recommend_help())), should_save: false }
        },
    }
}

fn help() -> String {
    format!("Version 016012000
Service mode usage: ./project_booking_cli command [task][labels][time]
Application mode usage: command [task][labels][time]
  Supported commands:
\t{0: <18} Creates a new task. Can be folowed by multiple labels.
\t{1: <18} Starts to monitor time for the indicated task. Must be followed by a task name.
\t{2: <18} Stops to monitor time for the indicated task and adds the duration between this event and clockIn event to the task. Must be followed by a task name.
\t{3: <18} Prints out a report of all the recorded tasks.
\t{4: <18} Prints out a report of all the duration spend on each label.
\t{5: <18} Add the specified time to the specified task. Must be followed by a task name. Must be followed by the time to add in the format hh:mm .
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
\t./project_booking_cli {5} task510 01:01
\t./project_booking_cli {6}
\t./project_booking_cli {7}
\t./project_booking_cli {8}
\t./project_booking_cli {9}
\t{10}
    ", NEW, CLOCKIN, CLOCKOUT, REPORT, REPORTBYLABEL, ADDTIME, DELETE, HELP, LICENSE, APPLICATIONMODE, EXIT)
}

fn delete(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    let task_name = match args.nth(0) {
        Some(task) => { task.clone() },
        None => {
            return Err(warn(format!("No task name received. \"{}\"", recommend_help())));
        },
    };

    to_do.remove_task(task_name).log_result()
}

fn report(to_do: &ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    Ok(to_do.to_report().as_string())
}

fn report_time_on_labels(to_do: &ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    Ok(to_do.report_time_spent_on_labels().as_string())
}

fn no_tasks_recorderd_message() -> String {
    format!("No tasks are recorded")
}

fn clock_out(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
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

fn clock_in(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
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

fn create_new_task_from_arguments(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
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

fn add_time(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
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

    let secs = (hours * 3600 + minutes * 60) as u64;
    let new_time = to_do.add_time_spent_to_task(task_name.clone(), Duration::new(secs, 0))?;
    Ok(trace(format!("Time spent on task \"{}\" is now \"{}\"", task_name, new_time.as_hhmmss())))
}

pub fn store(to_do: ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    forced_store(to_do)
}

fn forced_store(to_do: ToDo) -> Result<String, String> {
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

fn recommend_help() -> String {
    format!("Call \"project_booking help\" for additional information.")
}

pub trait ToString
{
    fn to_string(&self) -> String;
}

impl ToString for Result<String, String>
{
    fn to_string(&self) -> String
    {
        match self {
            Ok(message) => message.clone(),
            Err(message) => message.clone(),
        }
    }
}

fn license() -> String {
    format!("
MIT License \n
Copyright (c) 2018 V Bota \n
Permission is hereby granted, free of charge, to any person obtaining a copy \n
of this software and associated documentation files (the \"Software\"), to deal \n
in the Software without restriction, including without limitation the rights \n
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell \n
copies of the Software, and to permit persons to whom the Software is \n
furnished to do so, subject to the following conditions: \n
The above copyright notice and this permission notice shall be included in all \n
copies or substantial portions of the Software. \n
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR \n
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, \n
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE \n
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER \n
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, \n
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE \n
SOFTWARE. \n
\n
\n
3\'rd party LICENSES: \n
\n
LICENSE for extern crate simple_logging: \n
Copyright 2017 Isabela Schulze \n
Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met: \n
1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer. \n
2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution. \n
3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission. \n
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE. \n
\n
LICENSE for extern crates serde, serde_json and serde_derive: \n
Copyright (c) 2014 The Rust Project Developers \n
Permission is hereby granted, free of charge, to any \n
person obtaining a copy of this software and associated \n
documentation files (the \"Software\"), to deal in the \n
Software without restriction, including without \n
limitation the rights to use, copy, modify, merge, \n
publish, distribute, sublicense, and/or sell copies of \n
the Software, and to permit persons to whom the Software \n
is furnished to do so, subject to the following \n
conditions: \n
The above copyright notice and this permission notice \n
shall be included in all copies or substantial portions \n
of the Software. \n
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF \n
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED \n
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A \n
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT \n
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY \n
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION \n
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR \n
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER \n
DEALINGS IN THE SOFTWARE. \n
    ")
}

#[cfg(test)]
mod test_set;
