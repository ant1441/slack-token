use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Commands {
    List,
    Get,
    Drop,
    AfterYou,
    Barge,
    Steal,
}

use super::Commands::*;

impl FromStr for Commands {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "list" => Ok(List),
            "get" => Ok(Get),
            "drop" => Ok(Drop),
            "afteryou" => Ok(AfterYou),
            "barge" => Ok(Barge),
            "steal" => Ok(Steal),
            _ => Err("invalid command"),
        }
    }
}
