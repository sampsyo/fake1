#![feature(plugin)]
#![plugin(peg_syntax_ext)]

// The new IO features are experimental. Remove this soon.
#![feature(fs)]
#![feature(io)]

extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::json;
use std::fs::File;
use std::io::Read;
use std::io;

mod ast;
peg_file! grammar("grammar.rustpeg");

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = try!(File::open(filename));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(contents)
}

fn main() {
    let fakefile = read_file("Fakefile").unwrap();

    let res = grammar::rulelist(fakefile.as_slice());
    let book = res.unwrap();

    println!("{}", book);
    print!("{}", json::as_pretty_json(&book));
}
