pub mod operation;
pub mod math;
pub mod token;

use std::{io::{self, Write}, env};

fn main() {
    // Interactive shell
    if env::args().len() <= 1 {
        shell()
    }

    // Calculate the args
    let mut query = Vec::<String>::new();
    for arg in env::args().skip(1) {
        query.push(arg);
    }
    
    let parse = match math::parse(query.join("").as_str()) {
        Ok(q) => q,
        Err(err) => {
            println!("{err}");
            std::process::exit(1);
        }
    };

    let result = match math::calculate(parse) {
        None => String::from("Invalid input. Please try again."),
        Some(num) => num.to_string()
    };
    println!("{result}");
}

fn shell() {
    print!("Query> ");
    io::stdout().flush().unwrap();
    let query = match math::parse(&read_input()) {
        Ok(q) => q,
        Err(err) => {
            println!("{err}");
            return shell();
        }
    };
    let result = match math::calculate(query) {
        None => String::from("Invalid input. Please try again."),
        Some(num) => num.to_string()
    };
    println!("{result}");
    shell()
}

fn read_input() -> String {
    let buffer = &mut String::new();
    loop {
        match io::stdin().read_line(buffer) {
            Ok(_) => break,
            Err(_) => {
                print!("\nCould not get input. Please try again")
            },
        };
    };
    buffer.to_owned().trim().replace("\\n", "")
}
