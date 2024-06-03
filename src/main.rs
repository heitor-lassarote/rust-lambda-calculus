use church::Church;
use de_bruijn::{eval, rename};
use lexer::Lexer;
use tok::Tok;

pub mod church;
pub mod de_bruijn;
pub mod lexer;
pub mod tok;

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
