#![feature(plugin)]
#[plugin]
extern crate peg_syntax_ext;
extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::json;
use std::io::File;

mod ast;
peg_file! grammar("grammar.rustpeg");

fn main() {
    let content = File::open(&Path::new("Fakefile")).read_to_end();
    let fakefile = String::from_utf8(content.unwrap()).unwrap();

    let res = grammar::rulelist(fakefile.as_slice());
    let book = res.unwrap();

    println!("{}", book);
    print!("{}", json::as_pretty_json(&book));
}
