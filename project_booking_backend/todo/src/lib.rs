extern crate identifiers;
extern crate task;
use identifiers::UniqueIdentifier;
use task::Task;
use std::fs::OpenOptions;
use std::io::Write;

extern crate formaters;

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

    pub fn to_report (&mut self) -> Vec<Vec<String>> {
        let mut output = Vec::new();
        let tasklist = self.list.as_slice();
        for t in tasklist {
            output.push(t.as_vec_string());
        }
        output
    }

    pub fn to_simple_report (&mut self) -> Vec<Vec<String>> {
        let mut output = Vec::new();
        let tasklist = self.list.as_slice();
        for t in tasklist {
            output.push(t.name_and_duration_as_vec_string());
        }
        output
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

        match OpenOptions::new().write(true).create(true).open(backup_file.clone()) {
            Ok(mut file) => {
                match file.write_all(serilazed_data.as_bytes() ) {
                    Ok(_) => {},
                    Err(error) => {
                        return Err(format!(" \"{}\" occured while saving data to \"{}\"", error.to_string(), backup_file.clone()));
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
        match self.list.index_of_task_by_name(task_name.clone()) {
            Ok(index) => {
                self.list.get_mut(index).unwrap().clock_in();
                Ok(format!("Clocked in taks \"{}\"", task_name))
            },
            Err(message) => {
                Err(message)
            },
        }
    }

    pub fn clock_out(&mut self, task_name: String) -> Result<String, String> {
        match self.list.index_of_task_by_name(task_name.clone()) {
            Ok(index) => {
                self.list.get_mut(index).unwrap().clock_out()
            },
            Err(message) => {
                Err(message)
            },
        }
    }

    pub fn count(&self) -> usize {
        self.list.len()
    }
    /*TODO not used
    pub fn remove_by_name ( &mut self, p_name: String ) {
        match self.list.index_of_task_by_name(p_name) {
            Ok(index) => {
                self.list.remove(index);
            },
            Err(_) => {},
        };
    }

    pub fn remove ( &mut self, p_task: Task ) {
        match self.list.index_of(p_task) {
            Ok(index) => {
                self.list.remove(index);
            },
            Err(_) => {},
        };
    }
    */
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
            return Err(format!(" \"{}\" while reading file \"{}\"", error.to_string(), backup_file.clone()));
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

/*TODO not used
trait FindTask {
    fn index_of(&self, Task) -> Result<usize, String>;
}

impl FindTask for Vec<Task> {
    fn index_of(&self, to_find: Task) -> Result<usize, String> {
        match self.iter().position(|o| o.eq(&to_find) ) {
            Some(index) => Ok(index),
            None => Err(format!("Task {} was not found in vector",to_find.name())),
        }
    }
}
*/

#[cfg(test)]
mod test_set;
