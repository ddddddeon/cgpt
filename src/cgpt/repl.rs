use super::client::Client;
use std::io::{self, Write};

pub struct Repl {
    running: bool,
}

impl Repl {
    pub fn new() -> Self {
        Self { running: false }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = Client::new();
        client.connect()?;
        self.running = true;

        let mut input = String::new();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            if let Err(e) = io::stdin().read_line(&mut input) {
                println!("Could not read from stdin: {e}");
                return Err(e.into());
            }

            print!("You entered: {input}");
            // TODO save message history
            input.clear();
        }

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
