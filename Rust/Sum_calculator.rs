use std::io;

fn main() {
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

    let num1: i32 = input1.trim().parse().expect("Please enter a valid number");
    let num2: i32 = input2.trim().parse().expect("Please enter a valid number");

    let sum = num1 + num2;
    println!("The sum of {} and {} is {}", num1, num2, sum);
}
