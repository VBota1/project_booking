extern crate project_booking_backend;
extern crate logger;
extern crate todo;
extern crate formaters;

use logger::*;
use project_booking_backend::*;
use todo::ToDo;
use std::slice::Iter;
use formaters::AsString;

pub struct Response {
    pub message: Result<String, String>,
    pub should_save: bool,
}

fn main() {
    match initiate_logging(None) {
        Ok(message) => { trace(message); }
        Err(message) => {
            eprintln!("{}", error(message));
            return;
        }
    };


    let arguments_vector: Vec<String> = std::env::args().collect();
    trace(format!("User application call: {}", arguments_vector.join(" ")));

    let mut to_do: ToDo = get_to_do(None);

    let mut args = arguments_vector.iter();

    args.next();

    let result = handle_command(args, &mut to_do);

    if true == result.should_save {
        match store(to_do) {
            Ok(_) => {}
            Err(store_error) => {
                eprintln!("{}", error(format!("{}. {}", result.message.as_string(), store_error)));
            }
        };
    }

    match result.message {
        Ok(text) => { println!("{}", text); }
        Err(text) => { eprintln!("{}", text); }
    };
}

fn handle_command(mut args: Iter<String>, to_do: &mut ToDo) -> Response {
    match args.nth(0) {
        Some(command) => {
            match command.as_str() {
                NEW => {
                    trace(format!("New task request detected"));
                    Response { message: create_new_task_from_arguments(args, to_do), should_save: true }
                }
                CLOCKIN => {
                    trace(format!("Clock in request detected"));
                    Response { message: clock_in(args, to_do), should_save: true }
                }
                CLOCKOUT => {
                    trace(format!("Clock out request detected"));
                    Response { message: clock_out(args, to_do), should_save: true }
                }
                REPORT => {
                    trace(format!("Report request detected"));
                    Response { message: report(to_do), should_save: false }
                }
                REPORTBYLABEL => {
                    trace(format!("Report time spent on labels request detected"));
                    Response { message: report_time_on_labels(to_do), should_save: false }
                }
                REPORTFORMONTH => {
                    trace(format!("Daily activity report request detected"));
                    Response { message: daily_activity_report(args, to_do), should_save: false }
                }
                ADDTIME => {
                    trace(format!("Add time request detected"));
                    Response { message: add_time(args, to_do), should_save: true }
                }
                REMOVETIME => {
                    trace(format!("Remove time request detected"));
                    Response { message: remove_time(args, to_do), should_save: true }
                }
                DELETE => {
                    trace(format!("Delete request detected"));
                    Response { message: delete(args, to_do), should_save: true }
                }
                HELP => {
                    trace(format!("Help request detected"));
                    Response { message: Ok(help()), should_save: false }
                }
                LICENSE => {
                    trace(format!("License request detected"));
                    Response { message: Ok(license()), should_save: false }
                }
                _ => {
                    Response { message: Err(warn(format!("Unknown command \"{}\". {}", command, recommend_help()))), should_save: false }
                }
            }
        }
        None => {
            Response { message: Err(warn(format!("No command received. {}", recommend_help()))), should_save: false }
        }
    }
}
