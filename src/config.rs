use std::env;
use std::error::Error;

pub struct Config {
    pub node_id: u32,
    pub next_node: String,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let node_id: u32 = env::var("NODE_ID")?.parse()?;
        let next_node: String = env::var("NEXT_NODE")?;

        Ok(Config { node_id, next_node })
    }

    pub fn get_next_node(&self) -> &str {
        &self.next_node
    }
}