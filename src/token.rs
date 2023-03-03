use crate::operation::{Operation};

#[derive(Debug)]
pub struct Token {
    pub is_operation: bool,
    pub operation: Option<Operation>,
    pub is_number: bool,
    pub number: Option<i64>
}

impl Token {
    pub fn from(s: &str) -> Token {
        if let Some(op) = Operation::from(s) {
            return Token {
                is_operation: true,
                is_number: false,
                number: None,
                operation: Some(op)
            };
        }

        let parsed = match str::parse::<i64>(s) {
            Ok(num) => num,
            Err(err) => panic!("{err}")
        };

        Token {
            is_operation: false,
            operation: None,
            is_number: true,
            number: Some(parsed)
        }
    }
}