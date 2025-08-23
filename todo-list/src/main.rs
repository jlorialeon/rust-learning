use std::io::{self, stdin};

struct Task {
    description: String,
    completed: bool,
}

fn main() {

    let mut tasks: Vec<Task> = Vec::new();

    loop {
        let selected_option = main_menu();

        match  selected_option {
            1 => print_taks_list(&mut tasks),
            2 => add_task(&mut tasks),
            3 => set_task_as_completed(&mut tasks),
            4 => delete_task_as_completed(&mut tasks),
            5 => break,
            _ => continue
        }
    }
}

fn main_menu() -> i32{

    println!("\n\nPlease select an option");
    println!("1 - Show all tasks");
    println!("2 - Add new task");
    println!("3 - Set task as completed");
    println!("4 - Delete task");
    println!("5 - Exit");

    let mut option = String::new();

    io::stdin().read_line(&mut option)
               .expect("There was an error reading the option");

    let option_num:i32 = option.trim().parse()
                                      .expect("Invalid number");

    return option_num;
}

fn print_taks_list(tasks: &Vec<Task>)->(){

    if tasks.len() == 0{
        println!("\nNo Tasks");
        return;
    }

    let mut task_num = 1;

    let mut completed_status:String;

     println!("\nTasks List\n\n");

    for task in tasks.iter(){

        if task.completed {
            completed_status = String::from("Completed    ");
        }
        else{
            completed_status = String::from("Not Completed");
        }

        println!("{} - Status {} - {}", task_num.to_string(), completed_status, task.description);
        task_num += 1;
    }

}

fn add_task(tasks:&mut Vec<Task>) -> (){
    
    println!("\n\nEnter the task:");
    let mut task_description = String::new();
    std::io::stdin().read_line(&mut task_description).expect("Failed to read line");
    let task = Task {
        description: task_description.trim().to_string(),
        completed: false,
    };

    tasks.push(task);

}

fn set_task_as_completed(tasks:&mut Vec<Task>) -> (){

    loop {
        
        let task_num = print_taks(&tasks);

        if task_num == 0{
            return;
        }

        let mut input = String::new();

        println!("\nPlease enter the task number to set as completed or press 0 to return to the main menu");
        input.clear();
        stdin().read_line(&mut input)
                .expect("There was an reading the option");

        let option: i32 = input.trim().parse().expect("Invalid menu option");

        if option < 0 || option > task_num {
            println!("Invalid option");
            continue;
        }

        if option == 0{
            break;
        }

        let task = tasks.get_mut((option - 1) as usize);
        match task {
            Some(task) => {
                task.completed = true;
                println!("Task {} updated!!!", option);
                return;
            }
            None => println!("Task not found")
        }
    }

    return;

}

fn delete_task_as_completed(tasks:&mut Vec<Task>) -> (){

    loop {
        
        let task_num = print_taks(&tasks);

        if task_num == 0{
            return;
        }

        let mut input = String::new();

        println!("\nPlease enter the task number you want to delete or press 0 to return to the main menu");
        input.clear();
        stdin().read_line(&mut input)
                .expect("There was an reading the option");

        let option: i32 = input.trim().parse().expect("Invalid menu option");

        if option < 0 || option > task_num {
            println!("Invalid option");
            continue;
        }

        if option == 0{
            break;
        }

        tasks.remove((option - 1) as usize);
        println!("Task {} deleted!!!!",option);
        break;
    }

    return;

}

fn print_taks(tasks:&Vec<Task>) -> i32{

    if tasks.len() == 0 {
        println!("\nNo Tasks");
        return 0;
    }

    let mut task_num = 0;
    let mut completed_status:String;

    println!("\nTasks List\n\n");

    for task in tasks.iter(){

        task_num += 1;

        if task.completed {
            completed_status = String::from("Completed    ");
        }
        else{
            completed_status = String::from("Not Completed");
        }

        println!("{} - Status {} - {}", task_num.to_string(), completed_status, task.description);
    }

    return  task_num;
}