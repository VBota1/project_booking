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

    let args: Vec<String> = std::env::args().collect();
    trace(format!("User application call: {}", args.join(" ")));
    println!("{}", handle_command(std::env::args()).to_string());
}
