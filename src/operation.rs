#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn to_string(&self) -> String {
        match self {
            Operation::Add => String::from("+"),
            Operation::Subtract => String::from("-"),
            Operation::Multiply => String::from("*"),
            Operation::Divide => String::from("/")
        }
    }

    pub fn from(s: &str) -> Option<Operation> {
        match s {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            _ => None
        }
    }

    pub fn get_operations() -> Vec<Operation> {
        vec![
            Operation::Add,
            Operation::Subtract,
            Operation::Multiply,
            Operation::Divide
        ]
    }

    pub fn execute(&self, a: i64, b: i64, ) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}