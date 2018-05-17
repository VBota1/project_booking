pub trait FindString {
    fn index_of(&self, String) -> Result<usize, String>;
}

impl FindString for Vec<String> {
    fn index_of(&self, to_find: String) -> Result<usize, String> {
        match self.iter().position(|o| *o == to_find ) {
            Some(index) => Ok(index),
            None => Err(format!("String {} was not found in vector",to_find)),
        }
    }
}
