pub mod helpers;

// Starter code for PS06 - thegrep
// 
// Author(s): Vincent Enierga, Euael Ketema
// ONYEN(s): venierga, esplash

//
// Author(s): Vincent Enierga
// ONYEN(s): venierga

//
// UNC Honor Pledge: I pledge I have received no unauthorized aid
// on this assignment. I further pledge not to distribute my solution
// to this code to anyone other than the course staff and partner.
//

use self::State::*;
use super::parser::Parser;
use super::parser::AST;
use super::tokenizer::Tokenizer;

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

impl NFA {
    /**
     * Construct an NFA from a regular expression pattern.
     */
    pub fn from(regular_expression: &str) -> Result<NFA, String> {
        let mut nfa = NFA::new();

        let start = nfa.add(Start(None));
        nfa.start = start;

        // Parse the Abstract Syntax Tree of the Regular Expression
        let ast = &Parser::parse(Tokenizer::new(regular_expression))?;
        // The "body" of the NFA is made of the states between Start and End
        let body = nfa.gen_fragment(ast);
        nfa.join(nfa.start, body.start);

        let end = nfa.add(End);
        nfa.join_fragment(&body, end);

        Ok(nfa)
    }

    /**
     * Given an input string, simulate the NFA to determine if the
     * input is accepted by the input string.
     */
    pub fn accepts(&self, input: &str) -> bool {
        let mut result = false;
        let mut chars = input.chars();
        let mut c_states = vec![self.start];

        // initializing current states with first state after start, if it exists
        if let State::Start(Some(first_state)) = self.states[c_states[0]] {
            c_states.push(first_state);
            c_states.swap_remove(0);
        }

        // this is where main computation happens
        self.accept_helper(input, &mut c_states);

        // checks if there is an end state in resulting current states
        for id in c_states {
            match self.states[id] {
                State::End => result = true,
                _ => continue,
            }
        }
        result
    }

    // returns next set of current states
    fn accept_helper(&self, input: &str, c_states: &mut Vec<StateId>) {
        let mut input_string = input.chars();

        loop {
            let mut character = input_string.next();
            let mut cstate_index = 0;
            let mut total_states = c_states.len();
            while cstate_index < total_states {
                match &self.states[c_states[cstate_index]] {
                    State::Match(char_enum, Some(next_state)) => {
                        if let Some(current_char) = character {
                            match char_enum {
                                Char::Literal(c) => {
                                    if current_char == *c {
                                        c_states.push(*next_state);
                                        c_states.swap_remove(cstate_index);
                                    }
                                }
                                Char::Any => {
                                    c_states.push(*next_state);
                                    c_states.swap_remove(cstate_index);
                                }
                            }
                        }
                        cstate_index += 1;
                    }
                    State::Split(Some(left_state), Some(right_state)) => {
                        c_states.push(*left_state);
                        c_states.swap_remove(cstate_index);
                        c_states.push(*right_state);
                        total_states += 1;
                    }
                    State::End => {
                        c_states.remove(cstate_index);
                        total_states -= 1;
                    }
                    _ => break,
                }
            }
            if let None = character {
                break;
            }
        }
    }
}

/*
 * Write Tests for Public API
 */
#[cfg(test)]
mod nfa_tests {
    use super::*;
    
    #[test]
    fn single_char() {
        let nfa = NFA::from("a*").unwrap();
        let string = String::from("a");
        assert_eq!(true,nfa.accepts(&string));
    }

    #[test]
    fn single_fail() {
        let nfa = NFA::from("a").unwrap();
        let string = String::from("b");
        assert_eq!(false, nfa.accepts(&string));
    }

    #[test]
    fn rando() {
        let nfa = NFA::from("(a|b)*").unwrap();
        let 
            string = String::from("ababa");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn sandwich() {
        let nfa = NFA::from("(.)*a(.)*").unwrap();
        let string = String::from("....a...");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn another_sammy() {
        let nfa = NFA::from("(.*)a(.*)").unwrap();
        let string = String::from("..a....");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn sammy_with_letters() {
        let nfa = NFA::from("a(.*)c").unwrap();
        let string = String::from("a......c");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn null_kleene_letter() {
        let nfa = NFA::from("a(.*)c").unwrap();
        let string = String::from("ac");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn simple_alt() {
        let nfa = NFA::from("a|b").unwrap();
        assert_eq!(nfa.accepts("a"), true);
        assert_eq!(nfa.accepts("b"), true);
        assert_eq!(nfa.accepts("ab"), true);
    }

    #[test]
    fn union_kleene() {
        let nfa = NFA::from("(a|b)*(c|d)*").unwrap();
        let string = String::from("");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn union_kleene_two() {
        let nfa = NFA::from("(a|b)*(c|d)*").unwrap();
        let string = String::from("abad");
        assert_eq!(true, nfa.accepts(&string));
    }


    #[test]
    fn clo_and_any() {
        let nfa = NFA::from("(.*)a(.*)").unwrap();
        assert_eq!(nfa.accepts("a"), true);
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
    fn union_kleene_tres() {
        let nfa = NFA::from("(a|b)*(c|d)*").unwrap();
        let string = String::from("acbc");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn union_kleene_quatro() {
        let nfa = NFA::from("(a|b)*(c|d)*").unwrap();
        let string = String::from("cabd");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn concatenation_char() {
        let nfa = NFA::from("bc").unwrap();
        let string = String::from("abcd");
        assert_eq!(false, nfa.accepts(&string));
    }

    #[test]
    fn any_char_cat() {
        let nfa = NFA::from(".a.").unwrap();
        let string = String::from("asd");
        assert_eq!(false, nfa.accepts(&string));
    }

    #[test]
    fn cat_and_clo() {
        let nfa = NFA::from("s(.)*e").unwrap();
        assert_eq!(nfa.accepts("sunshine"), true);
        assert_eq!(nfa.accepts("sale"), true);
    }

    #[test]
    fn alter_and_cat() {
        let nfa = NFA::from("a(x|y).").unwrap();
        let string = String::from("axy");
        assert_eq!(true, nfa.accepts(&string));
    }

    #[test]
    fn simple_alternations() {
        let nfa = NFA::from("a|b|c|d|e|1|2|3").unwrap();
        let string = String::from("0a");
        assert_eq!(false, nfa.accepts(&string));
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
    fn add(&mut self, state: State) -> StateId {
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
                let state = self.add(Match(Char::Any, None));
                Fragment {
                    start: state,
                    ends: vec![state],
                }
            }
            AST::Char(c) => {
                let state = self.add(Match(Char::Literal(*c), None));
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
        let state = self.add(Split(Some(left.start), Some(right.start))); // create split state with left + right
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
        let state = self.add(Split(Some(kleene_char.start), None)); // creating split state with match state at lhs
        self.join_fragment(&kleene_char, state); // join closure ast and split state
        Fragment {
            start: state,
            ends: vec![state],
        }
    }
}
