use std::io::{self, Write};

struct App {
    counter: i32,
}

impl App {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn handle_input(&mut self, input: &str) {
        match input.trim() {
            "left" => {
                self.counter -= 1;
                println!("Counter decreased to {}", self.counter);
            }
            "right" => {
                self.counter += 1;
                println!("Counter increased to {}", self.counter);
            }
            "exit" | "quit" => {
                println!("Exiting...");
                std::process::exit(0);
            }
            "help" => {
                println!("Available commands: left, right, help, quit");
            }
            _ => {
                println!("Unknown command. Type 'help' for available commands.");
            }
        }
    }

    fn run(&mut self) {
        println!("Welcome to the CLI Engine!");
        println!("Type 'help' to see available commands.");

        loop {
            print!("> ");
            io::stdout().flush().unwrap(); // Flush prompt

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                self.handle_input(&input);
            } else {
                println!("Error reading input. Try again.");
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}
