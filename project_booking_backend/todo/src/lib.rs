extern crate identifiers;
extern crate task;
extern crate formaters;

use identifiers::UniqueIdentifier;
use task::Task;
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::HashMap;
use std::time::Duration;
use formaters::AsHHMMSS;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub task_id: UniqueIdentifier,
    pub list: Vec<Task>,
}

impl ToDo {
    pub fn new() -> ToDo {
        ToDo { task_id: UniqueIdentifier::init(), list: Vec::new() }
    }

    pub fn add ( &mut self, p_name: String, p_labels: Vec<String> ) -> Result<String,String> {
        match self.list.iter().position(|t| t.name() == p_name ) {
            Some(_) => {
                Err(format!("A task with the name \"{}\" already exists", p_name))
            },
            None => {
                self.list.push(Task::new (self.task_id.new(), p_name.clone(), p_labels ) );
                Ok(format!("Task with name \"{}\" was added", p_name))
            },
        }
    }

    pub fn to_report(&self) -> Vec<Vec<String>> {
        let mut output = Vec::new();
        let tasklist = self.list.as_slice();
        for t in tasklist {
            output.push(t.as_vec_string());
        }
        output
    }

    pub fn report_time_spent_on_labels(&self) -> Vec<Vec<String>> {
        let mut time_on_labels = HashMap::new();

        let tasklist = self.list.as_slice();
        for t in tasklist {
            let time_per_label = t.time_spent().checked_div(t.labels().len() as u32).unwrap_or(t.time_spent());

            let labels = t.labels();
            for l in labels {
                let value = time_on_labels.entry(l).or_insert(Duration::new(0, 0));
                *value += time_per_label;
            }
        }

        let mut result: Vec<Vec<String>> = Vec::new();
        for (label, time) in time_on_labels {
            let mut line: Vec<String> = vec![label, time.as_hhmmss()];
            result.push(line);
        }
        result
    }

    pub fn save(&self, save_file: Option<String>) -> Result<String,String> {
        let serilazed_data;
        match serde_json::to_string(self) {
            Ok(data_as_string) => {
                serilazed_data = data_as_string;
            },
            Err(_) => {
                return Err (format!("Data could not be serialized"));
            }

        };

        let backup_file = save_file.unwrap_or(backup_file());

        match OpenOptions::new().create(true).write(true).truncate(true).open(backup_file.clone()) {
            Ok(mut file) => {
                match file.write_all(serilazed_data.as_bytes() ) {
                    Ok(_) => {},
                    Err(error) => {
                        return Err(format!(" \"{}\" occurred while saving data to \"{}\"", error.to_string(), backup_file.clone()));
                    }
                };
            },
            Err(error) => {
                return Err(format!(" \"{}\" while creating file \"{}\"", error.to_string(), backup_file.clone()));
            }
        };

        Ok(format!("Todo was saved."))
    }

    pub fn clock_in(&mut self, task_name: String) -> Result<String, String> {
        let index = self.list.index_of_task_by_name(task_name.clone())?;
        self.list.get_mut(index).unwrap().clock_in()
    }

    pub fn clock_out(&mut self, task_name: String) -> Result<String, String> {
        let index = self.list.index_of_task_by_name(task_name.clone())?;
        self.list.get_mut(index).unwrap().clock_out()
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }

    pub fn add_time_spent_to_task(&mut self, task_name: String, time: Duration) -> Result<Duration, String> {
        let index = self.list.index_of_task_by_name(task_name)?;
        Ok(self.list.get_mut(index).unwrap().add_time_spent(time))
    }

    pub fn remove_task(&mut self, task_name: String) -> Result<String, String> {
        let index = self.list.index_of_task_by_name(task_name.clone())?;
        self.list.remove(index);
        Ok(format!("Task {} was removed.", task_name))
    }
}

fn backup_file () -> String {
    format!("ToDoData")
}

pub fn load(load_file: Option<String>) -> Result<ToDo, String> {
    let backup_file = load_file.unwrap_or(backup_file());

    let file = match OpenOptions::new().read(true).open(backup_file.clone()) {
        Ok(file) => {
            file
        },
        Err(error) => {
            return Err(format!("\"{}\" while reading file \"{}\"", error.to_string(), backup_file.clone()));
        },
    };

    match serde_json::from_reader(file) {
        Ok(todo) => {
            Ok(todo)
        },
        Err(error) => {
            Err(format!(" \"{}\" while deserializing file \"{}\"", error.to_string(), backup_file.clone()))
        },
    }
}

trait FindTaskByName {
    fn index_of_task_by_name(&self, String) -> Result<usize, String>;
}

impl FindTaskByName for Vec<Task> {
    fn index_of_task_by_name(&self, to_find: String) -> Result<usize, String> {
        match self.iter().position(|o| o.name() == to_find ) {
            Some(index) => Ok(index),
            None => Err(format!("Task {} was not found.", to_find)),
        }
    }
}

#[cfg(test)]
mod test_set;

/*
LICENSE for extern crates serde, serde_json and serde_derive:
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
*/
