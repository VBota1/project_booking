use std::time::{Duration, SystemTime};
extern crate search;
//TODO not used use search::FindString;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    _id: i32,
    _name: String,
    _time_spent: Duration,
    _labels: Vec<String>,
    _clock_in_timestamp: Option<SystemTime>,
}

impl Task {
    pub fn new(uid: i32, p_name: String, p_labels: Vec<String>) -> Task {
        Task { _id: uid, _name: p_name, _labels: p_labels, _time_spent: Duration::new(0, 0), _clock_in_timestamp: None }
    }

    pub fn as_vec_string (&self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        output.push(format!("{}",self._id));
        output.push(format!("{}",self._name));

        match self._clock_in_timestamp {
            Some(timestamp) => {
                output.push(format!("{:?}", timestamp));
            },
            None => {
                output.push(format!("None"));
            }
        };

        output.push( format!("{}",self._time_spent.as_secs()));
        let labels = self._labels.clone();
        for l in labels {
            output.push(format!("{}",l));
        }
        output
    }

    pub fn name_and_duration_as_vec_string (&self) -> Vec<String> {
        self.as_vec_string()[1..3].to_vec()
    }

    pub fn clock_in(&mut self) {
        self._clock_in_timestamp = Some(SystemTime::now());
    }

    /*TODO not used
    pub fn add_time_spent(&mut self, duration: Duration ) {
        self._time_spent += duration;
    }

    pub fn add_label (&mut self, label: String) {
        self._labels.push(label);
    }

    pub fn remove_label (&mut self, label: String) {
        match self._labels.index_of(label) {
            Ok(index) => {
                self._labels.remove(index);
            },
            Err(_) => {},
        };
    }
*/
    pub fn name (&self) -> String {
        self._name.clone()
    }

    pub fn id (&self) -> i32 {
        self._id.clone()
    }

}
/*TODO not used
impl PartialEq for Task {
fn eq(&self, to_find: &Task) -> bool {
    if ( self._id == to_find._id) {
        if ( self._name == to_find._name) {
            if ( self._time_spent == to_find._time_spent) {
                if ( self._labels == to_find._labels) {
                    return true
                }
            }
        }
    }
    false
}
}
*/