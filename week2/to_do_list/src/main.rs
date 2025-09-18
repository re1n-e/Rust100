use std::fs::File;
use std::io::{self, Read, Write};
use std::{env::args, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

enum ShowTask {
    Completed,
    Incomplete,
    All,
}

fn load_task(file_path: &str) -> Vec<Task> {
    if !Path::new(file_path).exists() {
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(b"[]").expect("Failed to write to new file");
    }
    
    let mut file = File::open(file_path).expect("Failed to open file for reading");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to read from file");

    if buf.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&buf).unwrap_or_else(|_| Vec::new())
    }
}

fn get_input(msg: &str) -> usize {
    println!("\n{msg}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
        .trim()
        .parse::<usize>()
        .expect("Input is not a valid unsigned integer")
}

fn add_task(tasks: &mut Vec<Task>) {
    let task_id = if tasks.is_empty() {
        1
    } else {
        tasks[tasks.len() - 1].id + 1
    };

    println!("\nEnter task description:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read description");

    let description = description.trim().to_string();

    tasks.push(Task {
        id: task_id,
        description,
        completed: false,
    });
    println!("Task added successfully");
}

fn mark_complete(tasks: &mut Vec<Task>) {
    let task_id = get_input("Enter the task id");
    if let Some(task) = tasks.iter_mut().find(|task| task.id == task_id) {
        task.completed = true;
        println!("Task marked as completed");
    } else {
        println!("The task id does not exist");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    let task_id = get_input("Enter the task id");
    if let Some((index, _)) = tasks
        .iter()
        .enumerate()
        .find(|(_, task)| task.id == task_id)
    {
        tasks.remove(index);
        println!("Task deleted successfully");
    } else {
        println!("The task id does not exist");
    }
}

fn write_tasks(tasks: &mut Vec<Task>, file_path: &str) {
    let file = File::create(file_path).expect("Failed to create file for writing");
    serde_json::to_writer_pretty(file, tasks).expect("Failed to write tasks to file");
}

fn show_task(tasks: &Vec<Task>, show: ShowTask) {
    println!("\n id\tDescription\t\t\tCompleted");
    match show {
        ShowTask::All => tasks.iter().for_each(|task| {
            println!(" {}\t{}\t\t\t{}", task.id, task.description, task.completed)
        }),
        ShowTask::Completed => tasks.iter().filter(|task| task.completed).for_each(|task| {
            println!(" {}\t{}\t\t\t{}", task.id, task.description, task.completed)
        }),
        ShowTask::Incomplete => tasks
            .iter()
            .filter(|task| !task.completed)
            .for_each(|task| {
                println!(" {}\t{}\t\t\t{}", task.id, task.description, task.completed)
            }),
    }
}

fn edit_task(tasks: &mut Vec<Task>) {
    let task_id = get_input("Enter the task id to be edited");
    if let Some(task) = tasks.iter_mut().find(|task| task_id == task.id) {
        println!("Enter new description:");
        let mut new_description = String::new();
        io::stdin()
            .read_line(&mut new_description)
            .expect("Can't read new description");
        task.description = String::from(new_description.trim());
        println!("Task updated successfully");
    } else {
        println!("Task id to be edited doesn't exist");
    }
}

fn main() {
    println!("Hello and welcome to the To-Do list app!");
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run -- <filepath>");
        return;
    }
    let file_path = &args[1];
    let mut tasks = load_task(file_path);

    loop {
        println!("\n To-Do List Menu:");
        println!("1. Add Task");
        println!("2. Mark Task as Complete");
        println!("3. Delete Task");
        println!("4. Edit a Task");
        println!("5. Show All Tasks");
        println!("6. Show Completed Tasks");
        println!("7. Show Incomplete Tasks");
        println!("8. Exit");

        let choice = get_input("Please enter your choice");
        match choice {
            1 => add_task(&mut tasks),
            2 => mark_complete(&mut tasks),
            3 => delete_task(&mut tasks),
            4 => edit_task(&mut tasks),
            5 => show_task(&tasks, ShowTask::All),
            6 => show_task(&tasks, ShowTask::Completed),
            7 => show_task(&tasks, ShowTask::Incomplete),
            8 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }

    write_tasks(&mut tasks, file_path);
}