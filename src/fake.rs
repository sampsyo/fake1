#![feature(phase)]
#[phase(plugin)]
extern crate peg_syntax_ext;

use std::io::File;

pub struct Rule {
    pub targets: Vec<Expr>,
    pub deps: Vec<Expr>,
    pub recipe: Vec<Recipe>
}

pub struct Expr {
    pub value: String
}

pub struct Recipe {
    pub line: String
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
    { match_str.into_string() }

line -> String
    = [A-Za-z0-9 \t_-]+
    { match_str.into_string() }
"#)

fn main() {
    let content = File::open(&Path::new("Fakefile")).read_to_end();
    let fakefile = String::from_utf8(content.unwrap()).unwrap();

    let res = grammar::rulelist(fakefile.as_slice());
    let rules = res.unwrap();
    println!("{} {}", rules[0].targets[0].value, rules[0].recipe[0].line);
}
