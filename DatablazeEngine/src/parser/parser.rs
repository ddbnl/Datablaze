use std::collections::VecDeque;
use crate::commands::command::Command;
use crate::commands::server_commands::{CreateDatabaseCommand, GetDatabaseCommand, StopServerCommand};
use crate::DatastoreVariants;
use crate::parser::errors::*;

pub fn parse_command(string: &mut str) -> Result<Box<dyn Command + Send>, ParseError> {
    
    let lower = string.to_lowercase();
    let mut tokens: VecDeque<&str> = lower.split_whitespace().collect();
    
    let first = VecDeque::pop_front(&mut tokens);
    if let Some(token) = first {
        match token {
            "create" => parse_create_command(tokens),
            "get" => parse_get_command(tokens),
            "stop" => parse_stop_command(tokens),
            _ => Err(ParseError { message: "Invalid base command".to_string() }),
        }
    }
    else {
        Err(ParseError { message: "Missing base command".to_string() })
    } 
}

fn parse_stop_command(_tokens: VecDeque<&str>) -> Result<Box<dyn Command + Send>, ParseError> {
    Ok(Box::new(StopServerCommand))
}

fn parse_create_command(mut tokens: VecDeque<&str>) -> Result<Box<dyn Command + Send>, ParseError> {

    let first = VecDeque::pop_front(&mut tokens);
    if let Some(token) = first {
        match token {
            "database" => parse_create_database_command(tokens),
            _ => Err(ParseError { message: "Invalid create sub command".to_string() }),
        }
    }
    else {
        Err(ParseError { message: "Missing create sub command".to_string() })
    }
}


fn parse_create_database_command(mut tokens: VecDeque<&str>) -> Result<Box<dyn Command + Send>, ParseError> {

    let maybe_variant = VecDeque::pop_front(&mut tokens);
    let maybe_name = VecDeque::pop_front(&mut tokens);
    
    let variant = if let Some(variant) = maybe_variant {
        match variant {
            "in_memory" => DatastoreVariants::InMemory,
            "file_based" => DatastoreVariants::FileBased,
            _ => return Err(ParseError { message: "Invalid database variant".to_string() })
        }
    }
    else {
        return Err(ParseError { message: "Missing database name".to_string() })
    };
    
    if let Some(name) = maybe_name {
        let command = CreateDatabaseCommand { name: name.to_string(), datastore_variant: variant };
        Ok(Box::new(command))
    }
    else {
        Err(ParseError { message: "Missing database name".to_string() })
    }
}

fn parse_get_command(mut tokens: VecDeque<&str>) -> Result<Box<dyn Command + Send>, ParseError> {

    let first = VecDeque::pop_front(&mut tokens);
    if let Some(token) = first {
        match token {
            "database" => parse_get_database_command(tokens),
            _ => Err(ParseError { message: "Invalid get sub command".to_string() }),
        }
    }
    else {
        Err(ParseError { message: "Missing get sub command".to_string() })
    }
}

fn parse_get_database_command(mut tokens: VecDeque<&str>) -> Result<Box<dyn Command + Send>, ParseError> {

    let maybe_name = VecDeque::pop_front(&mut tokens);
    if let Some(name) = maybe_name {
        let command = GetDatabaseCommand { name: name.to_string() };
        Ok(Box::new(command))
    }
    else {
        Err(ParseError { message: "Missing database name".to_string() })
    }
}
