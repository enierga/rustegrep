use super::NFA;
use super::Char;
use super::State::*;

/**
 * Helper functions for visualizing our NFA
 * Both at the internal representation level and in dot format
 * to generate a graphical representation.
 */

/**
 * Generate a string of the internal structure of the NFA.
 */
pub fn nfa_dump(nfa: &NFA) -> String {
    let mut s = String::new();
    for (id, state) in nfa.states.iter().enumerate() {
        s.push_str(&format!("{:03} | {:?}\n", id, state));
    }
    s
}

/**
 * Generate a DOT structured string.
 */
pub fn nfa_dot(nfa: &NFA) -> String {
    let mut dot = String::from("digraph nfa {\n\tnode [shape = circle];\n");
    for (id, state) in nfa.states.iter().enumerate() {
        dot.push_str(&match state {
            Start(Some(next)) => format!("\tstart [shape=\"none\"]\n\tstart -> {}\n", next),
            Match(c, Some(next)) => format!("\t{} -> {} [label=\"{}\"]\n", id, next, c),
            Split(Some(lhs), Some(rhs)) => format!(
                "\t{0} -> {1} [label=\"ε\"]\n\t{0} -> {2} [label=\"ε\"]\n",
                id, rhs, lhs
            ),
            End => format!("\t{} [shape=\"doublecircle\"]\n", id),
            _ => String::new(),
        });
    }
    dot += "}";
    dot
}

/**
 * Used by the DOT helper function to generate labels for each edge.
 */
impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Char::Literal(c) => write!(f, "{}", c),
            Char::Any => write!(f, "ANY"),
        }
    }
}
