#[derive(Debug, Clone)]
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

    pub fn execute(&self, numbers: Vec<i64>) -> i64 {
        match self {
            Operation::Add => self.calculate(numbers, |a, b| a + b),
            Operation::Subtract => self.calculate(numbers, |a, b| a - b),
            Operation::Multiply => self.calculate(numbers, |a, b| a * b),
            Operation::Divide => self.calculate(numbers, |a, b| a / b),
            Operation::Modulo => self.calculate(numbers, |a, b| a % b)
        }
    }

    pub fn get_priority(&self) -> i8 {
        match self {
            Operation::Modulo => 2,
            Operation::Divide => 2,
            Operation::Multiply => 2,
            Operation::Add => 1,
            Operation::Subtract => 1
        }
    }

    fn calculate<F>(&self, numbers: Vec<i64>, f: F) -> i64 where F: Fn(i64, i64) -> i64 {
        let mut res: Option<i64> = None;
        for n in numbers {
            if res.is_none() {
                res = Some(n);
                continue;
            }
            res = Some(f(res.unwrap(), n));
        };
        res.unwrap()
    }
}