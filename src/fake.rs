#![feature(plugin)]

#[plugin]
extern crate peg_syntax_ext;
extern crate serialize;

use std::io::File;
use std::fmt;
use serialize::{json, Encodable, Encoder};

#[derive(Encodable)]
pub struct Rule {
    pub targets: Vec<Expr>,
    pub deps: Vec<Expr>,
    pub recipe: Vec<Recipe>
}

fn write_join<T: fmt::Show>(stuff: &Vec<T>, sep: &str,
                            f: &mut fmt::Formatter) -> fmt::Result {
    let mut first = true;
    for v in stuff.iter() {
        if !first {
            try!(write!(f, "{}", sep));
        }
        first = false;
        try!(v.fmt(f));
    }
    Ok(())
}

impl fmt::Show for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write_join(&self.targets, " ", f));
        try!(write!(f, ": "));

        try!(write_join(&self.deps, " ", f));
        try!(write!(f, "\n"));

        for step in self.recipe.iter() {
            try!(step.fmt(f));
        }

        Ok(())
    }
}

#[derive(Encodable)]
pub struct Expr {
    pub value: String
}

impl fmt::Show for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Encodable)]
pub struct Recipe {
    pub line: String
}

impl fmt::Show for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    {}\n", self.line)
    }
}

peg! grammar(r#"
use super::Rule;
use super::Expr;
use super::Recipe;

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
    let mut encoder = json::PrettyEncoder::new(&mut buffer);
    o.encode(&mut encoder).ok().expect("JSON encode failed");
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
