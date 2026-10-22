use std::io;

struct Task { 
    description: String,
    done: bool,
}

fn print_tasks(tasks: &Vec<Task>){
    if tasks.is_empty(){
        println!("No tasks yet.");
        return;
    }
    for task in tasks{
        let status = if task.done {"x"} else {" "};
        println!("[{}]{}",status, task.description);
    }
}

fn read_line()->String{
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop{
        println!("\nCommands: add | done <index> | remove <index> | list | quit");
        let command = read_line();
        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.as_slice() {
            ["add"] => {
                println!("Enter task description:");
                let description = read_line();
                tasks.push(Task{description, done: false});
                println!("Task added.");
            }
            ["done", index] => {
                match index.parse::<usize>() {
                    Ok(i) => match tasks.get_mut(i) {
                        Some(task) => {
                            task.done = true;
                            println!("Marked task {} as done.",i);
                        }
                        None => println!("No task at index {}.",i),
                    },
                    Err(_) => println!("'{}' is not a valid number.", index),
                }
            }
            ["remove",index] => {
                match index.parse::<usize>(){
                    Ok(i) if i < tasks.len() => {
                        tasks.remove(i);
                        println!("Removed tasks {}.",i);
                    }
                    _ => println!("Invalid index."),
                }
            }
            ["list"] => print_tasks(&tasks),
            ["quit"] => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command."),
        }
    }

}


