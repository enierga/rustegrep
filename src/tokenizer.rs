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
            _ => panic!("Expected parentheses"),
        }
    }

    fn union(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        if c != '|' {
            panic!("Expected union bar");
        }
        Token::UnionBar
    }

    fn kleene(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        if c != '*' {
            panic!("Expected Kleene star");
        }
        Token::KleeneStar
    }

    fn any_char(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        if c != '.' {
            panic!("Expected any char");
        }
        Token::AnyChar
    }

    fn other_chars(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        Token::Char(c)
    }
}
