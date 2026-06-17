use std::io;

fn main() {
    let banner = "----------------------------------------"; // not mut because we don't need to change it, only print it.
    println!("{}", banner);
    println!("Welcome to: Calculator");
    println!("Developed by: Smite @ Dopamine Studios. Just a fun little first project to learn Rust lol");
    println!("{}", banner);
    println!("Enter your first number>>");

    let mut input1 = String::new();
    let mut operator = String::new();
    let mut input2 = String::new();

    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read your line!");

    println!("Enter the operator>>");

    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read your line!");

    println!("Enter your second number>>");

    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read your line!");

    let num1: f64 = input1.trim().parse().expect("Invalid number!");
    let num2: f64 = input2.trim().parse().expect("Invalid number!");

    let op = operator.trim();

    println!("{}", banner);

    match op {
        "+" => println!("{} + {} = {}", num1, num2, num1 + num2),
        "-" => println!("{} - {} = {}", num1, num2, num1 - num2),
        "*" => println!("{} * {} = {}", num1, num2, num1 * num2),
        "/" => {
            if num2 == 0.0 {println!("Dividing by zero is a war crime!");
            }
            else {println!("{} / {} = {}", num1, num2, num1 / num2)}
        },
        _ => println!("{} is not a valid operator!", op)
    }
    println!("{}", banner);


}