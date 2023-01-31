use minigrep::{read_file, Config};
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let file_content: String = read_file(&config).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {err}");
        process::exit(1);
    });
    let line = match config.ignore_case {
        false => minigrep::search_case_insensitive(&config.query, &file_content),
        true => minigrep::search(&config.query, &file_content),
    };
    for ln in line.iter() {
        println!("{}", ln)
    }
}
