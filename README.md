### Prerequisites:

1. **Install Rust and Cargo:**

   - If you haven't installed Rust and Cargo, you can do so by running:
     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

2. **Verify Installation:**
   - After installation, ensure that Rust and Cargo are installed correctly by running:
     ```sh
     rustc --version
     cargo --version
     ```

### Create and Run the Project:

1. **Create a New Rust Project:**

   - Create a new project using Cargo:
     ```sh
     cargo new todo_cli
     cd todo_cli
     ```

2. **Add Dependencies:**

   - Open `Cargo.toml` and add the required dependencies:
     ```toml
     [dependencies]
     serde = { version = "1.0", features = ["derive"] }
     serde_json = "1.0"
     ```

3. **Write the Code:**

   - Replace the contents of `src/main.rs` with the provided code:

     ```rust
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
     ```

4. **Run the Project:**
   - Build and run your Rust project using Cargo:
     ```sh
     cargo run
     ```

### Summary:

- **Install Rust and Cargo:** Ensure Rust and Cargo are installed.
- **Create a New Project:** Use Cargo to create a new Rust project.
- **Add Dependencies:** Update `Cargo.toml` with necessary dependencies.
- **Write Code:** Implement the to-do list application in `main.rs`.
- **Run the Project:** Build and run the project with `cargo run`.

This should set up your environment correctly and allow you to run the to-do list application. If you encounter any specific errors, feel free to ask for further assistance!
