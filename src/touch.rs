use std::fs::OpenOptions;
use std::io;
use std::io::{stderr, Write};
use std::path::Path;

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().append(true).create(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.len() < 1 {
        writeln!(&mut stderr(), "file name");
    }
    for _path in paths {
        touch(&Path::new("e.txt")).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

}