pub mod lexer;
mod state_fns;

pub use lexer::Lexer;
use state_fns::*;

// TODO: Make this less fragile. It seems to get into infinite loops an awful
// lot!
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Plus,
    Minus,
    Number(i64),
}

struct StateFn(fn(&mut Lexer<Token>) -> Option<StateFn>);

pub fn run(mut input: impl std::io::Read) -> Vec<lexer::Item<Token>> {
    let mut input_text = String::new();
    input.read_to_string(&mut input_text).unwrap();
    let mut l = Lexer::new(input_text);

    let mut state = Some(StateFn(lex_general));
    while let Some(next_action) = state {
        state = next_action.0(&mut l);
    }
    l.to_vec()
}

#[cfg(test)]
mod test {
    use crate::lexing::{self, lexer::Item, Token};

    #[test]
    fn plus_minus() {
        assert_eq!(
            lexing::run(&b"plusminus"[..]),
            vec![
                Item {
                    token: Token::Plus,
                    pos: 0
                },
                Item {
                    token: Token::Minus,
                    pos: 4
                }
            ]
        );
    }

    #[test]
    fn basic_hex() {
        assert_eq!(
            lexing::run(&b"0x100"[..]),
            vec![Item {
                token: Token::Number(0x100),
                pos: 0,
            }]
        )
    }
}
