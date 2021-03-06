use super::ToDo;
use super::*;
use std::time::Duration;
use std::thread::sleep;
use formaters::{AsHHMMSS, AsDMY};

#[test]
fn control_1_task_as_service() {
    let mut to_do: ToDo = ToDo::new();

    let task_name = format!("task510");
    let label_1 = format!("label_1");
    let label_2 = format!("label_2");
    let args_vec = vec![task_name.clone(), label_1.clone(), label_2.clone()];
    let args = args_vec.iter();
    create_new_task_from_arguments(args, &mut to_do);

    store(to_do);

    let mut to_do: ToDo = get_to_do(None);

    let args_vec = vec![task_name.clone()];
    let args = args_vec.iter();
    clock_in(args.clone(), &mut to_do);

    store(to_do);

    let actual_time_spent_on_task = Duration::new(5, 0);
    sleep(actual_time_spent_on_task);

    let mut to_do: ToDo = get_to_do(None);

    clock_out(args, &mut to_do);

    store(to_do);

    let mut to_do: ToDo = get_to_do(None);

    let actual_report = match report(&mut to_do) {
        Ok(message) => { message },
        Err(_) => {
            assert!(false, "ToDo report could not be extracted!");
            format!("")
        }
    };

    let expected_report = format!("[{{\"id\":\"1\",\"name\":\"{}\",\"time_spent\":\"{}\",\"labels\":[\"{}\",\"{}\"],\"clock_in_timestamp\":\"None\"}}]", task_name, actual_time_spent_on_task.as_hhmmss(), label_1, label_2);

    assert!(actual_report == expected_report, "Actual report \"{}\" Expected report \"{}\"", actual_report, expected_report);
}

#[test]
fn control_1_task_as_aplication() {
    let mut to_do: ToDo = ToDo::new();

    let task_name = format!("task510");
    let label_1 = format!("label_1");
    let label_2 = format!("label_2");
    let args_vec = vec![task_name.clone(), label_1.clone(), label_2.clone()];
    let args = args_vec.iter();
    create_new_task_from_arguments(args, &mut to_do);

    let args_vec = vec![task_name.clone()];
    let args = args_vec.iter();
    clock_in(args.clone(), &mut to_do);

    let actual_time_spent_on_task = Duration::new(5, 0);
    sleep(actual_time_spent_on_task);

    clock_out(args, &mut to_do);

    let actual_report = match report(&mut to_do) {
        Ok(message) => { message },
        Err(_) => {
            assert!(false, "ToDo report could not be extracted!");
            format!("")
        }
    };

    let expected_report = format!("[{{\"id\":\"1\",\"name\":\"{}\",\"time_spent\":\"{}\",\"labels\":[\"{}\",\"{}\"],\"clock_in_timestamp\":\"None\"}}]", task_name, actual_time_spent_on_task.as_hhmmss(), label_1, label_2);

    assert!(actual_report == expected_report.clone(), "Actual report \"{}\" Expected report \"{}\"", actual_report, expected_report.clone());

    store(to_do);
    let mut to_do: ToDo = get_to_do(None);

    let actual_report = match report(&mut to_do) {
        Ok(message) => { message },
        Err(_) => {
            assert!(false, "ToDo report could not be extracted!");
            format!("")
        }
    };

    assert!(actual_report == expected_report.clone(), "Actual report \"{}\" Expected report \"{}\"", actual_report, expected_report.clone());
}

#[test]
fn check_add_time() {
    let mut to_do: ToDo = ToDo::new();

    let task_name = format!("task510");
    let label_1 = format!("label_1");
    let label_2 = format!("label_2");
    let args_vec = vec![task_name.clone(), label_1.clone(), label_2.clone()];
    let args = args_vec.iter();
    create_new_task_from_arguments(args, &mut to_do);

    let date_time = Local::now().naive_local();
    let current_date = NaiveDate::from_ymd(date_time.year(), date_time.month(), date_time.day()).as_dmy();

    let time_argument = format!("01:01");
    let args_vec = vec![task_name.clone(), time_argument];
    let args = args_vec.iter();
    match add_time(args, &mut to_do) {
        Ok(response) => {
            let expected_response = format!("Time spent for task \"{}\" on date \"{}\" is now \"01:01:00\"", task_name.clone(), current_date);
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        },
        Err(response) => {
            assert!(false, response);
        }
    };

    let time_argument = format!("0101");
    let args_vec = vec![task_name.clone(), time_argument.clone()];
    let args = args_vec.iter();
    match add_time(args, &mut to_do) {
        Ok(response) => {
            assert!(false, "Expected Err Actual {}", response);
        },
        Err(response) => {
            let expected_response = format!("Time to be added to the task, \"{}\" ,is not in the expected format. \"{}\"", time_argument, recommend_help());
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        }
    };

    let time_argument = format!("0a:01");
    let args_vec = vec![task_name.clone(), time_argument.clone()];
    let args = args_vec.iter();
    match add_time(args, &mut to_do) {
        Ok(response) => {
            assert!(false, "Expected Err Actual {}", response);
        },
        Err(response) => {
            let expected_response = format!("Time to be added to the task, \"{}\" ,is not in the expected format. \"{}\"", time_argument, recommend_help());
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        }
    };

    let time_argument = format!("01:o1");
    let args_vec = vec![task_name.clone(), time_argument.clone()];
    let args = args_vec.iter();
    match add_time(args, &mut to_do) {
        Ok(response) => {
            assert!(false, "Expected Err Actual {}", response);
        },
        Err(response) => {
            let expected_response = format!("Time to be added to the task, \"{}\" ,is not in the expected format. \"{}\"", time_argument, recommend_help());
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        }
    };
}

