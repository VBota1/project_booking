extern crate project_booking_backend;

use project_booking_backend::*;

extern crate todo;

use todo::ToDo;

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

    let mut to_do = match ToDo::load(None) {
        Ok(todo) => {
            trace(format!("Jobs loaded from database"));
            todo
        }
        Err(error) => {
            warn(error.to_string());
            ToDo::new()
        }
    };

    let args: Vec<String> = std::env::args().collect();
    trace(format!("User application call: {}", args.join(" ")));
    println!("{}", handle_command(std::env::args(), &mut to_do).to_string());

    match to_do.save(None) {
        Ok(_) => {}
        Err(message) => { error(message); }
    };
}
