#[macro_use]
extern crate log;
use log::LevelFilter;
extern crate simple_logging;

pub fn initiate_logging(log_name: Option<String>) -> Result<String,String>
{
    let log_file = log_name.unwrap_or(format!("trace.log"));
    match simple_logging::log_to_file(log_file, LevelFilter::Trace) {
        Ok(_) => {
            Ok(format!("Logging initiated"))
        },
        Err(error) => {
            Err(format!(" {} occured while trying to intiate logging",error))
        },
    }
}

pub fn error(message: String) -> String {
    error!("{}", message.clone());
    message
}

pub fn warn(message: String) -> String {
    warn!("{}", message);
    message
}

pub fn info(message: String) -> String {
    info!("{}", message);
    message
}

pub fn debug(message: String) -> String {
    debug!("{}", message);
    message
}

pub fn trace(message: String) -> String {
    trace!("{}", message);
    message
}
