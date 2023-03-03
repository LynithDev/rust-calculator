#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl Operation {
    pub fn to_string(&self) -> String {
        match self {
            Operation::Add => String::from("+"),
            Operation::Subtract => String::from("-"),
            Operation::Multiply => String::from("*"),
            Operation::Divide => String::from("/"),
            Operation::Modulo => String::from("%")
        }
    }

    pub fn from(s: &str) -> Option<Operation> {
        match s {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            "%" => Some(Operation::Modulo),
            _ => None
        }
    }

    pub fn execute(&self, a: i64, b: i64, ) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
            Operation::Modulo => a % b
        }
    }
}