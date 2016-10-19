use std::io::prelude::*;
use std::fs::File;

fn main() {
    let s: &str = "Hello, world!";
    let resf = foo(s);
    let resb = bar(s);
}

fn foo(t: &str) -> std::io::Result<()> {
    let mut f = try!(File::create("foo.txt"));
    try!(f.write_all(t.as_bytes()));
    Ok(())
}

fn bar(t: &str)  -> std::io::Result<()> {
    let mut f = try!(File::open("foo.txt"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    assert_eq!(s, t);
    Ok(())
}
