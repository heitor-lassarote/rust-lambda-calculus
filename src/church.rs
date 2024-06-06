use crate::cst::Cst;
use std::rc::Rc;

#[derive(Debug)]
pub enum Church {
    Var(String),
    Abs(String, Rc<Church>),
    App(Rc<Church>, Rc<Church>),
}

impl Church {
    pub fn var(name: &str) -> Rc<Church> {
        Rc::new(Church::Var(name.to_string()))
    }

    pub fn abs(name: &str, body: Rc<Church>) -> Rc<Church> {
        Rc::new(Church::Abs(name.to_string(), body))
    }

    pub fn abs_many(names: &Vec<String>, body: Rc<Church>) -> Rc<Church> {
        names.iter().rfold(body, |acc, name| Self::abs(name, acc))
    }

    pub fn app(l: Rc<Church>, r: Rc<Church>) -> Rc<Church> {
        Rc::new(Church::App(l, r))
    }

    pub fn desugar(cst: Rc<Cst>) -> Rc<Church> {
        match &*cst {
            Cst::Var(id) => Self::var(id),
            Cst::Abs(names, body) => {
                Self::abs_many(names, Self::desugar(body.clone()))
            }
            Cst::App(left, right) => Self::app(
                Self::desugar(left.clone()),
                Self::desugar(right.clone()),
            ),
            Cst::Par(inside) => Self::desugar(inside.clone()),
        }
    }
}
