
struct Task {
    description: String,
    completed: bool,
}

fn main() {

    let mut tasks: Vec<Task> = Vec::new();
    add_task(&mut tasks);

    println!("Your tasks: {}", tasks.get(0).unwrap().description);
 
}

fn add_task(tasks:&mut Vec<Task>) -> (){
    
    println!("Enter the task:");
    let mut task_description = String::new();
    std::io::stdin().read_line(&mut task_description).expect("Failed to read line");
    let task = Task {
        description: task_description.trim().to_string(),
        completed: false,
    };

    tasks.push(task);

}
