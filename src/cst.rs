use std::rc::Rc;

#[derive(Debug)]
pub enum Cst {
    Var(String),
    Abs(Vec<String>, Rc<Cst>),
    App(Rc<Cst>, Rc<Cst>),
    Par(Rc<Cst>),
}

impl Cst {
    pub fn var(name: String) -> Rc<Cst> {
        Rc::new(Cst::Var(name))
    }

    pub fn abs(names: Vec<String>, body: Rc<Cst>) -> Rc<Cst> {
        Rc::new(Cst::Abs(names, body))
    }

    pub fn app(l: Rc<Cst>, r: Rc<Cst>) -> Rc<Cst> {
        Rc::new(Cst::App(l, r))
    }

    pub fn par(inside: Rc<Cst>) -> Rc<Cst> {
        Rc::new(Cst::Par(inside))
    }
}
