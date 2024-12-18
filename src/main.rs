// crate o lib :: modulo:: funcion
use lib::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(e) = run(config) {
        process::exit(1);
    }
}
