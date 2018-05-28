use super::ToDo;
use super::*;
use std::time::Duration;
use std::thread::sleep;
use super::formaters::AsHHMMSS;

#[test]
fn test_handle_command_as_service() {
    let to_do: ToDo = ToDo::new();
    forced_store(to_do);

    let aplication_name = format!("test");
    let command = format!("new");
    let task_name = format!("task512");
    let label_1 = format!("label_1");
    let label_2 = format!("label_2");
    let args_vec = vec![aplication_name.clone(), command, task_name.clone(), label_1.clone(), label_2.clone()];
    handle_command_as_service(args_vec);

    let command = format!("clockIn");
    let args_vec = vec![aplication_name.clone(), command, task_name.clone()];
    handle_command_as_service(args_vec);

    let actual_time_spent_on_task = Duration::new(5, 0);
    sleep(actual_time_spent_on_task);

    let command = format!("clockOut");
    let args_vec = vec![aplication_name.clone(), command, task_name.clone()];
    handle_command_as_service(args_vec);

    let command = format!("report");
    let args_vec = vec![aplication_name.clone(), command, task_name.clone()];
    let actual_report = handle_command_as_service(args_vec);

    let expected_report = format!("1 {} {} None {} {}", task_name, actual_time_spent_on_task.as_hhmmss(), label_1, label_2);

    assert!(actual_report == expected_report.clone(), "Actual report \"{}\" Expected report \"{}\"", actual_report, expected_report.clone());
}

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

    let expected_report = format!("1 {} {} None {} {}", task_name, actual_time_spent_on_task.as_hhmmss(), label_1, label_2);

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

    let expected_report = format!("1 {} {} None {} {}", task_name, actual_time_spent_on_task.as_hhmmss(), label_1, label_2);

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

    let time_argument = format!("01:01");
    let args_vec = vec![task_name.clone(), time_argument];
    let args = args_vec.iter();
    match add_time(args, &mut to_do) {
        Ok(response) => {
            let expected_response = format!("Time spent on task \"{}\" is now \"01:01:00\"", task_name.clone());
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
fn delete_task() {
    let mut todo = ToDo::new();
    let task_name = format!("task1");
    todo.add(task_name.clone(), Vec::new());
    todo.add(format!("task2"), Vec::new());

    let args_vec = vec![task_name.clone()];
    let args = args_vec.iter();
    match delete(args, &mut todo) {
        Ok(message) => {
            let expected_message = format!("Task {} was removed.", task_name);
            assert!(message == expected_message, "Expected {} Actual {}", expected_message, message);
        },
        Err(message) => {
            assert!(false, message);
        }
    };

    assert!(todo.count() == 1, "Expected numer of Tasks 1. Actual {}", todo.count());
}