use std::io::{ self, Write };
fn main() {
    loop {
        //show main menu, with options
        show_menu();

        let option = read_option();

        //program reads the 2 numbers the user wants to evalue and the option that the user wants
        //calculate does a match to whatever is the number call a different function, does the operation and returns it
        let (user_number1, user_number2) = read_two_numbers();

        //calculate has the value of the operation
        let result = calculate(option, user_number1, user_number2);

        show_result(result);
        println!();
    }
}

fn show_menu() {
    println!("<---CALCULATOR--->");
    println!("1.- Addition");
    println!("2.- Subtraction");
    println!("3.- Multiplication");
    println!("4.- Division");
    println!("0.- Exit");
    println!("")
}

fn read_number() -> i32 {
    //Input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    number
}

fn calculate(option: i32, a: i32, b:i32) -> i32 {

    let result = match option {
        1 => add(a, b),
        2 => subtract(a, b),
        3 => multiply(a, b),
        4 => divide(a, b),
        _ => panic!("Invalid option"),
    };
    result
}

fn read_two_numbers() -> (i32, i32) {
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let user_number1 = read_number();
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let user_number2 = read_number();
    (user_number1, user_number2)
}

fn read_option() -> i32 {
    print!("What you want to do? ");
    io::stdout().flush().unwrap();
    let option = read_number();
    option
}

fn show_result(result: i32) -> i32 {
    println!("The result is {result}");
    println!("");
    result
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn divide(a: i32, b: i32) -> i32 {
    a / b
}
