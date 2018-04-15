const DOC: &'static str =
"cp
";

use std::env;
use std::error::Error;
use std::fs::{ File, OpenOptions };
use std::io;
use std::result;
use std::io::{BufWriter, Write };
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // parse_args();

    if args.len() == 1 {
        print!("{}", DOC);
        process::exit(0);
    }

    let path = Path::new(&args[1]);
    let target = Path::new(&args[2]);

    let display = path.display();

    if path.is_dir() {
        print!("directory is not supported");
        process::exit(0);
    }

    let mut file = File::open(&path).unwrap();

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!(""),
    }

//　コピー先ファイル作成
    let mut duplication = match File::create(&target) {
        Err(expr) => panic!("{}", expr),
        Ok(file) => file,
    };

    match duplication.write_all(content.as_bytes()) {
        Err(expr) => panic!("{}", expr),
        Ok(_) => 0,
    };
}

// fn parse_args() {
//     let args: Vec<String> = env::args().collect();

//     let mut opts = Options::new();

//     opts.optopt("f", "", "", "hint");

//     opts.optflag("h", "help", "print this help menu");

//     let matches = match opts.parse(&args[1..]) {
//         Ok(m) => { m }
//         Err(f) => { panic!(f.to_string())}
//     };

//     if matches.opt_present("h"){
//         print!("-h");
//     }

// }

// fn open_file(name: &str) -> result::Result<File, io::Error> {
//     OpenOptions::new().read(true).open(name)
// }