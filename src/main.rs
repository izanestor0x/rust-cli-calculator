use std::io::{self, Write};

fn main() {

    //show main menu, with options
    show_menu();

    //program reads the 2 numbers the user wants to evalue and the option that the user wants
    //checker does a match to whatever is the number call a different function, does the operation and returns it
    //checker has the value of the operation
    let result = checker();
    println!("The result is {}.", result )
}

fn show_menu() {
    println!("<---CALCULATOR--->");
    println!("What you want to do?");
    println!("1.- Addition");
    println!("2.- Subtraction");
    println!("3.- Multiplication");
    println!("4.- Division");
}

fn reader() -> i32{
    //Input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    number
}

fn checker() -> i32 {
    let option = reader();
    let (user_number1, user_number2) = user_numbers();

    let result = match option {
        1 => addition(user_number1, user_number2),
        2 => substraction(user_number1, user_number2),
        3 => multiplication(user_number1, user_number2),
        4 => division(user_number1, user_number2),
        _ => panic!("Invalid option"),
    };
    result
}

fn user_numbers() -> (i32, i32) {
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let user_number1 = reader();
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let user_number2 = reader();
    (user_number1, user_number2)
}


fn addition(a: i32, b:i32) -> i32{
    a + b
}
fn substraction (a: i32, b:i32) -> i32 {
    a-b
}
fn multiplication (a: i32, b:i32) -> i32 {
    a*b
}
fn division (a: i32, b:i32) -> i32 {
    a/b
}