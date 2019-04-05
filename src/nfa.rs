pub mod helpers;

// Starter code for PS06 - thegrep
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
        let mut clist = vec![self.start];   // initialize with start StateId
        let mut input_str = input.chars();
        let mut input_length = input.chars().count();

        for i in 0..input_length {  // iterates through each character and tests against states
            if let Some(c) = input_str.next() {
                self.accept_helper(c, &mut clist);
            }
        }

        if clist.len() > 0 {    // checks empty string against the states
            self.empty_str(&mut clist);
        }

        let mut result = false;
        for state_id in clist { // checking for end state in the set of current lists
            match &self.states[state_id] {
                State::End => result = true,
                _ => continue,
            }
        }
        result
    }

    fn empty_str(&self, c_states: &mut Vec<StateId>) {
        let mut i = 0;
        while i < 2 {   // only want to loop 2x: to move out of start and to push arms of potential split state
            match &self.states[c_states[i]] {   // only valid states for empty are start and split
                State::Start(Some(first_state_id)) => {
                    c_states.push(*first_state_id); // push first state
                    c_states.swap_remove(i);
                },
                State::Split(Some(left_state_id), Some(right_state_id)) => {    // if closure, empty string can be accepted
                    c_states.push(*left_state_id);
                    c_states.swap_remove(i);
                    c_states.push(*right_state_id);
                    i += 1;
                },
                _ => break, // otherwise break loop and empty string not accepted
            }
        }

    }

    fn accept_helper(&self, input_char: char, c_states: &mut Vec<StateId>) {
        let mut total_current_states = c_states.len();
        let mut i = 0;
        loop {
            if i >= total_current_states {  // checking each state in the list of current states
                break;
            }
            match &self.states[c_states[i]] {
                State::Start(Some(first_state_id)) => {
                    c_states.push(*first_state_id); // push the first state
                    c_states.swap_remove(i);        // replace the start state with the first state
                },
                State::Match(char_enum, Some(next_state_id)) => {
                    match char_enum {
                        Char::Literal(c) => {
                            if input_char == *c {
                                c_states.push(*next_state_id);
                                c_states.swap_remove(i);
                            }
                        },
                        Char::Any => {
                            c_states.push(*next_state_id);
                            c_states.swap_remove(i);
                        },
                    }
                    i += 1; // if match state is ecountered, move to next state
                },
                State::Split(Some(left_state_id), Some(right_state_id)) => {
                    c_states.push(*left_state_id);  // push left state
                    c_states.swap_remove(i);        // replace split state with left state
                    c_states.push(*right_state_id); // push right state (doesnt replace anything)
                    total_current_states += 1;      // increase # total states by 1

                },
                State::End => {
                    c_states.remove(i); // to be accepted, last char must be at end state
                    i += 1;
                },
                _ => break,
            }
        }
    }
}

/** Tests accepts method */
#[cfg(test)]
mod nfa_accepts {
    use super::*;

    #[test]
    fn any_char() {
        let nfa = NFA::from(".").unwrap();
        assert_eq!(nfa.accepts("a"), true);
        assert_eq!(nfa.accepts("9"), true);
        assert_eq!(nfa.accepts(""), false);
    }

    #[test]
    fn simple_cat() {
        let nfa = NFA::from("ab").unwrap();
        assert_eq!(nfa.accepts("a"), false);
        assert_eq!(nfa.accepts("ab"), true);
    }

    #[test]
    fn simple_alt() {
        let nfa = NFA::from("a|b").unwrap();
        assert_eq!(nfa.accepts("a"), true);
        assert_eq!(nfa.accepts("b"), true);
    }

    #[test]
    fn simple_closure() {
        let nfa = NFA::from("a*").unwrap();
        assert_eq!(nfa.accepts(""), true);
        assert_eq!(nfa.accepts("a"), true);
        assert_eq!(nfa.accepts("aaaaaaa"), true);
    }

    #[test]
    fn cat_and_any() {
        let nfa = NFA::from("a.c").unwrap();
        assert_eq!(nfa.accepts("abc"), true);
        assert_eq!(nfa.accepts("a9c"), true);
    }

    #[test]
    fn five_letter_starts_with_a() {
        let nfa = NFA::from("a....").unwrap();
        assert_eq!(nfa.accepts("apple"), true);
        assert_eq!(nfa.accepts("apples"), false);
        assert_eq!(nfa.accepts("ankle"), true);
        assert_eq!(nfa.accepts("butts"), false);
    }

    #[test]
    fn cat_and_closure() {
        let nfa = NFA::from("s(.)*e").unwrap();
        assert_eq!(nfa.accepts("sunshine"), true);
        assert_eq!(nfa.accepts("sale"), true);
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
