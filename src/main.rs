use std::io::{self, Write};

struct App {
    counter: i32,
}

impl App {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn run(&mut self) {
        println!("Welcome to the CLI Counter App!");
        println!("Commands: left, right, show, help, quit");

        loop {
            print!("> ");
            io::stdout().flush().unwrap(); // Ensure prompt is printed

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read input.");
                continue;
            }

            let command = input.trim().to_lowercase();
            match command.as_str() {
                "left" => {
                    self.counter -= 1;
                    println!("Counter decreased to {}", self.counter);
                }
                "right" => {
                    self.counter += 1;
                    println!("Counter increased to {}", self.counter);
                }
                "show" => {
                    println!("Current counter value: {}", self.counter);
                }
                "help" => {
                    println!("Available commands:\n  left  - decrement\n  right - increment\n  show  - display counter\n  help  - show this message\n  quit  - exit the app");
                }
                "quit" | "exit" => {
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("Unknown command. Type 'help' for available commands.");
                }
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}
