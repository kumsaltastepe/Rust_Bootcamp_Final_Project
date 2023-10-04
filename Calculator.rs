use std::io::{self, Write};

// Define the Operation enum
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Function to perform calculation
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    // Prompt the user to input the first number
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap();

    // Prompt the user to input the operation
    print!("Enter the operation (Add, Subtract, Multiply, Divide): ");
    io::stdout().flush().unwrap();
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).unwrap();
    let operation = operation.trim();

    // Prompt the user to input the second number
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();

    // Create an Operation enum instance with the parsed input values
    let op = match operation {
        "Add" => Operation::Add(num1, num2),
        "Subtract" => Operation::Subtract(num1, num2),
        "Multiply" => Operation::Multiply(num1, num2),
        "Divide" => Operation::Divide(num1, num2),
        _ => panic!("Invalid operation!"),
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(op);

    // Print the result to the console
    println!("Result: {}", result);
}
