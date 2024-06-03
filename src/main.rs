use std::{collections::HashMap, iter::Peekable, rc::Rc, str::Chars};

#[derive(Debug)]
enum Tok {
    Lam,
    Dot,
    LPar,
    RPar,
    Id(String),
}

struct Lexer<'a> {
    peekable: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            peekable: input.chars().peekable(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.peekable.peek().copied()
    }

    fn skip_whitespace(&mut self) -> () {
        while self.peek().map_or(false, char::is_whitespace) {
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
        while let Some(c) = self.peek() {
            if Self::is_id_char(c, id.len()) {
                id.push(c);
                self.peekable.next();
            } else {
                break;
            }
        }
        id
    }

    fn eat(&mut self, tok: Tok) -> Option<Tok> {
        self.peekable.next()?;
        Some(tok)
    }

    fn advance(&mut self) -> Option<Tok> {
        let c = self.peek()?;
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
    type Item = Tok;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()
    }
}

#[derive(Debug)]
enum Church {
    Var(String),
    Abs(String, Rc<Church>),
    App(Rc<Church>, Rc<Church>),
}

impl Church {
    fn var(name: &str) -> Rc<Church> {
        Rc::new(Church::Var(name.to_string()))
    }

    fn abs(name: &str, body: Rc<Church>) -> Rc<Church> {
        Rc::new(Church::Abs(name.to_string(), body))
    }

    fn app(l: Rc<Church>, r: Rc<Church>) -> Rc<Church> {
        Rc::new(Church::App(l, r))
    }
}

type Idx = i32;
type Lvl = i32;

#[derive(Debug)]
enum DeBruijn {
    Bound(String, Idx),
    Free(String),
    Abs(String, Rc<DeBruijn>),
    App(Rc<DeBruijn>, Rc<DeBruijn>),
}

impl DeBruijn {
    fn bound(name: String, idx: Idx) -> Rc<DeBruijn> {
        Rc::new(DeBruijn::Bound(name, idx))
    }

    fn free(name: String) -> Rc<DeBruijn> {
        Rc::new(DeBruijn::Free(name))
    }

    fn abs(name: String, body: Rc<DeBruijn>) -> Rc<DeBruijn> {
        Rc::new(DeBruijn::Abs(name, body))
    }

    fn app(l: Rc<DeBruijn>, r: Rc<DeBruijn>) -> Rc<DeBruijn> {
        Rc::new(DeBruijn::App(l, r))
    }
}

fn rename_impl(
    exp: Rc<Church>,
    env: &mut HashMap<String, Idx>,
    lvl: Lvl,
) -> Rc<DeBruijn> {
    match &*exp {
        Church::Var(name) => match env.get(name) {
            None => DeBruijn::free(name.to_string()),
            Some(idx) => DeBruijn::bound(name.to_string(), lvl - 1 - *idx),
        },
        Church::Abs(name, body) => {
            let old_lvl = env.insert(name.to_string(), lvl);
            let body = rename_impl(body.clone(), env, lvl + 1);
            if let Some(old_lvl) = old_lvl {
                env.insert(name.to_string(), old_lvl);
            } else {
                env.remove(name);
            }
            DeBruijn::abs(name.to_string(), body)
        }
        Church::App(l, r) => DeBruijn::app(
            rename_impl(l.clone(), env, lvl),
            rename_impl(r.clone(), env, lvl),
        ),
    }
}

fn rename(exp: Rc<Church>) -> Rc<DeBruijn> {
    rename_impl(exp, &mut HashMap::new(), 0)
}

fn subst(
    var @ (name, idx): &(String, Idx),
    body: Rc<DeBruijn>,
    with: Rc<DeBruijn>,
) -> Rc<DeBruijn> {
    match &*body {
        DeBruijn::Bound(_var2, idx2) => {
            if idx == idx2 {
                with
            } else {
                body
            }
        }
        DeBruijn::Free(_var2) => body,
        DeBruijn::Abs(var2, body) => DeBruijn::abs(
            var2.clone(),
            subst(&(name.to_string(), *idx + 1), body.clone(), with),
        ),
        DeBruijn::App(l, r) => DeBruijn::app(
            subst(&var, l.clone(), with.clone()),
            subst(&var, r.clone(), with),
        ),
    }
}

fn eval(exp: Rc<DeBruijn>) -> Rc<DeBruijn> {
    match &*exp {
        DeBruijn::Bound(_, _) | DeBruijn::Free(_) | DeBruijn::Abs(_, _) => exp,
        DeBruijn::App(l, r) => {
            let l = eval(l.clone());
            match &*l {
                DeBruijn::Abs(x, body) => {
                    eval(subst(&(x.to_string(), 0), body.clone(), r.clone()))
                }
                _ => DeBruijn::app(l, r.clone()),
            }
        }
    }
}

fn main() {
    let exp = Church::app(
        Church::app(
            Church::abs("x", Church::abs("y", Church::var("x"))),
            Church::var("a"),
        ),
        Church::var("b"),
    );

    println!("Input:  {:?}", exp);
    let input = r"(λx. λy. x) a b";
    println!("String: {:?}", input);
    let tokens: Vec<Tok> = Lexer::new(input).collect();
    println!("Tokens: {:?}", tokens);
    let result = eval(rename(exp));
    println!("Result: {:?}", result);
}
