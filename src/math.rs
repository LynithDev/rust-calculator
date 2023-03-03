use crate::{operation::Operation, token::Token};

pub fn parse(expression: &str) -> Result<Vec<Token>, String> {
    let mut query = Vec::<Token>::new();
    let mut builder = String::new();

    for (i, s) in expression.split("").enumerate() {
        if s.eq(" ") || s.is_empty() || s.eq("\n") {
            continue
        };
        
        match Operation::from(s) {
            None => builder += s,
            Some(operator) => {
                match builder.as_str() {
                    "" if operator.to_string().eq("-") || operator.to_string().eq("+") => query.push(match Token::from("0") {
                        Ok(r) => r,
                        Err(err) => return Err(format!("{err} at '{i}' char '{s}'")) 
                    }),

                    "" => return Err(format!("Invalid token at '{i}' char '{s}'")),

                    _ => query.push(match Token::from(builder.as_str()) {
                        Ok(e) => e,
                        Err(err) => return Err(err)
                    }),
                }

                query.push(match Token::from(&operator.to_string()) {
                    Ok(d) => d,
                    Err(err) => return Err(format!("{err} at '{i}' char '{s}'"))
                });

                builder = String::new();
            }
        };
    }

    if !builder.is_empty() {
        builder = builder.trim().to_owned();
        query.push(match Token::from(builder.as_str()) {
            Ok(e) => e,
            Err(err) => return Err(err)
        });
    }

    return Ok(query);
}

pub fn calculate(query: Vec<Token>) -> Option<i64> {
    let mut answer: Option<i64> = None;
    for (iter, token) in query.iter().enumerate() {
        if token.is_number {
            if answer.is_none() {
                answer = token.number;
                continue;
            }

            if iter != 0 && query.len() > 0 && query[iter - 1].is_operation {
                let op = &query[iter - 1].operation;
                if op.is_some() {
                    answer = Some(op.as_ref().unwrap().execute(answer.unwrap(), token.number.unwrap()));
                }
            }
        }
    };

    return answer;
}