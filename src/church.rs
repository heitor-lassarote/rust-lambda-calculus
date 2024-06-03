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

    pub fn app(l: Rc<Church>, r: Rc<Church>) -> Rc<Church> {
        Rc::new(Church::App(l, r))
    }
}
