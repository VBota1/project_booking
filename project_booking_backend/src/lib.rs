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
                "new" => {
                    trace(format!("New task request detected"));
                    Response { message: create_new_task_from_arguments(args, to_do).to_string(), should_save: true }
                },
                "clockIn" => {
                    trace(format!("Clock in request detected"));
                    Response { message: clock_in(args, to_do).to_string(), should_save: true }
                },
                "clockOut" => {
                    trace(format!("Clock out request detected"));
                    Response { message: clock_out(args, to_do).to_string(), should_save: true }
                },
                "report" => {
                    trace(format!("Report request detected"));
                    Response { message: report(to_do).to_string(), should_save: false }
                },
                "reportByLabel" => {
                    trace(format!("Report time spent on labels request detected"));
                    Response { message: report_time_on_labels(to_do).to_string(), should_save: false }
                },
                "addTime" => {
                    trace(format!("Add time request detected"));
                    Response { message: add_time(args, to_do).to_string(), should_save: true }
                }
                "remove" => {
                    trace(format!("Remove request detected"));
                    //TODO MEDIUM PRIO remove task
                    Response { message: warn(format!("remove command not implemented")), should_save: false }
                }
                "help" => {
                    trace(format!("Help request detected"));
                    //TODO HIGH PRIO return help information
                    Response { message: warn(format!("help command not implemented")), should_save: false }
                },
                "license" => {
                    trace(format!("License request detected"));
                    //TODO HIGH PRIO return license information
                    Response { message: warn(format!("license command not implemented")), should_save: false }
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
            match to_do.clock_out(task_name.to_string()) {
                Ok(message) => { Ok(trace(message)) },
                Err(message) => { Err(error(message)) }
            }
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
            match to_do.clock_in(task_name.to_string()) {
                Ok(message) => { Ok(trace(message)) },
                Err(message) => { Err(error(message)) }
            }
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
            match to_do.add(task.clone(), labels) {
                Ok(message) => { Ok(trace(message)) },
                Err(message) => { Err(error(message)) },
            }
        },
        None => {
            Err(warn(format!("No task name received. \"{}\"", recommend_help())))
        },
    }
}

fn add_time(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
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
    match to_do.save(None) {
        Ok(message) => { Ok(trace(message)) }
        Err(message) => { Err(error(message)) }
    }
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

#[cfg(test)]
mod test_set;
