use crate::tokens::Token;

mod tokens;

struct Lexer {
    input: String,
    pos: usize,
}

pub fn convert_to_token(word: &str) -> Option<Token> {
    match word {
        "+" => Some(Token::Plus),
        "-" => Some(Token::Minus),
        "*" => Some(Token::Star),
        "/" => Some(Token::Div),
        "fn" => Some(Token::Function),
        "" => Some(Token::Eof),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "true" => Some(Token::True),
        "false" => Some(Token::False),
        "for" => Some(Token::For),
        "=" => Some(Token::Equal),
        _ => None,
    }
}

fn main() {
    // testing to see if the token is applied
    println!("this token is: {}", tokens::token_to_string(convert_to_token("+").unwrap()));

}

