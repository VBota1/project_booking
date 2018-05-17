extern crate todo;
use todo::ToDo;
extern crate logger;
use logger::*;
use std::env;

fn backend_main() {

    match initiate_logging(None) {
        Ok(message) => { trace(message); },
        Err(message) => { error(message); return; },
    };

    let mut to_do = match ToDo::load(None) {
        Ok(todo) => { trace(format!("Jobs loaded from database")); todo },
        Err(error) => { warn(error.to_string()); ToDo::new() },
    };

    let args: Vec<String> = env::args().collect();
    trace(format!("User application call: {}", args.join(" ")));
    handle_command(env::args(), &mut to_do);

    match to_do.save(None) {
        Ok(_) => {},
        Err(message) => { error(message); },
    };

    //TODO MEDIUM PRIO GUI (QT)

    //TODO MEDIUM PRIO import tasks from Jira

    //TODO MEDIUM PRIO export tasks to PTT

    //TODO LOW PRIO detect AFK and stop recoding

    //TODO LOW PRIO detect return on Keyboard and ask what task I am working on
}

fn handle_command(mut args: std::env::Args, to_do: &mut ToDo) {
    match args.nth(1) {
        Some(command) => {
            match command.as_str() {
                "new" => {
                    trace(format!("New task request detected"));
                    create_new_task_from_arguments(args, to_do);
                },
                "clockIn" => {
                    //TODO HIGH PRIO start to keep track of time (on task) (external thread)
                },
                "clockOut" => {
                    //TODO HIGH PRIO stop to keep track of time (on task)
                    //TODO HIGH PRIO add recorded duration to task
                },
                "report" => {
                    //TODO HIGH PRIO report time spent on one or all tasks
                    //TODO HIGH PRIO report time spent on one or all labels
                },
                "help" => {
                    //TODO HIGH PRIO return help information
                },
                _ => {
                    error(format!("Unkown command \"{}\". {}", command, recommend_help()));
                    return;
                },
            };
        },
        None => {
            error(format!("No command received. {}", recommend_help()));
            return;
        },
    };
}

fn create_new_task_from_arguments(mut args: std::env::Args, to_do: &mut ToDo) {
    match args.nth(0) {
        Some(task) => {
            let labels: Vec<String> = args.collect();
            match to_do.add(task.clone(), labels) {
                Ok(_) => {
                    trace(format!("New task {} created.", task));
                },
                Err(message) => { error(message); }
            };
        },
        None => {
            error(format!("No task name received. {}", recommend_help()));
        },
    };
}

fn recommend_help() -> String {
    format!("Call \"project_booking help\" for aditional information.")
}