// STATUS: Complete! To do list now works with file read/write 

use std::io;
use std::process;
use std::fs::File;
use std::io::Write;
use std::fs;

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

fn write_file(tasks: &Vec<Task>)-> std::io::Result<()>{
    let mut file = File::create("tasks.csv")?;
    for t in tasks{
        let line = format!("{},{}\n",t.done,t.description);
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}

fn read_file()-> io::Result<Vec<Task>>{
    let contents = match fs::read_to_string("tasks.csv"){
        Ok(c) => c, //Ok gets the vale from the file and stores in c which gets passed to contents
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    let mut tasks = Vec::new();
    for (i,line) in contents.lines().enumerate(){
        if let Some((done, description)) = line.split_once(',') {
            tasks.push(Task {
                description: description.to_string(),
                done: done == "true",
            });
        } else {
            eprintln!("File corrupted! Bad data at line {}",i+1);
            process::exit(1);
        }
    }
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) {
    if let Err(e) = write_file(tasks){
        eprintln!("error: {e}");
        process::exit(1);
    }
}


fn main() {
    let mut tasks = match read_file(){
        Ok(t) => t,
        Err(e) => {
            eprintln!("error reading tasks: {e}");
            process::exit(1);
        }
    };
    //let _line = format!("{},{}",tasks[0].done, tasks[0].description);
    loop{
        println!("Enter a command");
        println!("add | done <index> | del <index> | list | quit");
        let cmd = read_line();
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        match parts.as_slice() {
        ["add"] =>{ println!("Enter new task");
                        tasks.push(Task{description: read_line(), done: false});
                        save_tasks(&tasks);
                },
        ["done",index] =>{
                match index.parse::<usize>(){
                    Ok(i) => {
                        match tasks.get_mut(i){
                            Some(task) => {
                                task.done = true;
                                println!("Marked task {} as done.",i);
                                save_tasks(&tasks);
                            }
                            None => println!("No task at index {}.",i),
                        }
                    }
                    Err(_) => println!("'{}' is not a valid number.", index),
                }
            },
        ["del",index] => {
                match index.parse::<usize>(){
                    Ok(i) => {
                        if i < tasks.len() {
                            tasks.remove(i);
                            println!("Removed task {}",i);
                            save_tasks(&tasks);
                        } else {
                            println!("No task at index {}",i);
                        }
                    }
                    _ => println!("'{}' is not a valid number.", index),
                }
            },
        ["list"] => print_tasks(&tasks),
        ["quit"] => {
                break;
            },
        _ => println!("Unknown command."),
        }
    }
}
