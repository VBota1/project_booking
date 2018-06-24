extern crate project_booking_backend;
extern crate logger;
extern crate formaters;

use logger::*;
use std::io;
use std::io::Write;
use project_booking_backend::*;
use std::slice::Iter;
use formaters::AsString;

extern crate todo;

use todo::*;

fn main() {
    match initiate_logging(None) {
        Ok(message) => { trace(message); }
        Err(message) => {
            error(message);
            return;
        }
    };

    let arguments_vector: Vec<String> = std::env::args().collect();
    trace(format!("User application call: {}", arguments_vector.join(" ")));
    let command = match arguments_vector.get(1) {
        Some(argument) => { argument.clone() },
        None => { format!("error") },
    };

    match command.as_str() {
        APPLICATIONMODE => {
            trace(format!("Application mode request detected"));
            application_mode();
        },
        _ => {
            println!("{}", handle_command_as_service(arguments_vector).to_string());
        }
    }
}

fn handle_command_as_service(args: Vec<String>) -> String {
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

fn application_mode() {
    println!("{}", trace(format!("Application mode request started.")));

    let mut to_do: ToDo = get_to_do(None);

    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => {},
            Err(message) => {
                println!("{}", error(format!("Error \"{}\" occurred while writing to stdout!", message)));
                break;
            }
        };

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(message) => {
                println!("{}", error(format!("Error \"{}\" occurred while reading user input!", message)));
            },
        };

        let input: Vec<String> = input.split_whitespace().map(|word| word.to_string()).collect();

        let command = match input.get(0) {
            Some(argument) => { argument.clone() },
            None => { format!("error") },
        };

        match command.as_str() {
            EXIT => {
                trace(format!("Exit application mode request detected."));
                break;
            },
            _ => {
                println!("{}", handle_command_as_application(input.iter(), &mut to_do).message);
            }
        };
    }

    match store(to_do) {
        Ok(_) => {},
        Err(store_error) => { println!("{}", store_error); }
    };
}

fn handle_command_as_application(mut args: Iter<String>, to_do: &mut ToDo) -> Response {
    match args.nth(0) {
        Some(command) => {
            match command.as_str() {
                NEW => {
                    trace(format!("New task request detected"));
                    Response { message: create_new_task_from_arguments(args, to_do).as_string(), should_save: true }
                },
                CLOCKIN => {
                    trace(format!("Clock in request detected"));
                    Response { message: clock_in(args, to_do).as_string(), should_save: true }
                },
                CLOCKOUT => {
                    trace(format!("Clock out request detected"));
                    Response { message: clock_out(args, to_do).as_string(), should_save: true }
                },
                REPORT => {
                    trace(format!("Report request detected"));
                    Response { message: report(to_do).as_string().format_for_display(), should_save: false }
                },
                REPORTBYLABEL => {
                    trace(format!("Report time spent on labels request detected"));
                    Response { message: report_time_on_labels(to_do).as_string().format_for_display(), should_save: false }
                },
                REPORTFORMONTH => {
                    trace(format!("Daily activity report request detected"));
                    Response { message: daily_activity_report(args, to_do).as_string().format_for_display(), should_save: false }
                },
                ADDTIME => {
                    trace(format!("Add time request detected"));
                    Response { message: add_time(args, to_do).as_string(), should_save: true }
                }
                DELETE => {
                    trace(format!("Delete request detected"));
                    Response { message: delete(args, to_do).as_string(), should_save: false }
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

trait FormatJsonForDisplay {
    fn format_for_display(self) -> String;
}

impl FormatJsonForDisplay for String {
    fn format_for_display(self) -> String {
        self.replace("\"", "")
            .replace("{", "")
            .replace("[", "")
            .replace("]", "")
            .replace(",tasks:", "\n")
            .replace("}", "\n")
            .replace(",id", "id")
            .replace(",name", "\tname")
            .replace(",time", "\ttime")
            .replace(",labels", "\tlabels")
            .replace(",clock", "\tclock")
            .replace(",label", "label")
            .replace(",date", "date")
    }
}