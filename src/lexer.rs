use std::{iter::Peekable, str::Chars};

use crate::tok::Tok;

pub struct Lexer<'a> {
    peekable: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            peekable: input.chars().peekable(),
        };
        lexer.skip_whitespace();
        lexer
    }

    fn peek_char(&mut self) -> Option<char> {
        self.peekable.peek().copied()
    }

    fn skip_whitespace(&mut self) -> () {
        while self.peek_char().map_or(false, char::is_whitespace) {
            self.peekable.next();
        }
    }

    fn is_id_char(c: char, idx: usize) -> bool {
        if idx == 0 {
            c.is_alphabetic() && c != 'λ'
        } else {
            c.is_alphanumeric() && c != 'λ'
        }
    }

    fn id(&mut self) -> String {
        let mut id = String::new();
        while let Some(c) = self.peek_char() {
            if Self::is_id_char(c, id.len()) {
                id.push(c);
                self.peekable.next();
            } else {
                break;
            }
        }
        id
    }

    fn eat(&mut self, tok: Tok<String>) -> Option<Tok<String>> {
        self.peekable.next()?;
        Some(tok)
    }

    pub fn peek(&mut self) -> Option<Tok<()>> {
        match self.peek_char()? {
            '\\' | 'λ' => Some(Tok::Lam),
            '.' => Some(Tok::Dot),
            '(' => Some(Tok::LPar),
            ')' => Some(Tok::RPar),
            _ => Some(Tok::Id(())),
        }
    }

    fn advance(&mut self) -> Option<Tok<String>> {
        let c = self.peek_char()?;
        let result = match c {
            '\\' | 'λ' => self.eat(Tok::Lam),
            '.' => self.eat(Tok::Dot),
            '(' => self.eat(Tok::LPar),
            ')' => self.eat(Tok::RPar),
            _ => Some(Tok::Id(self.id())),
        };
        self.skip_whitespace();
        result
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Tok<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()
    }
}
