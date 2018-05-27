use super::ToDo;
use super::*;

#[test]
fn control_1_task_as_service() {
    let task_name = format!("task510");
    let label_1 = format!("label_1");
    let label_2 = format!("label_2");
    let args_vec = vec![task_name, label_1, label_2];
    let args = args_vec.iter();
    //load
    let mut to_do: ToDo = get_to_do(None);
    //create
    create_new_task_from_arguments(args);
    //save
    //load
    let mut to_do: ToDo = get_to_do(None);
    //clock in
    //save
    //wait
    //load
    let mut to_do: ToDo = get_to_do(None);
    //clock out
    //save
    //load
    let mut to_do: ToDo = get_to_do(None);
    //report
    //save
}

#[test]
fn control_1_task_as_aplication() {
    //load
    //create
    //clock in
    //wait
    //clock out
    //report
    //save
}

#[test]
fn help() {
    assert!(false, "Test for Help to be implemented!");
}

#[test]
fn license() {
    assert!(false, "Test for License to be implemented!");
}
