use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Ident(&'static str),
    Keyword(&'static str),

    // symbols
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Comma,

    Pipe,
    FatArrow,

    // literals
    IntLit(i64),
    BoolLit(bool),

    // whitespace, pruned
    None,
}

impl TokenKind {
    pub fn length(&self) -> usize {
        use TokenKind::*;
        match self {
            Ident(s) => s.len(),
            Keyword(s) => s.len(),
            FatArrow => 2,
            BoolLit(b) => b.to_string().len(),
            IntLit(num) => num.to_string().len(),
            _ => 1,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token: TokenKind,
    start: usize,
    len: usize,

    path: &'static str,
    src: &'static str,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

fn get_line_info(s: &str, index: usize) -> (usize, usize) {
    let mut line_number = 1;
    let mut line_start = 0;

    for (i, c) in s.char_indices() {
        if i >= index {
            break;
        }
        if c == '\n' {
            line_number += 1;
            line_start = i + 1;
        }
    }

    let position_in_line = index - line_start + 1;
    (line_number, position_in_line)
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (line, start) = get_line_info(self.src, self.start);
        write!(f, "{}:{}:{} {:?}", self.path, line, start, self.token)
    }
}

impl Token {
    pub fn in_context(&self) -> String {
        let (line, col) = get_line_info(self.src, self.start);
        let line_src = self.src.lines().nth(line - 1).unwrap();

        format!(
            "{}:{}:{}\n{}\n{}{}",
            self.path,
            line,
            col,
            line_src,
            " ".repeat(col - 1),
            "^".repeat(self.len)
        )
    }
}

fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

fn is_keyword(s: &str) -> bool {
    match s {
        "fn" | "let" | "match" | "cond" | "itself" => true,
        _ => false,
    }
}

fn is_int(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

fn is_bool(s: &str) -> bool {
    s == "true" || s == "false"
}

pub struct Lexer {
    path: &'static str,
    src: &'static str,

    pos: usize,
    accum: String,

    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(path: &str) -> Self {
        let src = std::fs::read_to_string(path).expect("could not read file");

        Self {
            path: leak(path),
            src: leak(&src),
            pos: 0,
            accum: String::new(),
            tokens: Vec::new(),
        }
    }

    fn push_accum(&mut self) {
        if !self.accum.is_empty() {
            let text = leak(&self.accum);

            let token = match text {
                _ if is_keyword(text) => TokenKind::Keyword(text),
                _ if is_int(text) => TokenKind::IntLit(text.parse().unwrap()),
                _ if is_bool(text) => TokenKind::BoolLit(text == "true"),
                _ => TokenKind::Ident(text),
            };

            self.accum.clear();
            self.push(token);
        }
    }

    fn push(&mut self, token: TokenKind) {
        self.push_accum();
        let len = token.length();

        self.tokens.push(Token {
            token,
            start: self.pos,
            len: len,

            path: self.path,
            src: self.src,
        });

        self.pos += len;
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut chars = self.src.chars();

        while let Some(c) = chars.next() {
            match c {
                c if c.is_whitespace() => self.push(TokenKind::None),
                '(' => self.push(TokenKind::LParen),
                ')' => self.push(TokenKind::RParen),
                '{' => self.push(TokenKind::LBrace),
                '}' => self.push(TokenKind::RBrace),
                ':' => self.push(TokenKind::Colon),
                ',' => self.push(TokenKind::Comma),
                '|' => self.push(TokenKind::Pipe),
                '=' => match chars.next() {
                    Some('>') => self.push(TokenKind::FatArrow),
                    Some(' ') => {
                        self.accum.push('=');
                        self.push(TokenKind::None);
                    }
                    Some(o) => {
                        self.accum.push('=');
                        self.accum.push(o);
                    }
                    None => self.push(TokenKind::None),
                },
                c => {
                    self.accum.push(c);
                }
            }
        }

        self.tokens
            .into_iter()
            .filter(|t| t.token != TokenKind::None)
            .collect()
    }
}
