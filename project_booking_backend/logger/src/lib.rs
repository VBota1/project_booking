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

pub fn error(error: String) {
    error!("{}", error);
}

pub fn warn(message: String) {
    warn!("{}", message);
}

pub fn info(message: String) {
    info!("{}", message);
}

pub fn debug(message: String) {
    debug!("{}", message);
}

pub fn trace(message: String) {
    trace!("{}", message);
}
