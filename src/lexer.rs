pub enum TokenType {
    If,
    Else,
    While,
    Var,
    Decimal(i32),
    VarName(String),
    // less than comparators not needed
    Less,
    LessEqual,
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
    True,
    False,
}

pub struct Token {
    index: usize,
    token: TokenType,
}

fn get_tokentype(s: String, ind: usize) -> TokenType {
    if !s.empty() {
        if s.chars().all(|c| c.is_numeric()) {
            return TokenType::Decimal(string.parse().unwrap());
        }
        match s => {
            "If" => TokenType::If,
            "Else" => TokenType::Else,
            "While" => TokenType::While
            "While" => TokenType::While,
            "<" => TokenType::Less,
            "<=" => TokenType::LessEqual,
            "+" => TokenType::Add,
            "-" => TokenType::Subtract,
            "*" => TokenType::Multiply,
            "/" => TokenType::Divide,
            "(" => TokenType::LParen,
            ")" => TokenType::RParen,
            "{" => TokenType::LBrace,
            "{" => TokenType::RBrace,
            "==" => TokenType::AssignEqual,
            "!" => TokenType::NotEqual,
            ";" => TokenType::Semicolon,
            "True" => TokenType::True,
            "False" => TokenType::False,
            _ => TokenType::VarName(s),
        }
    }
}

pub fn tokenize(code: &str) -> Vec<Token> {
    // code should contain all text in the source code file
    let mut tokens: Vec<Token> = Vec::new();
    let mut ind: usize = 0;
    let mut cur_token = String::new();
    let mut str_end: bool = false;
    // variables should only be made up of a-z and/or A-Z
    for c in code.chars() {
        match c {
            '(' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::LParen});
                ind += 1;
            },
            ')' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::RParen});
                ind += 1;
            },
            '{' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::LBrace});
                ind += 1;
            },
            '}' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::RBrace});
                ind += 1;
            },
            ';' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Semicolon});
                ind += 1;
            },
            '!' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::NotEqual});
                ind += 1;
            }
            '+' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Add});
                ind += 1;
            },
            '-' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Subtract});
                ind += 1;
            },
            '*' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Multiply});
                ind += 1;
            },
            '/' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Divide});
                ind += 1;
            },
            '=' => {
                match cur_token {
                    "=" => {
                        cur_token = String::new();
                        tokens.push(Token {index: ind, token: TokenType::CheckEqual});
                        ind += 1;
                    },
                    "<" => {
                        cur_token = String::new();
                        tokens.push(Token {index: ind, token: TokenType::LessEqual});
                        ind += 1;
                    },
                    _ => {
                        if !cur_token.is_empty() {
                            let t: TokenType = get_tokentype(cur_token, ind);
                            tokens.push(Token {index: ind, token: t});
                            cur_token = String::new();
                            ind += 1;
                        }
                    }
                }
            },
            '<' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                cur_token.push(c);
            },
            '0'..='9' => {
                if !cur_token.is_empty() && !v.last().is_numeric() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                cur_token.push(c);
            },
            'a'..='z' | 'A'..='Z' => {
                if !cur_token.is_empty() && !v.last().is_alphabetic() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                cur_token.push(c);
            },
            ' ' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token, ind);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
            },
            _ => {
                println!("syntax error");
            }
        }
    }
    return tokens;
}
