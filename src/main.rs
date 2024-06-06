use church::Church;
use de_bruijn::{eval, rename};
use lexer::Lexer;
use parser::Parser;

pub mod church;
pub mod cst;
pub mod de_bruijn;
pub mod lexer;
pub mod parser;
pub mod tok;

fn main() {
    //let input = r"(x)";
    //let input = r"(λx. λy. x) a";
    //let input = r"(λx. λy. x) a b";
    let input = r"(λx y z. x) a b c";
    //let input = r"a b (c)";
    //let input = r"a b c";
    //let input = r"(a b) (c d)";
    println!("Input:     {:?}", input);
    let lexer = Lexer::new(input);
    let tokens: Vec<tok::Tok<String>> = Lexer::new(input).collect();
    println!("Tokens:    {:?}", tokens);
    let cst = Parser::new(lexer).parse().unwrap();
    println!("Parsed:    {:?}", cst);
    let exp = Church::desugar(cst);
    println!("Desugared: {:?}", exp);
    let de_bruijn = rename(exp);
    println!("De Bruijn: {:?}", de_bruijn);
    let result = eval(de_bruijn);
    println!("Result:    {:?}", result);
}
