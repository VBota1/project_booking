extern crate todo;

use todo::*;
extern crate logger;
use logger::*;

//TODO MEDIUM PRIO GUI (QT)
//TODO MEDIUM PRIO import tasks from Jira
//TODO MEDIUM PRIO export tasks to PTT
//TODO LOW PRIO detect AFK and stop recoding
//TODO LOW PRIO detect return on Keyboard and ask what task I am working on

pub fn handle_command(mut args: std::env::Args) -> Result<String, String> {

    match args.nth(1) {
        Some(command) => {
            match command.as_str() {
                "new" => {
                    trace(format!("New task request detected"));
                    create_new_task_from_arguments(args)
                },
                "clockIn" => {
                    //TODO HIGH PRIO start to keep track of time (on task) (external thread)
                    Err(format!("clockIn command not implemented"))
                },
                "clockOut" => {
                    //TODO HIGH PRIO stop to keep track of time (on task)
                    //TODO HIGH PRIO add recorded duration to task
                    Err(format!("clockOut command not implemented"))
                },
                "report" => {
                    //TODO HIGH PRIO report time spent on one or all tasks
                    //TODO HIGH PRIO report time spent on one or all labels
                    Err(format!("report command not implemented"))
                },
                "help" => {
                    //TODO HIGH PRIO return help information
                    Err(format!("help command not implemented"))
                },
                _ => {
                    let result = format!("Unkown command \"{}\". {}", command, recommend_help());
                    error(result.clone());
                    Err(result)
                },
            }
        },
        None => {
            let result = format!("No command received. {}", recommend_help());
            error(result.clone());
            Err(result)
        },
    }
}

fn create_new_task_from_arguments(mut args: std::env::Args) -> Result<String, String> {
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
                    error(message.clone());
                    Err(message)
                },
            }
        },
        None => {
            let result = format!("No task name received. \"{}\"", recommend_help());
            error(result.clone());
            Err(result)
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