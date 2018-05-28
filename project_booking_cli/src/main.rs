extern crate project_booking_backend;
use project_booking_backend::*;
extern crate logger;
use logger::*;
use std::io;
use std::io::Write;

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
