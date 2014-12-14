#![feature(phase)]
#[phase(plugin)]
extern crate peg_syntax_ext;

pub struct Rule {
    pub target: Expr,
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
rule -> Rule
    = t:expr ws ":" ws d:expr ws r:recipe*
    { Rule { target: t, recipe: r } }

recipe -> Recipe
    = "\n" [ \t]+ l:line
    { Recipe { line: l } }

ws -> ()
    = [ \t]*

expr -> Expr
    = i:ident
    { Expr { value: i } }

ident -> String
    = [A-Za-z0-9_-]+
    { match_str.into_string() }

line -> String
    = [A-Za-z0-9 \t_-]+
    { match_str.into_string() }
"#)

fn main() {
    let res = grammar::rule("foo: bar\n    baz\n    qux");
    let rule = res.unwrap();
    println!("{} {}", rule.target.value, rule.recipe[0].line);
}
