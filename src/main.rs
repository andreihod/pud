mod lang;

#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

fn main() {
    let input = "2 + 2 * 4.0 oi 3 + 5 * 2";
    println!("'{}' evaluates to {:#?}", input, lang::evaluate(input));
}
