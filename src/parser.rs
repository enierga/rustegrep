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
    OneOrMore(Box<AST>),
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

pub fn plus(val: AST) -> AST {
    AST::OneOrMore(Box::new(val))
}

pub fn cha(c: char) -> AST {
    AST::Char(c)
}

// dont really need factory for AnyChar

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<AST, String> {
        let mut parser = Parser {
            tokens: tokenizer.peekable(),
        };

        // start of recursive descent parsing, also checks if any tokens not parse at end
        let out = parser.reg_expr()?;
        if let Some(token) = parser.tokens.peek() {
            Err(format!("Expected end of input, found {:?}", token))
        } else {
            Ok(out)
        }
    }
}

/** write tests here for public api */
#[cfg(test)]
mod parser_parse {
    use super::*;

    #[test]
    fn simple_cat() {
        let par = Parser::parse(Tokenizer::new("aa")).unwrap();
        assert_eq!(cat(cha('a'), cha('a')), par);
    }

    #[test]
    fn simple_alt() {
        let par = Parser::parse(Tokenizer::new("a|b")).unwrap();
        assert_eq!(alt(cha('a'), cha('b')), par);
    }

    #[test]
    fn simple_clo() {
        let par = Parser::parse(Tokenizer::new("a*")).unwrap();
        assert_eq!(clo(cha('a')), par);
    }

    #[test]
    fn simple_plus() {
        let par = Parser::parse(Tokenizer::new("a+")).unwrap();
        assert_eq!(plus(cha('a')), par);
    }

    #[test]
    fn multi_cat() {
        let par = Parser::parse(Tokenizer::new("abc")).unwrap();
        assert_eq!(cat(cha('a'), cat(cha('b'), cha('c'))), par);
    }

    #[test]
    fn multi_alt() {
        let par = Parser::parse(Tokenizer::new("a|b|c")).unwrap();
        assert_eq!(alt(cha('a'), alt(cha('b'), cha('c'))), par);
    }

    #[test]
    fn cat_anychar() {
        let par = Parser::parse(Tokenizer::new("a.*")).unwrap();
        assert_eq!(cat(cha('a'), clo(AST::AnyChar)), par);
    }

    #[test] // this is almost absurdly long but i guess it was good practice to parse through lol
    fn everything() {
        let par = Parser::parse(Tokenizer::new("b(oo*|a).m")).unwrap();
        assert_eq!(cat(cha('b'), cat(alt(cat(cha('o'), clo(cha('o'))), cha('a')), cat(AST::AnyChar, cha('m')))), par);
    }
}

// this is the recursive descent chain for parsing
impl <'tokens> Parser<'tokens> {
    
    // RegExpr ::= Catenation (UnionBar RegExpr)?
    fn reg_expr(&mut self) -> Result<AST, String> {
        let cat_result = self.catenation()?;
        if let Some(t) = self.tokens.peek() {
            match t {
                Token::UnionBar => {
                    self.consume_token(Token::UnionBar);
                    Ok(alt(cat_result, self.reg_expr()?))
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
        if let Some(t) = self.tokens.peek() {
            match t {
                Token::LParen => {
                    Ok(cat(clo_result, self.catenation().unwrap()))
                },
                Token::AnyChar => Ok(cat(clo_result, self.catenation().unwrap())),
                Token::Char(c) => Ok(cat(clo_result, self.catenation().unwrap())),
                _ => Ok(clo_result),
            }
        } else {
            Ok(clo_result)
        }
    }

    // Closure ::= Atom [KleeneStar|KleenePlus]?
    fn closure(&mut self) -> Result<AST, String> {
        let atom_result = self.atom()?;
        if let Some(t) = self.tokens.peek() {
            match t {
                Token::KleeneStar => {
                    self.consume_token(Token::KleeneStar);
                    Ok(clo(atom_result))
                },
                Token::KleenePlus => {
                    self.consume_token(Token::KleenePlus);
                    Ok(plus(atom_result))
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
                let expr =  self.reg_expr();
                self.consume_token(Token::RParen)?;
                expr
            },
            Token::AnyChar => Ok(AST::AnyChar),
            Token::Char(c) => Ok(cha(c)),
            _ => Err(format!("Unexpected token: {:?}", t)),
        }
    }
}

/** write tests for private api */
#[cfg(test)]
mod parser_recur {
    use super::*;

    #[test]
    fn clear_paren() {
        assert_eq!(Parser::from("a").atom().unwrap(), cha('a'));
        assert_eq!(Parser::from("(a)").atom().unwrap(), cha('a'));
        assert_eq!(Parser::from("((a))").atom().unwrap(), cha('a'));
    }

    #[test]
    fn empty_paren() {
        assert_eq!(Parser::from("()").atom(), Err(String::from("Unexpected end of input")));
    }

    #[test]
    fn kleene_no_char() {
        assert_eq!(Parser::from("*").atom(), Err(String::from("Unexpected token: KleeneStar")));
    }

    #[test]
    fn no_union() {
        assert_eq!(Parser::from("a|").reg_expr(), Err(String::from("Unexpected end of input")));
    }

    #[test]
    fn unclosed_paren() {
        assert_eq!(Parser::from("(a").atom(), Err(String::from("Unexpected end of input")));
    }
}


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
