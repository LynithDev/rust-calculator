use crate::operation::{Operation};

#[derive(Debug)]
pub struct Token {
    pub is_operation: bool,
    pub operation: Option<Operation>,
    pub is_number: bool,
    pub number: Option<i64>,
    pub is_special: bool,
    pub special: Option<String>
}

impl Token {
    pub fn from_number(n: i64) -> Token {
        return Token {
            is_operation: true,
            operation: None,
            is_number: true,
            number: Some(n),
            is_special: false,
            special: None
        };
    }

    pub fn from(s: &str) -> Result<Token, String> {
        if let Some(op) = Operation::from(s) {
            return Ok(Token {
                is_operation: true,
                is_number: false,
                number: None,
                operation: Some(op),
                is_special: false,
                special: None
            });
        }

        if s.eq("(") || s.eq(")") {
            return Ok(Token {
                is_number: false,
                is_operation: false,
                is_special: true,
                number: None,
                operation: None,
                special: Some(s.to_owned())
            });
        }

        let parsed = match str::parse::<i64>(s) {
            Ok(num) => num,
            Err(_) => {
                return Err(String::from("Invalid character"))
            }
        };

        Ok(Token {
            is_operation: false,
            operation: None,
            is_number: true,
            number: Some(parsed),
            is_special: false,
            special: None
        })
    }
}