extern crate "rustc-serialize" as rustc_serialize;

use std::fmt;

#[derive(RustcEncodable)]
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

#[derive(RustcEncodable)]
pub struct Expr {
    pub value: String
}

impl fmt::Show for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(RustcEncodable)]
pub struct Recipe {
    pub line: String
}

impl fmt::Show for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    {}\n", self.line)
    }
}