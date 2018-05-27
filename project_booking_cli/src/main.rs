extern crate project_booking_backend;
use project_booking_backend::*;
extern crate logger;
use logger::*;
use std::slice::Iter;

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
    let arguments_iterator: Iter<String> = arguments_vector.iter();
    println!("{}", handle_command(arguments_iterator).to_string());
}
