const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct Client {
    pub url: String,
    connected: bool,
}

impl Client {
    pub fn new() -> Self {
        Self {
            url: OPENAI_API_URL.to_string(),
            connected: false,
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.connected = true;

        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
