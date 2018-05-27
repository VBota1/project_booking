extern crate project_booking_backend;
use project_booking_backend::*;
extern crate logger;
use logger::*;

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
        "applicationMode" => {
            println!("TODO implement application mode");
        },
        _ => {
            println!("{}", handle_command_as_service(arguments_vector).to_string());
        }
    }
}

fn application_mode() {}
