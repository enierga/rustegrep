#![allow(unused)]

/**
 *
 * Tar Heel egrep - random string generator
 *
 *
 * this generates a specified number of strings from 
 * a given regex... this should be pretty dope i think.
 *
 *
 */

use super::nfa::NFA;
use super::parser::Parser;
use super::parser::AST;
use super::tokenizer::Tokenizer;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub struct StringGen {
    rand_strings: Vec<String>,
}

impl StringGen {
    
    // return vec of generated strings and print in main
    pub fn generate(regex: &str, n: usize) -> Result<Vec<String>, String> {
        let mut generator = StringGen::new();
        let ast = &Parser::parse(Tokenizer::new(regex))?;

        while generator.rand_strings.len() < n {
            generator.rand_strings.push(generator.string_factory(ast));
        }

        Ok(generator.rand_strings)
    }

    fn new() -> StringGen {
        StringGen {
            rand_strings: vec![],
        }
    }

    // r e c u r s i v e - d e s c e n t
    fn string_factory(&self, ast: &AST) -> String {
        let mut rand_string = String::new();

        match ast {
            AST::AnyChar => self.rand_anychar(&mut rand_string),
            AST::Char(c) => self.rand_char(&mut rand_string, *c),
            AST::Catenation(lhs, rhs) => self.rand_catenation(&mut rand_string, lhs, rhs),
            AST::Alternation(lhs, rhs) => self.rand_alternation(&mut rand_string, lhs, rhs),
            AST::Closure(clo) => self.rand_closure(&mut rand_string, clo),
        }
        rand_string
    }

    fn rand_anychar(&self, rand_string: &mut String) {
        let mut rng = rand::thread_rng();
        rand_string.push(rng.sample(Alphanumeric))
    }

    fn rand_char(&self, rand_string: &mut String, c: char) {
        rand_string.push(c);
    }

    fn rand_catenation(&self, rand_string: &mut String, lhs: &AST, rhs: &AST) {
        rand_string.push_str(&self.string_factory(&lhs));
        rand_string.push_str(&self.string_factory(&rhs));
    }

    fn rand_alternation(&self, rand_string: &mut String, lhs: &AST, rhs: &AST) {
        let left = rand::random();
        if left {
            rand_string.push_str(&self.string_factory(&lhs));
        } else {
            rand_string.push_str(&self.string_factory(&rhs));
        }
    }

    fn rand_closure(&self, rand_string: &mut String, ast: &AST) {
        let mut rng = rand::thread_rng();
        let rand_int = rng.gen_range(0, 50);

        for i in 0..rand_int {
            rand_string.push_str(&self.string_factory(&ast));
        }
    }


}

#[cfg(test)]
mod string_gen {
    use super::*;

    #[test]
    fn rand_anychar() {
        let regex = ".";
        let nfa = NFA::from(regex).unwrap();
        let rand_strings = StringGen::generate(regex, 3).unwrap();
        for string in rand_strings {
            assert_eq!(nfa.accepts(&string), true);
        }
    }

    #[test]
    fn rand_alternation() {
        let regex = "a|b";
        let nfa = NFA::from(regex).unwrap();
        let rand_strings = StringGen::generate(regex, 3).unwrap();
        for string in rand_strings {
            assert_eq!(nfa.accepts(&string), true);
        }
    }

    #[test]
    fn rand_closure() {
        let regex = "a*";
        let nfa = NFA::from(regex).unwrap();
        let rand_strings = StringGen::generate(regex, 3).unwrap();
        for string in rand_strings {
            assert_eq!(nfa.accepts(&string), true);
        }
    }

    #[test]
    fn rand_string1() {
        let regex = "big* chungus*";
        let nfa = NFA::from(regex).unwrap();
        let rand_strings = StringGen::generate(regex, 3).unwrap();
        for string in rand_strings {
            assert_eq!(nfa.accepts(&string), true);
        }
    }

    #[test]
    fn rand_string2() {
        let regex = "(ass*)|(booty*) eaters*";
        let nfa = NFA::from(regex).unwrap();
        let rand_strings = StringGen::generate(regex, 3).unwrap();
        for string in rand_strings {
            assert_eq!(nfa.accepts(&string), true);
        }
    }

    #[test]
    fn rand_string3() {
        let regex = "hi, my name .....";
        let nfa = NFA::from(regex).unwrap();
        let rand_strings = StringGen::generate(regex, 3).unwrap();
        for string in rand_strings {
            assert_eq!(nfa.accepts(&string), true);
        }
    }
}
