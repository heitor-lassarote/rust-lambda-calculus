#[derive(Debug)]
pub enum Tok {
    Lam,
    Dot,
    LPar,
    RPar,
    Id(String),
}
