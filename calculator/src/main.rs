use std::io;

fn main() {
    
    let num1 = get_number("Please enter the first number: ".to_string());
    let operator = get_operator();
    let num2 = get_number("Please enter the second number: ".to_string());

    match operator {
        '+' => println!("{} + {} = {}", num1, num2, num1 + num2),
        '-' => println!("{} - {} = {}", num1, num2, num1 - num2),
        '*' => println!("{} * {} = {}", num1, num2, num1 * num2),
        '/' => {
            if num2 == 0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            println!("{} / {} = {}", num1, num2, num1 / num2);
        },
        _ => {
            println!("Error: Invalid operator. Please use +, -, *, or /.");
            return;
        }
        
    }
    
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut String::new())
        .expect("Failed to read line");

}

fn get_number(message: String) -> i32{
    
    println!("{}", message);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
               .expect("Failed to read line");

    let number = input.trim().parse()
                            .expect("Please enter a valid number");
    return number;
}

fn get_operator() -> char {
    println!("Please enter an operator (+, -, *, /): ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
               .expect("Failed to read line");

    let operator = input.trim().chars().next()
                            .expect("Please enter a valid operator");
    return operator;
}
