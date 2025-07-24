use crate::task::Task;

pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = tasks.len() + 1;
    let task = Task { id, description, completed: false };
    tasks.push(task);
    println!("Task added.");
}

pub fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("{} [{}] - {}", task.id, if task.completed { "x" } else { " " }, task.description);
    }
}

pub fn mark_done(tasks: &mut Vec<Task>, id: usize) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        println!("Task marked as done.");
    } else {
        println!("Task not found.");
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, id: usize) {
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        println!("Task removed.");
    } else {
        println!("Task not found.");
    }
}
