#[derive(Debug)]
pub enum Cst {
    Var(String),
    Abs(Vec<String>, Box<Cst>),
    App(Box<Cst>, Box<Cst>),
    Par(Box<Cst>),
}

impl Cst {
    pub fn var(name: String) -> Box<Cst> {
        Box::new(Cst::Var(name))
    }

    pub fn abs(names: Vec<String>, body: Box<Cst>) -> Box<Cst> {
        Box::new(Cst::Abs(names, body))
    }

    pub fn app(l: Box<Cst>, r: Box<Cst>) -> Box<Cst> {
        Box::new(Cst::App(l, r))
    }

    pub fn par(inside: Box<Cst>) -> Box<Cst> {
        Box::new(Cst::Par(inside))
    }
}
