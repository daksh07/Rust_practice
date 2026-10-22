use std::io;

struct Task {
    description: String,
    done: bool,
}

fn print_tasks(tasks: &Vec<Task>){
    for task in tasks{
        println!("[{}]{}",if task.done {"x"}else{" "},task.description);
    }
}

fn read_line()-> String{
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

fn main() {
    let mut tasks =vec![Task{description: ("Go to chemist").to_string(), done: false},Task{description: ("Do coding").to_string(), done: true}];
    loop{
        println!("Enter a command");
        println!("add | done <index> | del <index> | list | quit");
        let cmd = read_line();
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        match parts.as_slice() {
        ["add"] =>{ println!("Enter new task");
                        tasks.push(Task{description: read_line(), done: false});
                },
        ["done",index] =>{
                match index.parse::<usize>(){
                    Ok(i) => {
                        match tasks.get_mut(i){
                            Some(task) => {
                                task.done = true;
                                println!("Marked task {} as done.",i);
                            }
                            None => println!("No task at index {}.",i),
                        }
                    }
                    Err(_) => println!("'{}' is not a valid number.", index),
                }
            },
        ["del",index] => {
                match index.parse::<usize>(){
                    Ok(i) if i < tasks.len() => {
                        tasks.remove(i);
                        println!("Removed task {}",i);
                    }
                    _ => println!("'{}' is not a valid number.", index),
                }
            },
        ["list"] => print_tasks(&tasks),
        ["quit"] => break,
        _ => println!("Unknown command."),
        }
    }
}





