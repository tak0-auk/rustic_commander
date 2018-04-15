use std::{fs, mem};
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;

mod lib;

fn cat(path: &Path) -> io::Result<String> {
    let mut file = try!(fs::File::open(path));
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    for _path in lib::getArgs() {
        let mut file = BufReader::new(fs::File::open(&_path).unwrap());
        let mut buffer = vec![];
        file.read_to_end(&mut buffer);
        println!("{}", std::str::from_utf8(&buffer).unwrap());
    }
}