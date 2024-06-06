#[derive(Debug, PartialEq, Eq)]
pub enum Tok<IdType> {
    Lam,
    Dot,
    LPar,
    RPar,
    Id(IdType),
}
