use super::ToDo;

#[test]
fn id_is_unique () {
    let mut todo = ToDo::new();
    todo.add(format!("task1"), Vec::new());
    todo.add(format!("task1"), Vec::new());
    todo.add(format!("task1"), Vec::new());
    todo.add(format!("task1"), Vec::new());

    let mut task_count =0;
    for mut t in todo.to_report() {
        task_count +=1;
        let actual_id = t.remove(0);
        let expected_id = format!("{}",task_count);
        assert!(actual_id==expected_id,format!("Actual id {} Expected id {}",actual_id,expected_id));
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
fn reterive_from_storage () {
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

    match ToDo::load(None) {
        Ok(mut reterived_data) => {
            let mut actual_task_name: String = format!("");

            if let Some(task) = reterived_data.to_simple_report().get(0) {
                match task.get(0) {
                    Some(task_name) => { actual_task_name = task_name.clone().to_string() },
                    _ => { assert!(false, "Task name could not be reterived from the loaded data."); }
                }

            }
            else { assert!(false,"No task could not be reterived from the loaded data."); }

            assert!(task_name==actual_task_name,format!("Expected first task named {} actual name {}",task_name,actual_task_name));
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
        Err(message) => { assert!(true,format!("Adding another tasks with name {} was rejected",task_name)); },
    };
}