pub mod helpers;

// Starter code for PS06 - thegrep
//
// Author(s): Vincent Enierga, Euael Ketema
// ONYEN(s): venierga, esplash
//
// UNC Honor Pledge: I pledge I have received no unauthorized aid
// on this assignment. I further pledge not to distribute my solution
// to this code to anyone other than the course staff and partner.
//

use self::State::*;
use super::parser::Parser;
use super::parser::AST;
use super::tokenizer::Tokenizer;
use std::ops;

/**
 * ===== Public API =====
 */

/**
 * An NFA is represented by an arena Vec of States
 * and a start state.
 */
#[derive(Debug)]
pub struct NFA {
    start: StateId,
    states: Vec<State>,
}

/**
 * Overloading the add operator for the nfa
 */

impl ops::Add<NFA> for NFA {
    type Output = NFA;

    fn add(self, rhs: NFA) -> NFA {
        let mut nfa = NFA::new();

        // add all lhs states except end state
        for state in self.states {
            match state {
                State::End => break,
                _ => nfa.add_state(state),
            };
        }
        
        // each state in rhs should be pushed down by length of lhs - end state
        let offset = nfa.states.len() - 1;

        // adding rhs to returned nfa with offset; skip rhs start state
        for state in rhs.states {
            match state {
                State::Start(Some(first_state)) => continue,
                State::Match(c, Some(next_state)) => {
                    nfa.add_state(Match(c, Some(next_state+offset)));
                },
                State::Split(Some(lnext_state), Some(rnext_state)) => {
                    nfa.add_state(Split(Some(lnext_state+offset), Some(rnext_state+offset)));
                },
                State::End => {
                    nfa.add_state(End);
                },
                _ => {},
            }
        }
        
        nfa
    }
}

#[cfg(test)]
mod add_op {
    use super::*;

    #[test]
    fn add_cat() {
        let sum = NFA::from("a").unwrap() + NFA::from("b").unwrap();
        assert!(sum.accepts("ab"));
        assert!(!sum.accepts("a"));
        assert!(!sum.accepts("b"));
    }

    #[test]
    fn add_cat_cat() {
        let sum = NFA::from("ab").unwrap() + NFA::from("cd").unwrap();
        assert!(sum.accepts("abcd"));
        assert!(!sum.accepts("abs"));
        assert!(sum.accepts("abcde"));
    }

    #[test]
    fn add_cat_alt() {
        let sum = NFA::from("a|b").unwrap() + NFA::from("c").unwrap();
        assert!(sum.accepts("ac"));
        assert!(sum.accepts("bc"));
        assert!(!sum.accepts("abc"));
    }

    #[test]
    fn add_alt_alt() {
        let sum = NFA::from("a|b").unwrap() + NFA::from("d|c").unwrap();
        assert!(sum.accepts("ac"));
        assert!(sum.accepts("bd"));
    }

    #[test]
    fn add_cat_clo() {
        let sum = NFA::from("a").unwrap() + NFA::from("b*").unwrap();
        assert!(sum.accepts("ab"));
        assert!(sum.accepts("a"));
        assert!(sum.accepts("abbbbbbbb"));
        assert!(!sum.accepts("bbbb"));
    }

    #[test]
    fn add_alt_clo() {
        let sum = NFA::from("a|b").unwrap() + NFA::from("c*").unwrap();
        assert!(sum.accepts("a"));
        assert!(sum.accepts("b"));
        assert!(sum.accepts("ac"));
        assert!(sum.accepts("bc"));
        assert!(sum.accepts("bccccccc"));
        assert!(sum.accepts("accccccc"));
    }

    #[test]
    fn add_clo_clo() {
        let sum = NFA::from("a*").unwrap() + NFA::from("b*").unwrap();
        assert!(sum.accepts("a"));
        assert!(sum.accepts("b"));
        assert!(sum.accepts("ab"));
        assert!(sum.accepts("aabb"));
    }
}

