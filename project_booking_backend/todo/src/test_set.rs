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
    for mut t in todo.to_report() {
        task_count +=1;
        let expected = format!("{0} task{0} 00:00:00 None", task_count);
        assert!(t == expected, format!("Actual {} Expected {}", t, expected));
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
            if let Some(task) = reterived_data.to_report().get(0) {
                let expected = format!("1 {} 00:00:00 None", task_name.clone());
                assert!(task.clone() == expected, format!("Actual {} Expected {}", task, expected));
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

    if let Some(task) = todo.to_report().get(0) {
        let expected = format!("1 {} 00:00:05 None", task_name.clone());
        assert!(task.clone() == expected, format!("Actual {} Expected {}", task, expected));
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
            assert!(false, "Loading data failed with error \"{}\" after clockIn and save perfomed.", error);
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

#[test]
fn report_time_by_label() {
    let task1 = format!("task1");
    let task2 = format!("task2");
    let task3 = format!("task3");
    let task4 = format!("task4");
    let label1 = format!("label_1");
    let label2 = format!("label_2");
    let label3 = format!("label_3");
    let mut todo = ToDo::new();
    todo.add(task1.clone(), vec![label1.clone()]);
    todo.add(task2.clone(), vec![label2.clone()]);
    todo.add(task3.clone(), vec![label1.clone(), label2.clone()]);
    todo.add(task4.clone(), vec![label3.clone()]);
    todo.clock_in(task1.clone());
    todo.clock_in(task2.clone());
    todo.clock_in(task3.clone());
    todo.clock_in(task4.clone());

    let actual_time_spent_on_task = Duration::new(6, 0);
    sleep(actual_time_spent_on_task);

    todo.clock_out(task1);
    todo.clock_out(task2);
    todo.clock_out(task3);
    todo.clock_out(task4);

    let actual_report = todo.report_time_spent_on_labels();

    for line in actual_report {
        let mut line_vector: Vec<String> = line.clone().split_whitespace().map(|s| format!("{}", s)).collect();
        let label_name = line_vector.remove(0);
        let time_spent = line_vector.remove(0);
        if label_name == label1 {
            let expected = Duration::new(9, 0).as_hhmmss();
            assert!(time_spent == expected, "Label {} Actual {} Expected {}", label1, time_spent, expected);
        } else {
            if label_name == label2 {
                let expected = Duration::new(9, 0).as_hhmmss();
                assert!(time_spent == expected, "Label {} Actual {} Expected {}", label2, time_spent, expected);
            } else {
                if label_name == label3 {
                    let expected = Duration::new(6, 0).as_hhmmss();
                    assert!(time_spent == expected, "Label {} Actual {} Expected {}", label3, time_spent, expected);
                } else {
                    assert!(false, "Unexpected label \"{}\" was reported", label_name);
                }
            }
        }
    }
}
