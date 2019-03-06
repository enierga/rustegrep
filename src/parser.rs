use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

/**
 * Tar Heel egrep - parser
 *
 */

#[derive(Debug, PartialEq)]
pub enum AST {
    Alternation(Box<AST>, Box<AST>),
    Catenation(Box<AST>, Box<AST>),
    Closure(Box<AST>),
    Char(char),
    AnyChar
}

/* factory helper funcs because why not */
pub fn alt(left: AST, right: AST) -> AST {
    AST::Alternation(Box::new(left), Box::new(right))
}

pub fn cat(left: AST, right: AST) -> AST {
    AST::Catenation(Box::new(left), Box::new(right))
}

pub fn clo(val: AST) -> AST {
    AST::Closure(Box::new(val))
}

pub fn cha(c: char) -> AST {
    AST::Char(c)
}

// dont really need one for AnyChar

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<AST, String> {
        let mut parser = Parser {
            tokens: tokenizer.peekable(),
        };

        // start of recursive descent parsing, also checks if any tokens not parse at end
        let out = parser.reg_expr();
        if let Some(token) = parser.token.peek() {
            Err(format!("Expected end of input, found {:?}", token))
        }
        out
    }
}

/** write tests here for public api */


// this is the recursive descent chain for parsing
impl <'tokens> Parser<'tokens> {
    
    // RegExpr ::= Catenation (UnionBar RegExpr)?
    fn reg_expr(&mut self) -> Result<AST, String> {
        let cat_result = self.catenation()?;
        if let Some(t) = self.peek_next() {
            match t {
                Token::UnionBar => {
                    self.consume_token(Token::UnionBar);
                    alt(cat_result, self.reg_expr().unwrap())
                },
                _ => Ok(cat_result),
            }
        } else {
            Ok(cat_result)
        }
    }


    // Catenation ::= Closure (Catenation)?
    fn catenation(&mut self) -> Result<AST, String> {
        let clo_result = self.closure()?;
        if let Some(t) = self.peek_next() {
            match t {
                Token::LParen => {
                    self.consume_token(Token::LParen);
                    return Ok(cat(clo_result, self.catenation()));
                    self.consume_token(Token::RParen);
                },
                Token::AnyChar | Token::Char(c) => {
                    Ok(cat(clo_result, self.catenation()))
                },
                _ => Ok(clo_result),
            }
        } else {
            Ok(clo_result)
        }
    }

    // Closure ::= Atom [KleeneStar]?
    fn closure(&mut self) -> Result<AST, String> {
        let atom_result = self.atom()?;
        if let Some(t) = self.peek_next() {
            match t {
                Token::KleeneStar => {
                    self.consume_token(Token::KleeneStar);
                    clo(atom_result)
                },
                _ => Ok(atom_result),
            }
        } else {
            Ok(atom_result)
        }
    }

    // Atom ::= [LParen] RegExpr [RParen]|[AnyChar]|[Char]
    fn atom(&mut self) -> Result<AST, String> {
        let t = self.take_next_token()?;
        match t {
            Token::LParen => {
                return Ok(self.reg_expr());
                self.consume_token(Token::RParen);
            },
            Token::AnyChar => Ok(AST::AnyChar),
            Token::Char(c) => Ok(cha(c)),
            _ => Err(format!("Unexpected token: {:?}", t)),
        }
    }
}

/** write tests for private api */


// helper methods to make parsing a tad easier
impl<'tokens> Parser<'tokens> {
    // we'll use this method for testing of the parser
    fn from(input: &'tokens str) -> Parser<'tokens> {
        Parser {
            tokens: Tokenizer::new(input).peekable(),
        }
    }

    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    fn peek_next(&mut self) -> Option<Token> {
        if let Some(t) = self.tokens.peek() {
            Some(*t)
        } else {
            None
        }
    }

    fn consume_token(&mut self, expected: Token) -> Result<Token, String> {
        if let Some(next) = self.tokens.next() {
            if next != expected {
                Err(format!("Expected: {:?} - Found {:?}", expected, next))
            } else {
                Ok(next)
            }
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }


}
