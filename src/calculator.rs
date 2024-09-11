use crate::lexing::Token;

pub fn calculate(tokens: impl Iterator<Item = Token>) -> Result<i64, &'static str> {
    let mut operands_stack = Vec::new();

    for token in tokens {
        match token {
            Token::Plus => {
                let sum = operands_stack.pop().unwrap() + operands_stack.pop().unwrap();
                operands_stack.push(sum);
            }
            Token::Minus => {
                let subtrahend = operands_stack.pop().unwrap();
                let minuend = operands_stack.pop().unwrap();
                let difference = minuend - subtrahend;
                operands_stack.push(difference);
            }
            Token::Number(n) => operands_stack.push(n),
        }
    }
    let result = operands_stack.pop().unwrap();
    if operands_stack.is_empty() {
        Ok(result)
    } else {
        Err("Too many operands on the stack.")
    } 
}
