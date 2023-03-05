use super::client::Client;
use std::io::{self, Write};

#[derive(Default)]
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

            if &input == "exit\n" {
                return Ok(());
            }

            print!("You entered: {input}");
            // TODO save message history
            input.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_repl() {
        let repl = Repl::new();
        assert!(!repl.running);
    }
}
