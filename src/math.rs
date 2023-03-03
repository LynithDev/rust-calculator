use crate::{operation::Operation, token::Token};

pub fn parse(s: &str) -> Vec<Token> {
    let mut query = Vec::<Token>::new();
    let mut builder = String::new();

    for i in s.split("") {
        if i.eq(" ") {
            continue
        };
        match Operation::from(i) {
            None => builder += i,
            Some(operator) => {
                query.push(Token::from(builder.as_str()));
                query.push(Token::from(&operator.to_string()));
                builder = String::new();
            }
        };
    }

    if !builder.is_empty() {
        builder = builder.trim().to_owned();
        query.push(Token::from(builder.as_str()));
    }

    return query;
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