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

pub mod nfa;
use self::nfa::helpers::nfa_dot;
use self::nfa::NFA;
use std::io;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "thegrep", about = "Tar Heel egrep")]
struct Options {
    #[structopt(short = "d", long = "dot")]
    /// Show DOT representation of NFA
    dot: bool,
    #[structopt(short = "p", long = "parse")]
    /// Show Parsed AST
    parse: bool,
    #[structopt(short = "t", long = "tokens")]
    /// Show Tokens
    tokens: bool,
    #[structopt(short = "g", long = "gen")]
    /// Show n Acceptable Strings
    n: Option<usize>,

    /// Regular Expression Pattern
    pattern: String,

    #[structopt(help = "files")]
    paths: Vec<String>,
}

pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;
pub mod string_gen;
use self::string_gen::StringGen;

fn main() {
    let options = Options::from_args();
    let input = Options::from_args().pattern;
    eval(&input, &options);
}

fn eval(input: &str, options: &Options) {
    if options.dot {
        eval_dot(input);
    }
    if options.parse {
        eval_parse(input);
    }
    if options.tokens {
        eval_tokens(input);
    }
    if let Some(number) = options.n {
        eval_gen(input, number);
    }

    // modifying input to allow pattern matching within strings
    let mut mod_input = String::from("(.*)");
    mod_input.push_str(input);
    mod_input.push_str("(.*)");

    let result = if options.paths.len() > 0 {
        print_files(&mod_input, options)
    } else {
        print_stdin(&mod_input)
    };
}

use std::fs::File;
use std::io::BufRead;

// for file input
fn print_files(input: &str, options: &Options) -> io::Result<()> {
    for path in options.paths.iter() {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        print_output(input, reader)?;
    }
    Ok(())
}

// for user input
fn print_stdin(input: &str) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    print_output(input, reader)
}

// generically printing from different sources with method below (borrowed from lecture 18 lol)
fn print_output<R: BufRead>(input: &str, reader: R) -> io::Result<()> {
    let nfa = NFA::from(input).unwrap();
    for line in reader.lines() {
        let line_in = &*line?;
        if nfa.accepts(line_in) {
            println!("{}", line_in);
        }
    }
    Ok(())
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

fn eval_dot(input: &str) {
    let nfa = NFA::from(&input).unwrap();
    println!("{}", nfa_dot(&nfa));
    std::process::exit(0);
}

fn eval_gen(input: &str, number: usize) {
    match StringGen::generate(input, number) {
        Ok(fine) => {
            for string in fine {
                println!("{:?}", string);
            }
        }
        Err(error) => eprintln!("thegrep: {}", error),
    }
    process::exit(EXIT_OK);
}
