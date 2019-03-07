#![allow(unused)]
/**
 * thegrep - Tar Heel egrep
 *
 * Author(s): Vincent Enierga
 * ONYEN(s): venierga
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff and partner.
 */
extern crate structopt;

const EXIT_OK: i32 = 0;
const EXIT_ERR: i32 = 1;

use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "thegrep", about = "Tar Heel egrep")]
struct Options {
    #[structopt(short = "p", long = "parse")]
    parse: bool,
    #[structopt(short = "t", long = "tokens")]
    tokens: bool,
}

pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;

fn main() {
    let options = Options::from_args();
    loop {
        eval(&read(), &options);
    }
}

fn eval(input: &str, options: &Options) {
    if options.parse {
        eval_parse(input);
    }
    if options.tokens {
        eval_tokens(input);
    }
}

// print helpers for each flag
fn eval_tokens(input: &str) {
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    print!("\n");
}

fn eval_parse(input: &str) {
    match Parser::parse(Tokenizer::new(input)) {
        Ok(fine) => {
            println!("{:?}", fine);
        }
        Err(error) => eprintln!("thegrep: {}", error),
    }
    print!("\n");
}

fn read() -> String {
    match read_line() {
        Ok(line) => {
            return line;
            std::process::exit(EXIT_OK);
        }
        Err(error) => {
            eprintln!("Err: {}", error);
            std::process::exit(EXIT_ERR);
        }
    }
}

fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
