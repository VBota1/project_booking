use std::time::{Duration, SystemTime};
use formaters::*;
use std::collections::HashMap;
use chrono::{NaiveDate, Datelike, Local};

extern crate search;
extern crate formaters;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    _id: i32,
    _name: String,
    _time_spent: HashMap<String, Duration>,
    _labels: Vec<String>,
    _clock_in_timestamp: Option<SystemTime>,
}

impl Task {
    pub fn new(uid: i32, p_name: String, p_labels: Vec<String>) -> Task {
        Task { _id: uid, _name: p_name, _labels: p_labels, _time_spent: HashMap::new(), _clock_in_timestamp: None }
    }

    pub fn as_vec_string (&self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        output.push(format!("task id: {}", self._id));

        output.push(format!("name: {}", self._name));

        output.push(format!("time spent: {}", self.total_time_spent().as_hhmmss()));

        output.push(format!("clock in timestamp:"));
        match self._clock_in_timestamp {
            Some(timestamp) => {
                output.push(format!("{:?}", timestamp));
            },
            None => {
                output.push(format!("None"));
            }
        };

        output.push(format!("labels:"));
        let labels = self._labels.clone();
        if self._labels.len() <= 0 {
            output.push(format!("None"));
        } else {
            for l in labels {
                output.push(format!("{}", l));
            }
        }

        output
    }

    pub fn clock_in(&mut self) -> Result<String, String> {
        self._clock_in_timestamp = Some(SystemTime::now());
        Ok(format!("Clocked in task \"{}\"", self.name()))
    }

    pub fn clock_out(&mut self) -> Result<String, String> {
        match self._clock_in_timestamp {
            Some(time) => {
                match time.elapsed() {
                    Ok(duration) => {
                        self.add_time_spent(None, duration);
                        self._clock_in_timestamp = None;
                        Ok(format!("Successfully clocked out of \"{}\"", self.name()))
                    },
                    Err(message) => {
                        Err(format!("Error \"{}\" occured when trying to get elapsed time since clock in event", message))
                    },
                }
            },
            None => {
                Err(format!("Can not clock out of \"{}\" as no previous clock in was done", self.name()))
            },
        }
    }

    pub fn name (&self) -> String {
        self._name.clone()
    }

    pub fn id (&self) -> i32 {
        self._id.clone()
    }

    pub fn labels(&self) -> Vec<String> {
        self._labels.to_vec()
    }

    pub fn total_time_spent(&self) -> Duration {
        self._time_spent.values().fold(Duration::new(0, 0), |acc, x| acc + x.clone())
    }

    pub fn time_spent(&self) -> HashMap<String, Duration> {
        self._time_spent.clone()
    }

    pub fn add_time_spent(&mut self, date_to_add_time: Option<NaiveDate>, time: Duration) -> Duration {
        let date_for_duration = match date_to_add_time {
            Some(date) => date,
            None => {
                let date_time = Local::now().naive_local();
                NaiveDate::from_ymd(date_time.year(), date_time.month(), date_time.day())
            }
        };
        let value = self._time_spent.entry(date_for_duration.as_dmy()).or_insert(Duration::new(0, 0));
        *value += time;
        value.clone()
    }
}

/*MIT LICENSE for extern crates serde, serde_json and serde_derive:
Copyright (c) 2014 The Rust Project Developers
Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:
The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

MIT LICENSE for extern crate chrono
The MIT License (MIT)
Copyright (c) 2014, Kang Seonghoon.
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/