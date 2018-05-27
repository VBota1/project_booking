extern crate todo;
use todo::*;
extern crate logger;
use logger::*;
extern crate formaters;
use formaters::AsString;
use std::slice::Iter;

//TODO MEDIUM PRIO GUI (QT)
//TODO MEDIUM PRIO import tasks from Jira
//TODO MEDIUM PRIO export tasks to PTT
//TODO LOW PRIO detect AFK and stop recoding
//TODO LOW PRIO detect return on Keyboard and ask what task I am working on

pub fn handle_command(mut args: Iter<String>) -> Result<String, String> {

    match args.nth(1) {
        Some(command) => {
            match command.as_str() {
                "new" => {
                    trace(format!("New task request detected"));
                    let mut to_do: ToDo = get_to_do(None);
                    let response = create_new_task_from_arguments(args, &mut to_do)?;
                    store(to_do);
                    Ok(response)
                },
                "clockIn" => {
                    trace(format!("Clock in request detected"));
                    let mut to_do: ToDo = get_to_do(None);
                    let response = clock_in(args, &mut to_do)?;
                    store(to_do);
                    Ok(response)
                },
                "clockOut" => {
                    trace(format!("Clock out request detected"));
                    let mut to_do: ToDo = get_to_do(None);
                    let response = clock_out(args, &mut to_do)?;
                    store(to_do);
                    Ok(response)
                },
                "report" => {
                    trace(format!("Report request detected"));
                    let mut to_do: ToDo = get_to_do(None);
                    report(&mut to_do)
                },
                "help" => {
                    trace(format!("Help request detected"));
                    //TODO HIGH PRIO return help information
                    Err(warn(format!("help command not implemented")))
                },
                _ => {
                    Err(warn(format!("Unknown command \"{}\". {}", command, recommend_help())))
                },
            }
        },
        None => {
            Err(warn(format!("No command received. {}", recommend_help())))
        },
    }
}

fn report(to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() {
        let message = format!("No tasks are recorded");
        warn(message.clone());
        return Err(message)
    }

    Ok(to_do.to_report().as_string())
}

//TODO LOW PRIO optimize pass function as a parameter
fn clock_out(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() {
        let message = format!("No tasks are recorded");
        warn(message.clone());
        return Err(message)
    }

    match args.nth(0) {
        Some(task_name) => {
            match to_do.clock_out(task_name.to_string()) {
                Ok(message) => {
                    trace(message.clone());
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

fn clock_in(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() {
        let message = format!("No tasks are recorded");
        warn(message.clone());
        return Err(message)
    }

    match args.nth(0) {
        Some(task_name) => {
            match to_do.clock_in(task_name.to_string()) {
                Ok(message) => {
                    trace(message.clone());
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

fn create_new_task_from_arguments(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    match args.nth(0) {
        Some(task) => {
            let labels: Vec<String> = args.cloned().collect();
            match to_do.add(task.clone(), labels) {
                Ok(message) => { Ok(trace(message.clone())) },
                Err(message) => { Err(error(message)) },
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

#[cfg(test)]
mod test_set;
