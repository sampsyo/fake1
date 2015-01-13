#![feature(plugin)]
#[plugin]
extern crate peg_syntax_ext;
extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::{json, Encodable};

use std::io::File;

mod ast;

peg! grammar(r#"
use ast::Rule;
use ast::Expr;
use ast::Recipe;

#[pub]
rulelist -> Vec<Rule>
    = r:rule*
    { r }

rule -> Rule
    = t:exprlist ws ":" ws d:exprlist ws r:recipe* [\n]*
    { Rule { targets: t, deps: d, recipe: r } }

recipe -> Recipe
    = "\n" [ \t]+ l:line
    { Recipe { line: l } }

ws -> ()
    = [ \t]*

expr -> Expr
    = i:ident ws
    { Expr { value: i } }

exprlist -> Vec<Expr>
    = e:expr*
    { e }

ident -> String
    = [A-Za-z0-9_-]+
    { String::from_str(match_str) }

line -> String
    = [A-Za-z0-9 \t_-]+
    { String::from_str(match_str) }
"#);

fn pretty_encode<T : Encodable>(o: &T) -> String {
    let mut buffer: Vec<u8> = Vec::new();

    // Using an inner scope limits the duration of the borrow, allowing us
    // to use the buffer again below to extract its string.
    {
        let mut encoder = json::PrettyEncoder::new(&mut buffer);
        o.encode(&mut encoder).ok().expect("JSON encode failed");
    }

    String::from_utf8(buffer).unwrap()
}

fn main() {
    let content = File::open(&Path::new("Fakefile")).read_to_end();
    let fakefile = String::from_utf8(content.unwrap()).unwrap();

    let res = grammar::rulelist(fakefile.as_slice());
    let rules = res.unwrap();

    for rule in rules.iter() {
        print!("{}\n", rule);
    }

    print!("{}", pretty_encode(&rules));
}
