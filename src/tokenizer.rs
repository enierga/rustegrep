use std::iter::Peekable;
use std::str::Chars;

/**
 * Tar Heel egrep: Tokenizer
 * 
 */

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    UnionBar,
    KleeneStar,
    AnyChar,
    Char(char),
}

pub struct Tokenizer<'str> {
    chars: Peekable<Chars<'str>>,
}

impl<'str> Tokenizer<'str> {
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.whitespace();
        if let Some(c) = self.chars.peek() {
            Some(match c {
                '(' | ')' => self.paren(),
                '|' => self.union(),
                '*' => self.kleene(),
                '.' => self.any_char(),
                _ => self.other_chars(),
            })
        } else {
            None
        }
    }
}

// helper methods for each token below
impl<'str> Tokenizer<'str> {
    fn whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\n' => self.chars.next(),
                _ => break,
            };
        }
    }

    fn paren(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => panic!("Paren: this isnt supposed to happen"),
        }
    }

    fn union(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        if c != '|' {
            panic!("Union: this isnt supposed to happen");
        }
        Token::UnionBar
    }

    fn kleene(&mut self) {
        let c = self.chars.next().unwrap();
        if c != '*' {
            panic!("Kleene Star: this isnt supposed to happen");
        }
        Token::KleeneStar
    }

    fn any_char(&mut self) {
        let c = self.chars.next().unwrap();
        if c != '.' {
            panic!("Any Char: this isnt supposed to happen");
        }
        Token::AnyChar
    }

    fn other_chars(&mut self) {
        let c = self.chars.next.unwrap();
        Token::Char(c)
    }


    /**
     * Unit Tests
     *
     */
    #[cfg(test)]
    mod iterator {
        use super::*;

        #[test]
        fn empty() {
            let mut tokens = Tokenizer::new("");
            assert_eq!(tokens.next(), None);
            assert_eq!(tokens.next(), None);
        }

        #[test]
        fn char() {
            let mut tokens = Tokenizer::new("abc");
            assert_eq!(tokens.next(), Token::Char('a'));
            assert_eq!(tokens.next(), Token::Char('b'));
            assert_eq!(tokens.next(), Token::Char('c'));
        }

        #[test]
        fn union_and_paren() {
            let mut tokens = Tokenizer::new("ab|().*");
            assert_eq!(tokens.next(), Token::Char('a'));
            assert_eq!(tokens.next(), Token::Char('b'));
            assert_eq!(tokens.next(), Token::UnionBar);
            assert_eq!(tokens.next(), Token::LParen);
            assert_eq!(tokens.next(), Token::RParen);
            assert_eq!(tokens.next(), Token::AnyChar);
            assert_eq!(tokens.next(), Token::KleeneStar);
        }
            
    }
}
