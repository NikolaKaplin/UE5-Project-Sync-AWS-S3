use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::{ Hinter};
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::borrow::Cow::{self, Borrowed, Owned};





pub struct MyHelper {
    commands: Vec<String>,
}

impl MyHelper {
    pub(crate) fn new(commands: Vec<String>) -> Self {
        MyHelper {
            commands: commands.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let mut completions = Vec::new();

        let word = line.split_whitespace().last().unwrap_or("");

        for cmd in &self.commands {
            if cmd.starts_with(word) {
                completions.push(Pair {
                    display: cmd.to_string(),
                    replacement: cmd.to_string(),
                });
            }
        }

        Ok((pos - word.len(), completions))
    }
}

impl Hinter for MyHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Owned(format!("\x1b[1;32m{prompt}\x1b[0m"))
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[1;35m{hint}\x1b[0m"))
    }
}

impl Validator for MyHelper {}

impl Helper for MyHelper {}