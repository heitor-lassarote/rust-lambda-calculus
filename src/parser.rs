use crate::{cst::Cst, lexer::Lexer, tok::Tok};
use std::result;

#[derive(Debug)]
pub enum Err {
    UnexpectedEof,
    UnexpectedTok(Tok<String>),
}

pub type Result = result::Result<Box<Cst>, Err>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser { lexer }
    }

    fn peek(&mut self) -> Option<Tok<()>> {
        self.lexer.peek()
    }

    fn peek_eof(&mut self) -> result::Result<Tok<()>, Err> {
        self.peek().ok_or(Err::UnexpectedEof)
    }

    fn next(&mut self) -> Option<Tok<String>> {
        self.lexer.next()
    }

    fn next_eof(&mut self) -> result::Result<Tok<String>, Err> {
        self.next().ok_or(Err::UnexpectedEof)
    }

    fn balance_app(left: Box<Cst>, right: Box<Cst>) -> Box<Cst> {
        match *right {
            Cst::App(middle, right) => {
                Cst::app(Self::balance_app(left, middle), right)
            }
            _ => Cst::app(left, right),
        }
    }

    fn eat(&mut self, expected_tok: Tok<String>) -> result::Result<(), Err> {
        let actual_tok = self.next_eof()?;
        if expected_tok == actual_tok {
            Ok(())
        } else {
            Err(Err::UnexpectedTok(actual_tok))
        }
    }

    fn eat_id(&mut self) -> result::Result<String, Err> {
        match self.next_eof()? {
            Tok::Id(id) => Ok(id),
            other => Err(Err::UnexpectedTok(other)),
        }
    }

    fn par(&mut self) -> Result {
        let () = self.eat(Tok::LPar)?;
        let inside = self.app()?;
        let () = self.eat(Tok::RPar)?;
        Ok(Cst::par(inside))
    }

    fn var(&mut self) -> Result {
        let id = self.eat_id()?;
        Ok(Cst::var(id))
    }

    fn eat_ids(&mut self) -> result::Result<Vec<String>, Err> {
        let mut ids = vec![self.eat_id()?];
        while let Some(Tok::Id(())) = self.peek() {
            ids.push(self.eat_id()?);
        }
        Ok(ids)
    }

    fn abs(&mut self) -> Result {
        let () = self.eat(Tok::Lam)?;
        let ids = self.eat_ids()?;
        let () = self.eat(Tok::Dot)?;
        let body = self.app()?;
        Ok(Cst::abs(ids, body))
    }

    fn app(&mut self) -> Result {
        let left = self.left()?;
        if let Some(Tok::RPar) = self.peek() {
            return Ok(left);
        } else if let Some(_) = self.peek() {
            ()
        } else {
            return Ok(left);
        };

        let right = self.app()?;

        Ok(Self::balance_app(left, right))
    }

    fn left(&mut self) -> Result {
        match self.peek_eof()? {
            Tok::Lam => self.abs(),
            Tok::Dot => Err(Err::UnexpectedTok(Tok::Dot)),
            Tok::LPar => self.par(),
            Tok::RPar => Err(Err::UnexpectedTok(Tok::RPar)),
            Tok::Id(_) => self.var(),
        }
    }

    fn eof(&mut self) -> result::Result<(), Err> {
        if let Some(tok) = self.next() {
            Err(Err::UnexpectedTok(tok))
        } else {
            Ok(())
        }
    }

    pub fn parse(&mut self) -> Result {
        let expr = self.app()?;
        let () = self.eof()?;
        Ok(expr)
    }
}
