extern crate project_booking_backend;
extern crate logger;
extern crate formaters;

use logger::*;
use std::io;
use std::io::Write;
use project_booking_backend::*;
use std::slice::Iter;
use formaters::{AsString, AsHHMMSS};
use std::time::Duration;
use std::thread::sleep;

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
                    Response { message: delete(args, to_do).as_string(), should_save: true }
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
            .trim()
            .to_string()
    }
}

#[test]
fn test_handle_command_as_service() {
    let to_do: ToDo = ToDo::new();
    forced_store(to_do);

    let aplication_name = format!("test");
    let command = format!("new");
    let task_name = format!("task512");
    let label_1 = format!("project_1");
    let label_2 = format!("project_2");
    let args_vec = vec![aplication_name.clone(), command, task_name.clone(), label_1.clone(), label_2.clone()];
    handle_command_as_service(args_vec);

    let command = format!("clockIn");
    let args_vec = vec![aplication_name.clone(), command, task_name.clone()];
    handle_command_as_service(args_vec);

    let actual_time_spent_on_task = Duration::new(5, 0);
    sleep(actual_time_spent_on_task);

    let command = format!("clockOut");
    let args_vec = vec![aplication_name.clone(), command, task_name.clone()];
    handle_command_as_service(args_vec);

    let command = format!("report");
    let args_vec = vec![aplication_name.clone(), command, task_name.clone()];
    let actual_report = handle_command_as_service(args_vec);

    let expected_report = format!("[{{\"id\":\"1\",\"name\":\"{}\",\"time_spent\":\"{}\",\"labels\":[\"{}\",\"{}\"],\"clock_in_timestamp\":\"None\"}}]", task_name, actual_time_spent_on_task.as_hhmmss(), label_1, label_2);
    let expected_report = expected_report.format_for_display().replace("\n", "").replace("\t", "").replace(" ", "");
    let actual_report = actual_report.replace("\n", "").replace("\t", "").replace(" ", "");

    assert!(actual_report == expected_report, "Actual report \"{}\" Expected report \"{}\"", actual_report, expected_report);
}

#[test]
fn activity_report() {
    let mut to_do: ToDo = ToDo::new();

    let task_name_510 = format!("task510");
    let label_1 = format!("label_1");
    let label_2 = format!("label_2");
    let args_vec = vec![task_name_510.clone(), label_1.clone(), label_2.clone()];
    let args = args_vec.iter();
    create_new_task_from_arguments(args, &mut to_do);

    let time_argument = format!("01:01");
    let args_vec = vec![task_name_510.clone(), time_argument];
    let args = args_vec.iter();
    match add_time(args, &mut to_do) {
        Ok(response) => {
            let expected_response = format!("Time spent on task \"{}\" is now \"01:01:00\"", task_name_510.clone());
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        },
        Err(response) => {
            assert!(false, response);
        }
    };

    let time_argument = format!("01:01");
    let date_argument = format!("01.01.2001");
    let args_vec = vec![task_name_510.clone(), time_argument, date_argument.clone()];
    let args = args_vec.iter();
    match add_time(args, &mut to_do) {
        Ok(response) => {
            let expected_response = format!("Time spent on task \"{}\" is now \"01:01:00\"", task_name_510.clone());
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        },
        Err(response) => {
            assert!(false, response);
        }
    };

    let task_name_500 = format!("task500");
    let args_vec = vec![task_name_500.clone()];
    let args = args_vec.iter();
    create_new_task_from_arguments(args, &mut to_do);

    let time_argument = format!("00:01");
    let args_vec = vec![task_name_500.clone(), time_argument];
    let args = args_vec.iter();
    match add_time(args, &mut to_do) {
        Ok(response) => {
            let expected_response = format!("Time spent on task \"{}\" is now \"00:01:00\"", task_name_500.clone());
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        },
        Err(response) => {
            assert!(false, response);
        }
    };

    let month_argument = format!("01");
    let args_vec = vec![month_argument];
    let args = args_vec.iter();
    let actual_report = daily_activity_report(args, &to_do).as_string();
    let expected_report = format!("[{{\"date\":\"{}\",\"tasks\":[{{\"id\":\"1\",\"name\":\"{}\",\"time_spent\":\"02:02:00\",\"labels\":[\"{}\",\"{}\"],\"clock_in_timestamp\":\"None\"}}]}}]", date_argument, task_name_510, label_1, label_2);
    assert!(actual_report == expected_report, "Actual report {} Expected report {}", actual_report, expected_report);
}