impl NFA {
    /**
     * Construct an NFA from a regular expression pattern.
     */
    pub fn from(regular_expression: &str) -> Result<NFA, String> {
        let mut nfa = NFA::new();

        let start = nfa.add_state(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add_state(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    /**
     * Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     */
    pub fn accepts(&self, input: &str) -> bool {
        let mut chars = input.chars();

        // initialize current set of states with start state then step to 1st state
        let mut c_states = vec![self.start];
        let mut n_states = self.nstate_gen(None, c_states);

        // this is where main computation happens
        // (its an iterative solution with a mild sprinkling of recursion)
        if !input.is_empty() {
            while let Some(c) = chars.next() {
                n_states = self.nstate_gen(Some(c), n_states);
            }
        } else {
            n_states = self.nstate_gen(None, n_states);
        }

        // acts as a '.*' at the end of a regex
        n_states = self.nstate_gen(None, n_states);

        // checks if there is an end state in resulting current states
        for id in n_states {
            match self.states[id] {
                State::End => return true,
                _ => continue,
            }
        }
        false
    }

    fn nstate_gen(&self, input_char: Option<char>, c_states: Vec<StateId>) -> Vec<StateId> {
        let mut n_states = Vec::new();

        for current in c_states {
            match &self.states[current] {
                State::Start(Some(first_state)) => n_states.push(*first_state),
                State::Match(char_enum, Some(next_state)) => {
                    // only run match state arm if there is an input char
                    if let Some(character) = input_char {
                        match *char_enum {
                            Char::Literal(c) => {
                                if character == c {
                                    n_states.push(*next_state);
                                }
                            }
                            Char::Any => n_states.push(*next_state),
                        }
                    }
                }
                State::Split(Some(lnext_state), Some(rnext_state)) => {
                    // if split state, test each split arm for matching the current char
                    for state in self.nstate_gen(input_char, vec![*rnext_state]) {
                        n_states.push(state);
                    }
                    for state in self.nstate_gen(input_char, vec![*lnext_state]) {
                        n_states.push(state);
                    }
                }
                _ => {
                    // push current state if nothing matches because its a current state anyway
                    n_states.push(current);
                }
            }
        }
        n_states
    }
}

/*
 * Write Tests for Public API
 */
#[cfg(test)]
mod nfa_accepts {
    use super::*;

    #[test]
    fn single_char() {
        let nfa = NFA::from("a").unwrap();
        assert_eq!(true, nfa.accepts("a"));
        assert_eq!(false, nfa.accepts(""));
    }

    #[test]
    fn simple_closure() {
        let nfa = NFA::from("a*").unwrap();
        assert_eq!(true, nfa.accepts("a"));
        assert_eq!(true, nfa.accepts(""));
        assert_eq!(true, nfa.accepts("aaaaaaaaaa"));
    }

    #[test]
    fn any_closure() {
        let nfa = NFA::from(".*").unwrap();
        assert_eq!(true, nfa.accepts(""));
        assert_eq!(true, nfa.accepts("bruhhh"));
    }

    #[test]
    fn simple_plus() {
        let nfa = NFA::from("a+").unwrap();
        assert_eq!(true, nfa.accepts("a"));
        assert_eq!(false, nfa.accepts(""));
        assert_eq!(true, nfa.accepts("aaaa"));
    }

    #[test]
    fn simple_catentation() {
        let nfa = NFA::from("ab").unwrap();
        let string = String::from("ab");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn alternating_closure() {
        let nfa = NFA::from("(a|b)*").unwrap();
        let string = String::from("ababa");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn simple_alternation() {
        let nfa = NFA::from("a|b").unwrap();
        assert_eq!(nfa.accepts("a"), true);
        assert_eq!(nfa.accepts("b"), true);
    }

    #[test]
    fn union_kleene() {
        let nfa = NFA::from("(a|b)*(c|d)*").unwrap();
        assert_eq!(true, nfa.accepts("ac"));
        assert_eq!(true, nfa.accepts("ad"));
        assert_eq!(true, nfa.accepts("bc"));
        assert_eq!(true, nfa.accepts(""));
    }

    #[test]
    fn closure_sandwich() {
        let nfa = NFA::from("(.*)a(.*)").unwrap();
        assert_eq!(nfa.accepts("poafs"), true);
        assert_eq!(
            nfa.accepts("bruh fucking a ugh why isnt this working"),
            true
        );
    }

    #[test]
    fn test_from_writeup() {
        let nfa = NFA::from("(.*)aut....a(.*)").unwrap();
        assert_eq!(nfa.accepts("Chautauqua"), true);
        assert_eq!(nfa.accepts("Chautauqua's"), true);
        assert_eq!(nfa.accepts("automata"), true);
        assert_eq!(nfa.accepts("beautification"), true);
        assert_eq!(nfa.accepts("beautification's"), true);
    }

    #[test]
    fn union_plus() {
        let nfa = NFA::from("(a|b)+(c|d)+").unwrap();
        assert_eq!(nfa.accepts("ad"), true);
        assert_eq!(nfa.accepts("aad"), true);
        assert_eq!(nfa.accepts("dd"), false);
    }

    #[test]
    fn any_char_catenation() {
        let nfa = NFA::from(".a.").unwrap();
        let string = String::from("asd");
        assert_eq!(false, nfa.accepts(&string));
    }

    #[test]
    fn sammy_with_letters() {
        let nfa = NFA::from("s(.*)e").unwrap();
        assert_eq!(nfa.accepts("sunshine"), true);
        assert_eq!(nfa.accepts("sale"), true);
        assert_eq!(nfa.accepts("s......e"), true);
    }

    #[test]
    fn alter_and_cat() {
        let nfa = NFA::from("a(x|y)+").unwrap();
        assert_eq!(nfa.accepts("ax"), true);
        assert_eq!(nfa.accepts("axxx"), true);
        assert_eq!(nfa.accepts("a"), false);
        assert_eq!(nfa.accepts("ayyy"), true);
    }

}

/**
 * ===== Internal API =====
 */
type StateId = usize;

/**
 * States are the elements of our NFA Graph
 * - Start is starting state
 * - Match is a state with a single matching transition out
 * - Split is a state with two epsilon transitions out
 * - End is the final accepting state
 */
#[derive(Debug)]
enum State {
    Start(Option<StateId>),
    Match(Char, Option<StateId>),
    Split(Option<StateId>, Option<StateId>),
    End,
}

/**
 * Chars are the matching label of a non-epsilon edge in the
 * transition diagram representation of the NFA.
 */
#[derive(Debug)]
enum Char {
    Literal(char),
    Any,
}

/**
 * Internal representation of a fragment of an NFA being constructed
 * that keeps track of the start ID of the fragment as well as all of
 * its unjoined end states.
 */
#[derive(Debug)]
struct Fragment {
    start: StateId,
    ends: Vec<StateId>,
}

/**
 * Private methods of the NFA structure.
 */
impl NFA {
    /**
     * Constructor establishes an empty states Vec.
     */
    fn new() -> NFA {
        NFA {
            states: vec![],
            start: 0,
        }
    }

    /**
     * Add a state to the NFA and get its arena ID back.
     */
    fn add_state(&mut self, state: State) -> StateId {
        let idx = self.states.len();
        self.states.push(state);
        idx
    }

    /**
     * Given an AST node, this method returns a Fragment of the NFA
     * representing it and its children.
     */
    fn gen_fragment(&mut self, ast: &AST) -> Fragment {
        match ast {
            AST::AnyChar => {
                let state = self.add_state(Match(Char::Any, None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Char(c) => {
                let state = self.add_state(Match(Char::Literal(*c), None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Catenation(lhs, rhs) => self.cat_helper(lhs, rhs),
            AST::Alternation(lhs, rhs) => {
                let ends = Vec::new();
                self.alt_helper(lhs, rhs, ends)
            }
            AST::Closure(ast) => self.clo_helper(ast),
            AST::OneOrMore(ast) => self.plus_helper(ast),
            node => panic!("Unimplemented branch of gen_fragment: {:?}", node),
        }
    }

    /**
     * Join all the loose ends of a fragment to another StateId.
     */
    fn join_fragment(&mut self, lhs: &Fragment, to: StateId) {
        for end in &lhs.ends {
            self.join(*end, to);
        }
    }

    /**
     * Join a loose end of one state to another by IDs.
     * Note in the Split case, only the 2nd ID (rhs) is being bound.
     * It is assumed when building an NFA with these constructs
     * that the lhs of an Split state will always be known and bound.
     */
    fn join(&mut self, from: StateId, to: StateId) {
        match self.states[from] {
            Start(ref mut next) => *next = Some(to),
            Match(_, ref mut next) => *next = Some(to),
            Split(_, ref mut next) => *next = Some(to),
            End => {}
        }
    }

    /**
     * this is a helper function for catenation
     */
    fn cat_helper(&mut self, lhs: &AST, rhs: &AST) -> Fragment {
        let left = self.gen_fragment(lhs);
        let right = self.gen_fragment(rhs);
        if right.start < self.states.len() {
            // leave last state unjoined so it can later be joined to end
            self.join_fragment(&left, right.start); // joining these two fragments together
        }
        Fragment {
            //  creating fragment that has left's start and right's end
            start: left.start,
            ends: right.ends,
        }
    }

    /**
     * helper for alternation
     */
    fn alt_helper(&mut self, lhs: &AST, rhs: &AST, mut ends: Vec<StateId>) -> Fragment {
        let left = self.gen_fragment(lhs);
        for end in left.ends {
            // this is meant to "collect" those loose ends from the fragments
            ends.push(end);
        }
        let right = self.gen_fragment(rhs);
        for end in right.ends {
            ends.push(end);
        }
        let state = self.add_state(Split(Some(left.start), Some(right.start))); // create split state with left + right
        Fragment {
            start: state,
            ends: ends,
        }
    }

    /**
     * attempting closure helper here (closure = split state + match state)
     */
    fn clo_helper(&mut self, ast: &AST) -> Fragment {
        let kleene_char = self.gen_fragment(ast); // generate fragment for the closure ast
        let state = self.add_state(Split(Some(kleene_char.start), None)); // creating split state with match state at lhs
        self.join_fragment(&kleene_char, state); // join closure ast and split state
        Fragment {
            start: state,
            ends: vec![state],
        }
    }

    /**
     * one or more = match state + split state (lhs points back to match, rhs points forward)
     */
    fn plus_helper(&mut self, ast: &AST) -> Fragment {
        let plus_char = self.gen_fragment(ast); // generating frag for oneormore ast
        let state = self.add_state(Split(Some(plus_char.start), None)); // create split state with lhs pointing to match
        self.join_fragment(&plus_char, state);
        Fragment { // unlike in closure, the start of this fragment is at the oneormore frag
            start: plus_char.start,
            ends: vec![state],
        }
    }
}
