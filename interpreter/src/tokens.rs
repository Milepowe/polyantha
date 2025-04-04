pub enum Token {
    Plus,
    Minus,
    Star,
    Div,
    Function,
    If,
    Else,
    True,
    False,
    For,
    Equal,
    Eof,
}

pub fn token_to_string(token: Token) -> String {
    match token {
        Token::Plus => String::from("+"),
        Token::Minus => String::from("-"),
        Token::Star => String::from("*"),
        Token::Div => String::from("/"),
        Token::Function => String::from("fn"),
        Token::Eof => String::from("EOF"),
        Token::If => String::from("if") ,
        Token::Else => String::from("else"),
        Token::True => String::from("true"),
        Token::False => String::from("false"),
        Token::For => String::from("for"),
        Token::Equal => String::from("Equal"),
    }
}

