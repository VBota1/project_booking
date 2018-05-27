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
/*
#[test]
TODO add fn help() {
    assert!(false, "Test for Help to be implemented!");
}

#[test]
TODO add fn license() {
    assert!(false, "Test for License to be implemented!");
}
*/