const DOC: &'static str =
"touch: missing file operand
Try \'touch --help\' for more infomation";

use std::fs::OpenOptions;
use std::io;
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
        println!("{}", DOC);
    }
    for _path in paths {
        touch(&Path::new(&_path)).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

}