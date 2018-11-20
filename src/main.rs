extern crate levenshtein;

use levenshtein::nondeterministic::{Automaton as NondeterministicAutomaton};
use levenshtein::deterministic::{Automaton as DeterministicAutomaton};

fn main() {
    let pattern = "Hello";
    let fuzziness = 2;
    println!("\nBuilding nondeterministic automaton with pattern \"{}\" and fuzziness {}...", pattern, fuzziness);
    let nfa = NondeterministicAutomaton::new(pattern, 2);
    println!("Done.");
    println!("\nBuilding deterministic automaton from previous automaton...");
    let dfa = DeterministicAutomaton::new(&nfa);
    println!("Done.");


    let check_string = |string: &str| {
        println!("Checking string \"{}\"", string);
        println!("Testing string with NFA: {}", nfa.is_match(string));
        println!("Testing string with DFA: {}\n", dfa.is_match(string));
    };

    check_string("Hello");
    check_string("Hello,");
    check_string("Hello,,");
    check_string("Hello       ,");
}
