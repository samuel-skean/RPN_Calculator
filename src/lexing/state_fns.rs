use super::{lexer::Lexer, StateFn, Token};

const DIGITS: &str = "0123456789";
const HEX_PREFIX: &str = "0x";
const OCT_PREFIX: &str = "0";
const BIN_PREFIX: &str = "0b";

pub fn lex_general(l: &mut Lexer<Token>) -> Option<StateFn> {
    while !l.end() {
        if l.accept_first_prefix(&["minus", "-"]) {
            l.emit(Token::Minus);
        } else if l.accept_first_prefix(&["plus", "+"]) {
            l.emit(Token::Plus);
        } else if DIGITS.contains(l.peek()) {
            return Some(StateFn(lex_number));
        }
        l.accept_run(" \t\n");
        l.ignore();
    }
    None
}

pub fn lex_number(l: &mut Lexer<Token>) -> Option<StateFn> {
    if l.accept_prefix(HEX_PREFIX) {
        return Some(LEX_HEX);
    } else if l.accept_prefix(BIN_PREFIX) {
        return Some(LEX_BIN);
    } else if l.accept_prefix(OCT_PREFIX) {
        return Some(LEX_OCT);
    }
    Some(LEX_DEC)
}

fn lex_digits<const PREFIX_LEN: usize, const RADIX: u32>(l: &mut Lexer<Token>) -> Option<StateFn> {
    l.accept_run(DIGITS);
    l.emit(Token::Number(
        i64::from_str_radix(&l.current_match()[PREFIX_LEN..], RADIX).unwrap(),
    ));
    Some(StateFn(lex_general))
}

// Why braces?
const LEX_DEC: StateFn = StateFn(lex_digits::<0, 10>);
const LEX_HEX: StateFn = StateFn(lex_digits::<{ HEX_PREFIX.len() }, 16>);
const LEX_OCT: StateFn = StateFn(lex_digits::<{ OCT_PREFIX.len() }, 8>);
const LEX_BIN: StateFn = StateFn(lex_digits::<{ BIN_PREFIX.len() }, 2>);
