pub mod operation;
pub mod math;
pub mod token;

use std::io::{self, Write};

fn main() {
    // Interactive shell
    if std::env::args().len() <= 1 {
        print!("Query> ");
        io::stdout().flush().unwrap();
        let query = math::parse(&read_input());
        match math::calculate(query) {
            None => panic!(),
            Some(num) => println!("Answer: {}", num)
        }
        return;
    }

    // Calculate the args

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
    buffer.to_owned()
}
