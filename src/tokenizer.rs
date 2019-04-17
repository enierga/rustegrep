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

/** test the 'next' method above */
#[cfg(test)]
mod tokenizer {
    use super::*;

    #[test]
    fn empty() {
        let mut tokens = Tokenizer::new("");
        assert_eq!(tokens.next(), None);
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn parentheses_w_chars() {
        let mut tokens = Tokenizer::new("(yuh)");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::Char('y')));
        assert_eq!(tokens.next(), Some(Token::Char('u')));
        assert_eq!(tokens.next(), Some(Token::Char('h')));
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn union_w_chars() {
        let mut tokens = Tokenizer::new("y|n");
        assert_eq!(tokens.next(), Some(Token::Char('y')));
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), Some(Token::Char('n')));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn kleene_star_w_char() { 
        let mut tokens = Tokenizer::new("x*");
        assert_eq!(tokens.next(), Some(Token::Char('x')));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn any_char_w_chars() {
        let mut tokens = Tokenizer::new("a.b");
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), Some(Token::Char('b')));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn all_tokens() {
        let mut tokens = Tokenizer::new("(a|b).c*");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), Some(Token::Char('b')));
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), Some(Token::Char('c')));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), None);
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


