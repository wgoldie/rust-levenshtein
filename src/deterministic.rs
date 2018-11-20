use std::collections::HashMap;
use super::nondeterministic::Automaton as NondeterministicAutomaton;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct StepState {
    index: usize,
    errors: u32,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum State {
    Match,
    Pending(StepState),
    Fail,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Input {
    Exact(char),
    Glob,
    Empty,
}

pub struct Automaton {
    transition_map: HashMap<StepState, HashMap<Input, State>>,
    max_errors: u32,
}

impl<'a> Automaton {
    fn explore_states(
        &'a mut self,
        pattern: &str,
        pattern_len: usize,
        state: StepState
        ) {

        let StepState { errors, index } = state;
       
        assert!(errors <= self.max_errors);
        assert!(pattern_len > index);
       
        let pattern_advance_preempting_state = if errors + 1 == self.max_errors {
            Some(State::Fail)
        } else if index + 1 == pattern_len {
            Some(State::Match)
        } else {
            None
        };

        let match_entry = (
            Input::Exact(pattern.chars().nth(index).unwrap()),
            pattern_advance_preempting_state.unwrap_or(
                State::Pending(StepState { index: index + 1, errors })
            ),
        );

        let transition_entries = if errors == self.max_errors {
            vec![
                // Match
                match_entry,
                // Deletion
                (
                    Input::Empty,
                    State::Pending(StepState { index, errors: errors + 1 })
                ),
                // Insertion
                (
                    Input::Glob,
                    pattern_advance_preempting_state.unwrap_or(
                        State::Pending(StepState { index: index + 1, errors: errors + 1})
                    ),
                ),
                // Substitution
                (
                    Input::Glob,
                    pattern_advance_preempting_state.unwrap_or(
                        State::Pending(StepState { index: index + 1, errors: errors + 1})
                    ),
                ),
            ]
        } else {
            vec![match_entry]
        };

        let mut transition_table = HashMap::new(); 
        let mut states = vec![];

        for (input, new_state) in transition_entries {
            transition_table.insert(input, new_state.clone());
            states.push(new_state);
        }

        self.transition_map.insert(state.clone(), transition_table);
       
        for new_state in states {
            if let State::Pending(step_state) = new_state {
                self.explore_states(pattern, pattern_len, step_state)
            }
        }
    }

    fn match_helper(&self, target: &str, target_len: usize, state: StepState, index: usize) -> bool {
        let transitions = match self.transition_map.get(&state) {
            Some(map) => map,
            None => { return false },
        };

        assert!(index < target_len);

        let pattern_char = match target.chars().nth(index) {
            Some(pc) => pc,
            None => { return false },
        };

        transitions.iter().map(|(input, new_state)| {
            let step = match new_state {
                State::Match => {
                    return ((target_len - index - 1) as u32) <= (self.max_errors - state.errors);
                },
                State::Fail => None,
                State::Pending(step) => Some(step)
            };

            let advance = match input {
                Input::Exact(c) =>
                    if *c == pattern_char 
                    { Some(1) } else { None },
                Input::Glob => Some(1),
                Input::Empty => Some(0),
            };

            if let (Some(step), Some(advance)) = (step, advance) {
                self.match_helper(target, target_len, step.clone(), index + advance)
            } else { false }
        }).all(|x| x)
    }

    pub fn is_match(&self, target: &str) -> bool { 
        self.match_helper(
            target,
            target.chars().count(),
            StepState { index: 0, errors: 0 },
            0
        )
    }

    pub fn new(source_automaton: &NondeterministicAutomaton) -> Automaton {
        let NondeterministicAutomaton { pattern, max_errors, .. } = source_automaton;
        let transition_map = HashMap::new();
        let start = StepState { index: 0, errors: 0 };
        let mut aut = Automaton {
            transition_map,
            max_errors: *max_errors,
        };

        let pattern = *pattern;
        aut.explore_states(pattern, pattern.chars().count(), start); 
        aut
    }
}
