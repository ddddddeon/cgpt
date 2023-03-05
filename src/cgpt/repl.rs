use super::client::Client;

pub struct Repl {
    name: String,
    running: bool,
}

impl Repl {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = Client::new();
        client.connect()?;
        self.running = true;

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
