use std::io::{self, Write};

pub struct DebugCLI;

impl DebugCLI {
    pub fn run() {
        println!("SPL Debug CLI");
        println!("Type 'exit' to quit");

        loop {
            print!("spl> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let cmd = input.trim();

            match cmd {
                "exit" => break,
                "status" => println!("System running"),
                "help" => println!("Commands: status, help, exit"),
                _ => println!("Unknown command: {}", cmd),
            }
        }
    }
}
