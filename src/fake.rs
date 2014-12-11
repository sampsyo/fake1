#![feature(phase)]
#[phase(plugin)]
extern crate peg_syntax_ext;

pub struct Rule {
    pub target: Expr
}

pub struct Expr {
    pub value: String
}

peg! grammar(r#"
use super::Rule;
use super::Expr;

#[pub]
rule -> Rule
    = t:expr ws ":" ws d:expr
    { Rule { target: t } }

ws -> ()
    = [ \t]*

expr -> Expr
    = i:ident
    { Expr { value: i } }

ident -> String
    = [A-Za-z0-9_-]+
    { match_str.into_string() }
"#)

fn main() {
    let res = grammar::rule("foo: bar");
    println!("{}", res.unwrap().target.value);
}
