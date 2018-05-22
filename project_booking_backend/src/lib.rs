extern crate todo;
use todo::*;
extern crate logger;
use logger::*;
use std::env::Args;

//TODO MEDIUM PRIO GUI (QT)
//TODO MEDIUM PRIO import tasks from Jira
//TODO MEDIUM PRIO export tasks to PTT
//TODO LOW PRIO detect AFK and stop recoding
//TODO LOW PRIO detect return on Keyboard and ask what task I am working on

pub fn handle_command(mut args: Args) -> Result<String, String> {

    match args.nth(1) {
        Some(command) => {
            match command.as_str() {
                "new" => {
                    trace(format!("New task request detected"));
                    create_new_task_from_arguments(args)
                },
                "clockIn" => {
                    trace(format!("Clock in request detected"));
                    clock_in(args)
                },
                "clockOut" => {
                    trace(format!("Clock out request detected"));
                    clock_out(args)
                },
                "report" => {
                    trace(format!("Clock out request detected"));
                    //TODO HIGH PRIO report time spent on one or all tasks
                    //TODO HIGH PRIO report time spent on one or all labels
                    Err(warn(format!("report command not implemented")))
                },
                "help" => {
                    trace(format!("Help request detected"));
                    //TODO HIGH PRIO return help information
                    Err(warn(format!("help command not implemented")))
                },
                _ => {
                    Err(warn(format!("Unkown command \"{}\". {}", command, recommend_help())))
                },
            }
        },
        None => {
            Err(warn(format!("No command received. {}", recommend_help())))
        },
    }
}

//TODO LOW PRIO optimize pass function as a parameter
fn clock_out(mut args: Args) -> Result<String, String> {
    let mut to_do: ToDo = get_to_do(None);

    if 0 == to_do.count() {
        let message = format!("No tasks are recorded");
        warn(message.clone());
        return Err(message)
    }

    match args.nth(0) {
        Some(task) => {
            match to_do.clock_out(task) {
                Ok(message) => {
                    trace(message.clone());
                    store(to_do);
                    Ok(message)
                },
                Err(message) => {
                    Err(error(message))
                }
            }
        },
        None => {
            Err(warn(format!("Please supply a valid task name for the clock out operation. {}", recommend_help())))
        },
    }
}

fn clock_in(mut args: Args) -> Result<String, String> {
    let mut to_do: ToDo = get_to_do(None);

    if 0 == to_do.count() {
        let message = format!("No tasks are recorded");
        warn(message.clone());
        return Err(message)
    }

    match args.nth(0) {
        Some(task) => {
            match to_do.clock_in(task) {
                Ok(message) => {
                    trace(message.clone());
                    store(to_do);
                    Ok(message)
                },
                Err(message) => {
                    Err(error(message))
                }
            }
        },
        None => {
            Err(warn(format!("Please supply a valid task name for the clock in operation. {}", recommend_help())))
        },
    }
}

fn create_new_task_from_arguments(mut args: Args) -> Result<String, String> {
    let mut to_do = get_to_do(None);

    match args.nth(0) {
        Some(task) => {
            let labels: Vec<String> = args.collect();
            match to_do.add(task.clone(), labels) {
                Ok(message) => {
                    trace(message.clone());
                    store(to_do);
                    Ok(message)
                },
                Err(message) => {
                    Err(error(message))
                },
            }
        },
        None => {
            Err(warn(format!("No task name received. \"{}\"", recommend_help())))
        },
    }
}

fn store(to_do: ToDo) {
    match to_do.save(None) {
        Ok(message) => { trace(message); }
        Err(message) => { error(message); }
    };
}

fn get_to_do(load_file: Option<String>) -> ToDo {
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
    format!("Call \"project_booking help\" for aditional information.")
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