
use crate::error::ParseError;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Command(String),
    Flag(String),
    Argument(String)
}

impl Token {

    pub fn get_value(&self) -> String {

        match self {
            Token::Command(value) => value.to_string(),
            Token::Flag(value) => value.to_string(),
            Token::Argument(value) => value.to_string()
        }
    }

    pub fn is_argument(&self) -> bool {

        match self {
            Token::Argument(_) => true,
            _ => false
        }
    }

    pub fn is_flag(&self) -> bool {

        match self {
            Token::Flag(_) => true,
            _ => false
        }
    }
}

pub type ParsedCommand = (Token, Option<Token>, Vec<Token>);

pub fn tokenize(input: &String) -> Vec<Token> {

    let mut tokens = Vec::new();

    /*
    Format the input
    Remove posible long whitespaces and collect as Vec<&str>
    */

    let binding = input.split_whitespace().collect::<Vec<&str>>().join(" ");
    let mut parsed_input = binding.split_whitespace().collect::<Vec<&str>>();

    tokens.push(Token::Command(parsed_input[0].to_string()));
    parsed_input.remove(0);

    for token in parsed_input {

        match token.starts_with("-") {
            true  => tokens.push(Token::Flag(token.to_string().replace("-", ""))),
            false => tokens.push(Token::Argument(token.to_string()))
        }
    }

    tokens
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<ParsedCommand, ParseError> {

    let command = tokens[0].clone();
    let mut flags: Option<Token> = None;
    let mut arguments: Vec<Token> = Vec::new();

    if tokens.iter().any(|token| token.is_flag()) {

        let position = tokens.iter().position(|token| token.is_flag()).unwrap();

        if position != 1 {
            return Err(ParseError::InvalidFlagPosition)
        }

        flags = Some(tokens[position].clone());
    }

    while tokens.iter().any(|token| token.is_argument()) {

        let position = tokens.iter().position(|token| token.is_argument()).unwrap();

        if position == 1 && flags.is_some() {
            return Err(ParseError::InvalidArgumentPosition)
        }
    
        arguments.push(tokens[position].clone());
        tokens.remove(position);
    }

    Ok((command, flags, arguments))
}
