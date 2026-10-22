// STATUS: Reading and writing plus status update works 
// NEXT_STEP: Update according to next 2 Claude issues

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
    for line in contents.lines(){
        if let Some((done, description)) = line.split_once(',') {
            tasks.push(Task {
                description: description.to_string(),
                done: done == "true",
            });
        }
    }
    Ok(tasks)
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
        ["quit"] => {
                if let Err(e) = write_file(&tasks){
                    eprintln!("error: {e}");
                    process::exit(1);
                }
                break;
            },
        _ => println!("Unknown command."),
        }
    }
}





