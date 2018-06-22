use super::ToDo;
use super::load;
use std::time::Duration;
use std::thread::sleep;
use formaters::AsHHMMSS;

#[test]
fn id_is_unique () {
    let mut todo = ToDo::new();
    todo.add(format!("task1"), Vec::new());
    todo.add(format!("task2"), Vec::new());
    todo.add(format!("task3"), Vec::new());
    todo.add(format!("task4"), Vec::new());

    let mut task_count =0;
    for mut t in todo.list {
        task_count +=1;
        assert!(t.id() == task_count, format!("\nActual \t\t{} \nExpected \t{}", t.id(), task_count));
    }
}

#[test]
fn put_data_to_storage () {
    let mut todo = ToDo::new();
    todo.add(format!("task1"), Vec::new());
    todo.add(format!("task2"), Vec::new());
    todo.add(format!("task3"), Vec::new());
    todo.add(format!("task4"), Vec::new());

    match todo.save(None) {
        Ok(message) => {
            assert!(true,"{}",message);
        },
        Err(message) => {
            assert!(false,"{}",message);
        },
    };
}

#[test]
fn retrieve_from_storage() {
    let mut todo = ToDo::new();
    let task_name = format!("task0");
    todo.add(task_name.clone(), Vec::new());
    todo.add(format!("task1"), Vec::new());
    todo.add(format!("task2"), Vec::new());
    todo.add(format!("task3"), Vec::new());

    match todo.save(None) {
        Ok(_) => { },
        Err(message) => {
            assert!(false,"{}",message);
        },
    };

    match load(None) {
        Ok(reterived_data) => {
            if let Some(task) = reterived_data.list.get(0) {
                let expected = task_name.clone();
                assert!(task.name() == expected, format!("\nActual \t\t{} \nExpected \t{}", task.name(), expected));
            } else { assert!(false, "No task could not be retrieved from the loaded data."); }
        },
        Err(message) => {
            assert!(false,"{}",message);
        },
    };
}

#[test]
fn reject_new_task_with_same_name () {
    let mut todo = ToDo::new();
    let task_name = format!("task0");
    match  todo.add(task_name.clone(), Vec::new()) {
        Ok(_) => {},
        Err(message) => { assert!(false,message); },
    };

    match  todo.add(task_name.clone(), Vec::new()) {
        Ok(_) => { assert!(false,format!("Adding 2 tasks with name {} was possible",task_name)); },
        Err(_) => { assert!(true, format!("Adding another tasks with name {} was rejected", task_name)); },
    };
}

#[test]
fn measure_time_spent_on_task() {
    let mut todo = ToDo::new();
    let task_name = format!("task1");
    todo.add(task_name.clone(), Vec::new());
    let actual_time_spent_on_task = Duration::new(5, 0);
    todo.clock_in(task_name.clone());
    sleep(actual_time_spent_on_task);
    match todo.clock_out(task_name.clone()) {
        Ok(_) => {}
        Err(message) => { assert!(false, message); }
    }

    if let Some(task) = todo.list.get(0) {
        let expected = format!("00:00:05");
        assert!(task.total_time_spent().as_hhmmss() == expected, format!("\nActual \t\t{} \nExpected \t{}", task.total_time_spent().as_hhmmss(), expected));
    } else { assert!(false, "No task could not be retrieved from the loaded data."); }
}

#[test]
fn load_data_after_clock_out() {
    let mut todo = ToDo::new();
    let task_name = format!("task1");
    todo.add(task_name.clone(), Vec::new());

    todo.clock_in(task_name.clone());
    todo.save(None);

    todo = match load(None) {
        Ok(todo) => { todo },
        Err(error) => {
            assert!(false, "Loading data failed with error \"{}\" after clockIn and save performed.", error);
            ToDo::new()
        },
    };
    todo.clock_out(task_name);
    todo.save(None);

    match load(None) {
        Ok(_) => {
            assert!(true, "Loaded data successfully after clockOut and save performed.");
        },
        Err(error) => {
            assert!(false, "Loading data failed with error \"{}\" after clockOut and save perfomed.", error);
        },
    };
}