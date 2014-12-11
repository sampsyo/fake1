#![feature(phase)]
#[phase(plugin)]
extern crate peg_syntax_ext;

pub struct Something {
    pub name: String
}

peg! grammar(r#"
use super::Something;

#[pub]
stuff -> Something
    = [a-z]+ { Something { name: match_str.into_string() } }
"#)

fn main() {
    let res = grammar::stuff("hello");
    println!("{}", res.unwrap().name);
}
