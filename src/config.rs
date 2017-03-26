use serde_json;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Deserialize)]
pub struct CommandConfig {
    pub token: String,
}

impl CommandConfig {
    pub fn from_path(path: &Path) -> io::Result<CommandConfig> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
