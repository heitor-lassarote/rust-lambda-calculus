use std::{collections::HashMap, rc::Rc};

use crate::church::Church;

pub type Idx = i32;
type Lvl = i32;

#[derive(Debug)]
pub enum DeBruijn {
    Bound(String, Idx),
    Free(String),
    Abs(String, Rc<DeBruijn>),
    App(Rc<DeBruijn>, Rc<DeBruijn>),
}

impl DeBruijn {
    pub fn bound(name: String, idx: Idx) -> Rc<DeBruijn> {
        Rc::new(DeBruijn::Bound(name, idx))
    }

    pub fn free(name: String) -> Rc<DeBruijn> {
        Rc::new(DeBruijn::Free(name))
    }

    pub fn abs(name: String, body: Rc<DeBruijn>) -> Rc<DeBruijn> {
        Rc::new(DeBruijn::Abs(name, body))
    }

    pub fn app(l: Rc<DeBruijn>, r: Rc<DeBruijn>) -> Rc<DeBruijn> {
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

pub fn rename(exp: Rc<Church>) -> Rc<DeBruijn> {
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

pub fn eval(exp: Rc<DeBruijn>) -> Rc<DeBruijn> {
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
