#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct UniqueIdentifier {
    pub value: i32,
}

impl UniqueIdentifier {
    pub fn init() -> UniqueIdentifier {
        UniqueIdentifier { value: 0 }
    }

    pub fn new(&mut self) -> i32{
        self.value += 1;
        self.value
    }
}