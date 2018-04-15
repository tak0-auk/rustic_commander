use std::env;

pub fn getArgs() -> Vec<String> {
    env::args().skip(1).collect()
}