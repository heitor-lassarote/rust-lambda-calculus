use crate::cst::Cst;

#[derive(Debug)]
pub enum Church {
    Var(String),
    Abs(String, Box<Church>),
    App(Box<Church>, Box<Church>),
}

impl Church {
    pub fn var(name: String) -> Box<Church> {
        Box::new(Church::Var(name))
    }

    pub fn abs(name: String, body: Box<Church>) -> Box<Church> {
        Box::new(Church::Abs(name, body))
    }

    pub fn abs_many(names: &[String], body: Box<Church>) -> Box<Church> {
        names
            .iter()
            .rfold(body, |acc, name| Self::abs(name.clone(), acc))
    }

    pub fn app(l: Box<Church>, r: Box<Church>) -> Box<Church> {
        Box::new(Church::App(l, r))
    }

    pub fn desugar(cst: Box<Cst>) -> Box<Church> {
        match *cst {
            Cst::Var(id) => Self::var(id),
            Cst::Abs(names, body) => {
                Self::abs_many(&names, Self::desugar(body))
            }
            Cst::App(left, right) => {
                Self::app(Self::desugar(left), Self::desugar(right))
            }
            Cst::Par(inside) => Self::desugar(inside),
        }
    }
}
