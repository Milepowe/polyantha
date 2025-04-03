use crate::tokens::Token;

mod tokens;

pub fn convert_to_token(word: &str) -> Option<Token> {
    match word {
        "+" => Some(Token::PLUS),
        "-" => Some(Token::MINUS),
        "*" => Some(Token::MULT),
        "/" => Some(Token::DIV),
        "fn" => Some(Token::FUNCTION),
        " " => Some(Token::EOF),
        "if" => Some(Token::IF),
        "else" => Some(Token::ELSE),
        "true" => Some(Token::TRUE),
        "false" => Some(Token::FALSE),
        "for" => Some(Token::FOR),
        _ => None
    }
}
fn main() {
}