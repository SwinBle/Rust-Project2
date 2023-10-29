use std::io;

// Define an enum called Operation with variants Add, Subtract, Multiply, and Divide
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Create a function called calculate that takes an Operation enum as an argument and returns an f64 result
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b == 0.0 {
                println!("Error: Division by zero.");
                f64::NAN
            } else {
                a / b
            }
        }
    }
}

fn main() {
    // Prompt the user to input the first number, the operation, and the second number
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("Invalid input");

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation = input.trim();

    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let num2: f64 = input2.trim().parse().expect("Invalid input");

    // Create an Operation enum instance with the parsed input values
    let op = match operation {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(op);

    // Print the result to the console
    println!("Result: {}", result);
}
