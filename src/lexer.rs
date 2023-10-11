pub enum TokenType {
    If,
    Else,
    While,
    Var,
    Decimal(i32),
    VarName(String),
    // less than comparators not needed
    Greater,
    GreaterEqual,
    Add, 
    Subtract,
    Multiply
    Divide,
    AssignEqual,
    CheckEqual,
    Print,
    NotEqual,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
}

pub struct Token {
    index: usize,
    token: TokenType,
}

pub fn tokenize(code: &str) -> Vec<Token> {
    // code should contain all text in the source code file
    let mut tokens: Vec<Token> = Vec::new();
    let mut curToken = String::new();
    // variables should only be made up of a-z and/or A-Z
    for c in code.chars() {
        match c {
            ' ' => {
                // end of current token, so parse it. 
                
                // need the if check in case of variable amount of whitespace
                if !curToken.empty() {
                    let t: Option<Token> = None;
                    let t = match curToken {
                        "if" => Some(Token::If),
                        "else" => Some(Token::Else),
                        "while" => Some(Token::While),
                        "var" => Some(Token::Var),
                        ">" => Some(Token::Greater),
                        ">=" => Some(Token::GreaterEqual),
                        "+" => Some(Token::Add),
                        "-" => Some(Token::Subtract),
                        "*" => Some(Token::Multiply),
                        "/" => Some(Token::Divide),
                        "=" => Some(Token::Else),
                        "else" => Some(Token::Else),
                        "else" => Some(Token::Else),
                        "else" => Some(Token::Else),
                        "else" => Some(Token::Else),
                        _ => Some(Token::VarName(curToken));
                        // only numerical => an int
                    };
                }
            }
            _ => curToken.push(c);
        }
    }
}
