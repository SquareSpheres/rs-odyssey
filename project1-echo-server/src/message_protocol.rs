use std::error::Error;
use std::fmt;

pub enum Command {
    Exit,
    Message(String),
}

#[derive(Debug)]
pub enum ParseError {
    EmptyLine,
    UnknownCommand(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyLine => write!(f, "Input was empty"),
            ParseError::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
        }
    }
}
impl Error for ParseError {}

impl Command {
    pub fn parse(line: &str) -> Result<Command, ParseError> {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            return Err(ParseError::EmptyLine);
        }

        let trimmed_lower = trimmed.to_lowercase();

        if trimmed_lower.starts_with("message:") {
            // Remove the prefix case-insensitively
            let (_, message) = trimmed
                .split_once(':')
                .ok_or_else(|| ParseError::UnknownCommand(trimmed.to_string()))?;
            return Ok(Command::Message(message.to_string()));
        }

        if trimmed_lower == "exit" {
            return Ok(Command::Exit);
        }

        Err(ParseError::UnknownCommand(trimmed.to_string()))
    }
}
