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
    Multiply,
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

// todo! : implement debugger for Token struct

fn get_tokentype(s: String) -> TokenType {
    let mut ret: Option<TokenType> = None;
    if !s.is_empty() {
        if s.chars().all(|c| c.is_numeric()) {
            ret = Some(TokenType::Decimal(s.parse().unwrap()));
        }
        match s.as_str() {
            "If" => { ret = Some(TokenType::If); },
            "Else" => { ret = Some(TokenType::Else);} ,
            "While" => { ret = Some(TokenType::While); },
            "<" => { ret = Some(TokenType::Less); },
            "<=" => { ret = Some(TokenType::LessEqual); },
            "+" => { ret = Some(TokenType::Add); },
            "-" => { ret = Some(TokenType::Subtract); },
            "*" => { ret = Some(TokenType::Multiply); },
            "/" => { ret = Some(TokenType::Divide); },
            "(" => { ret = Some(TokenType::LParen); },
            ")" => { ret = Some(TokenType::RParen); },
            "{" => { ret = Some(TokenType::LBrace); },
            "{" => { ret = Some(TokenType::RBrace); },
            "==" => { ret = Some(TokenType::AssignEqual); },
            "!" => { ret = Some(TokenType::NotEqual); },
            ";" => { ret = Some(TokenType::Semicolon); },
            "True" => { ret = Some(TokenType::True); },
            "False" => { ret = Some(TokenType::False); },
            _ => { ret = Some(TokenType::VarName(s)); },
        }
    }
    return ret.unwrap();
}

pub fn tokenize(code: String) -> Vec<Token> {
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
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::LParen});
                ind += 1;
            },
            ')' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::RParen});
                ind += 1;
            },
            '{' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::LBrace});
                ind += 1;
            },
            '}' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::RBrace});
                ind += 1;
            },
            ';' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Semicolon});
                ind += 1;
            },
            '!' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::NotEqual});
                ind += 1;
            }
            '+' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Add});
                ind += 1;
            },
            '-' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Subtract});
                ind += 1;
            },
            '*' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Multiply});
                ind += 1;
            },
            '/' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                tokens.push(Token {index: ind, token: TokenType::Divide});
                ind += 1;
            },
            '=' => {
                match cur_token.as_str() {
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
                            let t: TokenType = get_tokentype(cur_token);
                            tokens.push(Token {index: ind, token: t});
                            cur_token = String::new();
                            ind += 1;
                        }
                    }
                }
            },
            '<' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                cur_token.push(c);
            },
            '0'..='9' => {
                if !cur_token.is_empty() && !cur_token.chars().last().unwrap().is_numeric() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                cur_token.push(c);
            },
            'a'..='z' | 'A'..='Z' => {
                if !cur_token.is_empty() && !cur_token.chars().last().unwrap().is_alphabetic() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
                cur_token.push(c);
            },
            ' ' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
            },
            '\n' => {
                if !cur_token.is_empty() {
                    let t: TokenType = get_tokentype(cur_token);
                    tokens.push(Token {index: ind, token: t});
                    cur_token = String::new();
                    ind += 1;
                }
            }
            _ => {
                println!("syntax error");
            }
        }
    }
    return tokens;
}
