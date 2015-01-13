#![feature(plugin)]
#[plugin]
extern crate peg_syntax_ext;
extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::{json, Encodable};
use std::io::File;

mod ast;
peg_file! grammar("grammar.rustpeg");

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
