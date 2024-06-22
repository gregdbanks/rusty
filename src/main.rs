use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Complete task");
        println!("4. Delete task");
        println!("5. Save and exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => add_task(&mut tasks),
            2 => view_tasks(&tasks),
            3 => complete_task(&mut tasks),
            4 => delete_task(&mut tasks),
            5 => {
                save_tasks(&tasks);
                break;
            }
            _ => println!("Invalid choice! Please enter a number between 1 and 5."),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    print!("Enter the task description: ");
    io::stdout().flush().unwrap();

    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    let description = description.trim().to_string();

    let task = Task::new(description);
    tasks.push(task);

    println!("Task added!");
}

fn view_tasks(tasks: &[Task]) {
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "completed" } else { "pending" };
        println!("{}. {} [{}]", index + 1, task.description, status);
    }
}

fn complete_task(tasks: &mut Vec<Task>) {
    print!("Enter the task number to complete: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let index: usize = input.trim().parse().unwrap_or(0);

    if index == 0 || index > tasks.len() {
        println!("Invalid task number!");
        return;
    }

    tasks[index - 1].completed = true;
    println!("Task marked as completed!");
}

fn delete_task(tasks: &mut Vec<Task>) {
    print!("Enter the task number to delete: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let index: usize = input.trim().parse().unwrap_or(0);

    if index == 0 || index > tasks.len() {
        println!("Invalid task number!");
        return;
    }

    tasks.remove(index - 1);
    println!("Task deleted!");
}

fn save_tasks(tasks: &[Task]) {
    let serialized = serde_json::to_string(tasks).unwrap();
    fs::write("tasks.json", serialized).unwrap();
    println!("Tasks saved!");
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}
