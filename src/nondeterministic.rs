#[derive(Clone, Copy, Debug)]
struct State {
    index: usize,
    errors: u32,
}

enum Output {
    Match,
    Step(u32),
    Fail,
}

struct Step(State, Output);

pub struct Automaton<'a> {
    pub pattern: &'a str,
    pub max_errors: u32,
    len: usize,
}

impl<'a> Automaton<'a> {
    pub fn new(pattern: &'a str, max_errors: u32) -> Automaton<'a> {
        Automaton { pattern, max_errors, len: pattern.chars().count() }
    }

    fn step(&self, state: State, c: char) -> Box<[Step]> {
        let State { index, errors } = state;
        assert!(index <= self.len);
        if index == self.len {
            return Box::new([Step(state, Output::Match)]);
        }

        let pattern_char = self.pattern.chars().nth(index as usize).unwrap();
        let sub_or_match_cost = if pattern_char == c { 0 } else { 1 }; 
        

        let both_advance_step = Step(
            State { index: index + 1, errors: errors + sub_or_match_cost },
            Output::Step(1));

        assert!(errors <= self.max_errors);
        if errors == self.max_errors {
            return if sub_or_match_cost == 1 {
                Box::new([Step(state, Output::Fail)])
            } else {
                Box::new([both_advance_step])
            }
        };

        Box::new([
            Step(
                State { index: index + 1, errors: errors + 1 },
                Output::Step(0)),
            Step(
                State { index, errors: errors + 1 },
                Output::Step(1)),
            both_advance_step,
        ])
    }

    pub fn is_match(&self, target: &str) -> bool {
        self.match_helper(
            State { index: 0, errors: 0 },
            0,
            target,
            target.chars().count(),
        )
    }

    fn match_helper(
        &self,
        state: State,
        index: usize,
        target: &str,
        target_len: usize) -> bool {
        if index >= target_len {
            return false;
        }

        for step in self.step(state, target.chars().nth(index).unwrap()).iter() {
            let Step(state, output) = step;
            let ret = match output {
                Output::Match => {
                    // The automaton is not aware of the target string index state
                    // So we need to check if we have enough edits left to
                    // delete any remaining text
                    return (target_len - index - 1)
                        < ((self.max_errors - state.errors) as usize)
                },
                Output::Fail => false,
                Output::Step(step) => self.match_helper(
                    *state,
                    index + (*step as usize),
                    target,
                    target_len
                )
            };

            if ret { return true };
        }

        false 
    }
}
