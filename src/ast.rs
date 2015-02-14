extern crate "rustc-serialize" as rustc_serialize;

use std::fmt;

#[derive(RustcEncodable)]
pub struct Cookbook {
    pub rules: Vec<Rule>,
}

#[derive(RustcEncodable)]
pub struct Rule {
    pub targets: Vec<Expr>,
    pub deps: Vec<Expr>,
    pub recipe: Vec<Recipe>,
}

#[derive(RustcEncodable)]
pub struct Expr {
    pub value: String,
}

#[derive(RustcEncodable)]
pub struct Recipe {
    pub line: String,
}

fn fmt_join<T: fmt::Display>(stuff: &Vec<T>, sep: &str,
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

impl fmt::Display for Cookbook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_join(&self.rules, "\n", f)
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt_join(&self.targets, " ", f));
        try!(write!(f, ": "));

        try!(fmt_join(&self.deps, " ", f));
        try!(write!(f, "\n"));

        for step in self.recipe.iter() {
            try!(step.fmt(f));
        }

        Ok(())
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    {}\n", self.line)
    }
}
