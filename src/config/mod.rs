use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod env_parser;
mod errors;

const CONFIG_FILE_NAME: &str = ".env";

pub struct Config {
    node_id: u32,
    next_node: String,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        // open the .env file
        let env_file = File::open(CONFIG_FILE_NAME)?;
        let mut buf_reader = BufReader::new(env_file);
        let mut env_contents = String::new();
        // Read the contents of the buffer to a string
        buf_reader.read_to_string(&mut env_contents)?;

        // parse the contents into a Config struct
        let node_id = match env_parser::EnvParser::parse_node_id(&env_contents) {
            Some(number) => number as u32,
            None => return Err(Box::new(errors::ConfigParseError)),
        };

        let next_node = match env_parser::EnvParser::parse_next_node(&env_contents) {
            Some(string) => string,
            None => return Err(Box::new(errors::ConfigParseError)),
        };

        Ok(Config { node_id, next_node })
    }

    pub fn get_node_id(&self) -> &u32 {
        &self.node_id
    }

    pub fn get_next_node(&self) -> &str {
        &self.next_node
    }
}