#[test]
fn check_remove_time() {
    let mut to_do: ToDo = ToDo::new();

    let task_name = format!("task510");
    let label_1 = format!("label_1");
    let label_2 = format!("label_2");
    let args_vec = vec![task_name.clone(), label_1.clone(), label_2.clone()];
    let args = args_vec.iter();
    create_new_task_from_arguments(args, &mut to_do);

    let date_time = Local::now().naive_local();
    let current_date = NaiveDate::from_ymd(date_time.year(), date_time.month(), date_time.day()).as_dmy();

    let time_argument = format!("01:05");
    let args_vec = vec![task_name.clone(), time_argument];
    let args = args_vec.iter();
    add_time(args, &mut to_do);

    let time_argument = format!("00:05");
    let args_vec = vec![task_name.clone(), time_argument];
    let args = args_vec.iter();
    match remove_time(args, &mut to_do) {
        Ok(response) => {
            let expected_response = format!("Time spent for task \"{}\" on date \"{}\" is now \"01:00:00\"", task_name.clone(), current_date);
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        }
        Err(response) => {
            assert!(false, response);
        }
    };

    let time_argument = format!("01:05");
    let args_vec = vec![task_name.clone(), time_argument];
    let args = args_vec.iter();
    match remove_time(args, &mut to_do) {
        Ok(response) => {
            let expected_response = format!("Time spent for task \"{}\" on date \"{}\" is now \"00:00:00\"", task_name.clone(), current_date);
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        }
        Err(response) => {
            assert!(false, response);
        }
    };

    let time_argument = format!("0101");
    let args_vec = vec![task_name.clone(), time_argument.clone()];
    let args = args_vec.iter();
    match remove_time(args, &mut to_do) {
        Ok(response) => {
            assert!(false, "Expected Err Actual {}", response);
        }
        Err(response) => {
            let expected_response = format!("Time to be added to the task, \"{}\" ,is not in the expected format. \"{}\"", time_argument, recommend_help());
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        }
    };

    let time_argument = format!("0a:01");
    let args_vec = vec![task_name.clone(), time_argument.clone()];
    let args = args_vec.iter();
    match remove_time(args, &mut to_do) {
        Ok(response) => {
            assert!(false, "Expected Err Actual {}", response);
        }
        Err(response) => {
            let expected_response = format!("Time to be added to the task, \"{}\" ,is not in the expected format. \"{}\"", time_argument, recommend_help());
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        }
    };

    let time_argument = format!("01:o1");
    let args_vec = vec![task_name.clone(), time_argument.clone()];
    let args = args_vec.iter();
    match remove_time(args, &mut to_do) {
        Ok(response) => {
            assert!(false, "Expected Err Actual {}", response);
        }
        Err(response) => {
            let expected_response = format!("Time to be added to the task, \"{}\" ,is not in the expected format. \"{}\"", time_argument, recommend_help());
            assert!(response == expected_response, "Expected {} Actual {}", expected_response, response);
        }
    };
}

#[test]
fn delete_task() {
    let mut todo = ToDo::new();
    let task_name = format!("task1");
    todo.add(task_name.clone(), Vec::new());
    todo.add(format!("task2"), Vec::new());

    let args_vec = vec![task_name.clone()];
    let args = args_vec.iter();
    match delete(args, &mut todo) {
        Ok(message) => {
            let expected_message = format!("Task {} was removed at index 0.", task_name);
            assert!(message == expected_message, "Expected {} Actual {}", expected_message, message);
        },
        Err(message) => {
            assert!(false, message);
        }
    };

    assert!(todo.count() == 1, "Expected number of Tasks 1. Actual {}", todo.count());
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

    let actual_report = todo.to_time_on_labels_report();

    for line in actual_report {
        if label1 == line.label {
            let expected = Duration::new(9, 0).as_hhmmss();
            assert!(line.time_spent == expected, "Label {} Actual {} Expected {}", label1, line.time_spent, expected);
        } else {
            if label2 == line.label {
                let expected = Duration::new(9, 0).as_hhmmss();
                assert!(line.time_spent == expected, "Label {} Actual {} Expected {}", label2, line.time_spent, expected);
            } else {
                if label3 == line.label {
                    let expected = Duration::new(6, 0).as_hhmmss();
                    assert!(line.time_spent == expected, "Label {} Actual {} Expected {}", label3, line.time_spent, expected);
                } else {
                    assert!(false, "Unexpected label \"{}\" was reported", line.label);
                }
            }
        }
    }
}

