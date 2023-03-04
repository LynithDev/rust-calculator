use crate::{operation::Operation, token::Token};

pub fn parse(expression: &str) -> Result<Vec<Token>, String> {
    let mut query = Vec::<Token>::new();
    let mut builder = String::new();
    let split: Vec<String> = expression.split("").map(|s| s.to_string()).collect();

    for (i, s) in split.iter().enumerate() {
        // Skip useless characters
        if s.eq(" ") || s.is_empty() || s.eq("\n") {
            continue
        };

        if s.eq("(") || s.eq(")") {
            continue
        }
        
        match Operation::from(s) {
            // Append to the string builder if the current character isn't an operation
            None => builder += s,
            Some(operator) => {
                match builder.as_str() {
                    // Negative number check
                    // If the operator is Subtract, or if the current builder string begins with the minus symbol, and if the next character in the list exists, then add on to the builder string and continue
                    "" if (operator.to_string().eq("-") || builder.starts_with("-")) && split.len() > i && !split[i + 1].is_empty() => {
                        builder += "-";
                        continue;
                    },

                    // Otherwise return an error as anything else can cause issues (e.g. "* 5" would be equal to "0 * 5" without the code below)
                    "" => return Err(format!("Invalid token at '{i}' char '{s}'")),

                    // if all is well then append the builder to the query vector
                    _ => query.push(match Token::from(builder.as_str()) {
                        Ok(e) => e,
                        Err(err) => return Err(err)
                    })
                };

                // Append the operation character
                query.push(match Token::from(&operator.to_string()) {
                    Ok(d) => d,
                    Err(err) => return Err(format!("{err} at '{i}' char '{s}'"))
                });

                // Clear the string builder
                builder = String::new();
            }
        };
    }

    // Adds remaining tokens
    if !builder.is_empty() {
        builder = builder.trim().to_owned();
        query.push(match Token::from(builder.as_str()) {
            Ok(e) => e,
            Err(err) => return Err(err)
        });
    }

    return Ok(query);
}

fn convert_to_postfix(query: Vec<Token>) -> Vec<Token> {
    let mut numbers = Vec::<Token>::new();
    let mut stack = Vec::<Token>::new();
    for (iter, token) in query.iter().enumerate() {
        if token.is_number {
            numbers.push(token.to_owned());
        } else if token.is_special {
            match token.special.as_ref().unwrap().as_str() {
                "(" => stack.push(token.to_owned()),
                ")" => {
                    while !stack.is_empty() && !stack[stack.len() - 1].to_string().eq("(") {
                        print!("{}", stack[stack.len() - 1].to_owned().to_string());
                        numbers.push(stack[stack.len() - 1].to_owned());
                        stack.pop();
                    }
                    stack.pop();
                }
                _ => stack.push(token.to_owned()),
            }
        } else if token.is_operation {
            while stack.len() != 0 && stack[stack.len() - 1].operation.as_ref().is_some() && token.operation.as_ref().unwrap().get_priority() <= stack[stack.len() - 1].operation.as_ref().unwrap().get_priority() {
                numbers.push(stack[stack.len() - 1].to_owned());
                stack.pop();
            }
            stack.push(token.to_owned());
        }
    };

    while !stack.is_empty() {
        numbers.push(stack[stack.len() - 1].to_owned());
        stack.pop();
    }

    numbers
}

fn calculate_postfix(query: Vec<Token>) -> i64 {
    let mut stack = Vec::<i64>::new();
    for (iter, token) in query.iter().enumerate() {
        if token.is_number {
            stack.push(token.number.unwrap())
        } else if token.is_operation {
            let val1 = stack.pop().unwrap();
            let val2 = stack.pop().unwrap();
            let op = token.operation.as_ref().unwrap();
            
            stack.push(op.execute(vec![val1, val2]));
        }
    };
    stack.pop().unwrap()
}

pub fn calculate(query: Vec<Token>) -> Option<i64> {
    let mut answer: Option<i64> = None;
    answer = Some(calculate_postfix(convert_to_postfix(query)));
    // for (iter, token) in query.iter().enumerate() {
    //     if token.is_number {
    //         // If the answer varialbe is null then set it to the current iterations number
    //         if answer.is_none() {
    //             answer = token.number;
    //             continue;
    //         }

    //         // If the iteration isn't 0, then check the previous iteration token to check whether its an operation
    //         if iter != 0 && query.len() > 0 && query[iter - 1].is_operation {
    //             let op = &query[iter - 1].operation;

    //             // If it is an operation, do the operation against the answer variable and the current iteration token value
    //             if op.is_some() {
    //                 println!("Evaluating: {} {} {}", answer.unwrap(), op.as_ref().unwrap().to_string(), token.number.unwrap());
    //                 answer = Some(op.as_ref().unwrap().execute(vec![answer.unwrap(), token.number.unwrap()]));
    //             }
    //         }
    //     }
    // };

    return answer;
}