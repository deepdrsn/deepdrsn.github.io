use std::io;

fn main() {
    // Display the operation options to the user
    println!("Choose a calculation to perform:");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    // Create a mutable variable to store the user's choice
    let mut choice = String::new();

    // Read the user's choice
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // Convert the input to an integer
    let choice: i32 = choice.trim().parse().expect("Please enter a valid number");

    // Ask the user for two numbers
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the first number:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    
    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    // Convert the inputs to integers
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number");
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number");

    // Perform the chosen operation
    match choice {
        1 => {
            println!("The result of {} + {} is {}", num1, num2, num1 + num2);
        }
        2 => {
            println!("The result of {} - {} is {}", num1, num2, num1 - num2);
        }
        3 => {
            println!("The result of {} * {} is {}", num1, num2, num1 * num2);
        }
        4 => {
            if num2 != 0.0 {
                println!("The result of {} / {} is {}", num1, num2, num1 / num2);
            } else {
                println!("Error: Cannot divide by zero");
            }
        }
        _ => println!("Invalid choice, please enter a number between 1 and 4."),
    }
}
