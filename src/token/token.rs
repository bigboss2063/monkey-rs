#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + Literals
    IDENT(String),
    INT(String),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NQ,

    // Delimiters
    SEMICOLON,
    LPAREN,
    RPAREN,
    COMMA,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Token {
    pub fn token_literal(&self) -> String {
        match self {
            Token::ILLEGAL => String::from("illegal"),
            Token::EOF => String::from("eof"),
            Token::IDENT(literal) => literal.clone(),
            Token::INT(literal) => literal.clone(),
            Token::ASSIGN => String::from("="),
            Token::PLUS => String::from("+"),
            Token::MINUS => String::from("-"),
            Token::BANG => String::from("!"),
            Token::ASTERISK => String::from("*"),
            Token::SLASH => String::from("/"),
            Token::LT => String::from("<"),
            Token::GT => String::from(">"),
            Token::EQ => String::from("=="),
            Token::NQ => String::from("!="),
            Token::SEMICOLON => String::from(";"),
            Token::LPAREN => String::from("("),
            Token::RPAREN => String::from(")"),
            Token::COMMA => String::from(","),
            Token::LBRACE => String::from("{"),
            Token::RBRACE => String::from("}"),
            Token::FUNCTION => String::from("function"),
            Token::LET => String::from("let"),
            Token::TRUE => String::from("true"),
            Token::FALSE => String::from("false"),
            Token::IF => String::from("if"),
            Token::ELSE => String::from("else"),
            Token::RETURN => String::from("return"),
        }
    }

    pub fn token_name(&self) -> String {
        match self {
            Token::ILLEGAL => String::from("ILLEGAL"),
            Token::EOF => String::from("EOF"),
            Token::IDENT(_) => String::from("IDENT"),
            Token::INT(_) => String::from("INT"),
            Token::ASSIGN => String::from("ASSIGN"),
            Token::PLUS => String::from("PLUS"),
            Token::MINUS => String::from("MINUS"),
            Token::BANG => String::from("BANG"),
            Token::ASTERISK => String::from("ASTERISK"),
            Token::SLASH => String::from("SLASH"),
            Token::LT => String::from("LT"),
            Token::GT => String::from("GT"),
            Token::EQ => String::from("EQ"),
            Token::NQ => String::from("NQ"),
            Token::SEMICOLON => String::from("SEMICOLON"),
            Token::LPAREN => String::from("LPAREN"),
            Token::RPAREN => String::from("RPAREN"),
            Token::COMMA => String::from("COMMA"),
            Token::LBRACE => String::from("LBRACE"),
            Token::RBRACE => String::from("RBRACE"),
            Token::FUNCTION => String::from("FUNCTION"),
            Token::LET => String::from("LET"),
            Token::TRUE => String::from("TRUE"),
            Token::FALSE => String::from("FALSE"),
            Token::IF => String::from("IF"),
            Token::ELSE => String::from("ELSE"),
            Token::RETURN => String::from("RETURN"),
        }
    }
}
