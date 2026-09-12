use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    let operations = ['+', '-', '/', '*'];

    println!("Enter the first number: ");
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read first number");

    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read second number");

    let num1: f64 = num1.trim().parse().expect("Invalid first number");
    let num2: f64 = num2.trim().parse().expect("Invalid second number");

    println!("Both numbers read successfully!");

    println!("Enter an operation (+, -, /, *):");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read second number");

    let operation = operation.trim().chars().next().unwrap();

    if operations.contains(&operation) {
        println!("Valid operation: {}", operation);
    } else {
        println!("Invalid operation!");
    }

    let result = match operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => unreachable!(),
    };

    println!("Result: {}", result);
}
