# 🦀 Rust CLI Calculator

A simple command-line calculator written in Rust. It started as a practice project to get comfortable with **ownership, functions, tuples and user input**, and it keeps growing one small step at a time.

## ✨ Features

- Interactive menu in the terminal
- Reads the chosen option and two integers from the keyboard
- Dispatches the operation with `match` and returns the result up to `main`
- Operations available:
  - [x] Addition
  - [ ] Subtraction
  - [ ] Multiplication
  - [ ] Division

## 🚀 Getting started

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)

### Run it

```bash
git clone https://github.com/izanestor0x/rust-cli-calculator.git
cd rust-cli-calculator
cargo run
```

## 🖥️ Example

```
<---CALCULATOR--->
What you want to do?
1.- Addition
1
Enter the first number: 5
Enter the second number: 3
The result is 8 !
```

## 🧱 How it works

| Function        | What it does                                                        |
| --------------- | ------------------------------------------------------------------- |
| `main`          | Shows the menu, calls `checker` and prints the result               |
| `show_menu`     | Prints the list of available operations                             |
| `reader`        | Reads a line from the keyboard and parses it into an `i32`          |
| `user_numbers`  | Asks for two numbers and returns them as a tuple `(i32, i32)`       |
| `checker`       | Reads the option, picks the operation with `match` and returns it   |
| `addition`      | Receives two numbers and returns their sum                          |

## 🗺️ Roadmap

- [ ] Add the remaining operations
- [ ] Replace option numbers with an `enum Operation`
- [ ] Handle invalid input and division by zero with `Result` instead of `unwrap`/`panic!`
- [ ] Split the code into modules
- [ ] Add unit tests (`cargo test`)

## 📚 What I'm practicing

`ownership` · `functions and return values` · `tuples` · `match` · `std::io` · `error handling` (next)

## 📄 License

This is a learning project. Feel free to use it as a reference.
