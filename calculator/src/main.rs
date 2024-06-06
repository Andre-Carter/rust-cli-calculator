//import io from standard library
use std::io::{stdin, stdout, Write};
fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}
fn main() {
    println!("Welcome to CLI Calculator!");
    loop {
        let mut num1: String = String::new();
        let mut num2: String = String::new();
        let mut operator: String = String::new();
        
        print!("What is your first number?: ");
        read(&mut num1);
        print!("Operation (+, -, *, /, %, ^): ");
        read(&mut operator);
        print!("What is your second number?: ");
        read(&mut num2);
        
        let num1: f32 = num1.trim().parse().unwrap(); 
        let num2: f32 = num2.trim().parse().unwrap(); 
        let operator: char = operator.trim().chars().next().unwrap();
        
        let operators = String::from("+-*/%^");
        
        if !operators.contains(operator) {
            println!("unknown operator");
            continue;
        }
        
        let solution = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            '%' => num1 % num2,
            '^' => num1.powf(num2),
            _ => panic!("error in operator")
        };
        
        println!("Solution: {} {} {} = {} <---", num1, operator, num2, solution);
    }   
}
    