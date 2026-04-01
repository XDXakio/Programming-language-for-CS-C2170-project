pub mod alpha;
pub mod ast;
pub mod display;
pub mod macros;
pub mod module;
pub mod parser;
pub mod step;
pub mod term;
pub mod types;

use nom::combinator::eof;
pub use term::Term;

use nom::Parser;
use nom::branch::alt;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};
use std::borrow::Cow;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use crate::{
    module::Module,
    parser::{parse_ast, parse_decl},
};

use crate::types::{type_of, Context};

struct LambdaPrompt;

impl Prompt for LambdaPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        Cow::Borrowed("λ> ")
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _prompt_mode: PromptEditMode) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        Cow::Borrowed("... ")
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: PromptHistorySearch,
    ) -> Cow<'_, str> {
        Cow::Borrowed("history> ")
    }
}

pub fn main() {
    let mut module = Module::new_with_prelude();
    let mut line_editor = Reedline::create();
    let prompt = LambdaPrompt;
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupt_flag = Arc::clone(&interrupted);

    ctrlc::set_handler(move || {
        interrupt_flag.store(true, Ordering::SeqCst);
    })
    .expect("Failed to set Ctrl-C handler");

    loop {
        let line = match line_editor.read_line(&prompt) {
            Ok(Signal::Success(line)) => line,
            Ok(Signal::CtrlD) => break,
            Ok(Signal::CtrlC) => {
                println!("Use Ctrl-D to exit");
                continue;
            }
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        };

        let mut line = line.trim();
        if line.is_empty() {
            continue;
        }

        match line {
            ":quit" | ":exit" => break,
            ":env" => {
                for (name, ast) in module.iter() {
                    println!("{name} = {}", ast.clone().desugar(&module));
                }
                continue;
            }
            ":help" => {
                println!("Commands:");
                println!("  :quit | :exit   Exit the REPL. Ctrl-D also exits.");
                println!("  :help           Show this help");
                println!("  :env            Print declared constants");
                println!("");
                println!(
                    "Trace mode: start your prompt with # to enable reduction tracing. For example"
                );
                println!("  # (w => w w) (w => w w)");
                println!("");
                println!(
                    "Ctrl-C handling: If evaluation gets stuck, you can press Ctrl-C to stop it."
                );
                continue;
            }
            _ => {}
        }

        let trace = if let Some(l) = line.strip_prefix("#") {
            line = l.trim();
            true
        } else {
            false
        };

        match alt((
            (|i| parse_decl(&module, i), eof).map(|(decl, _)| decl),
            ((|i| parse_ast(&module, i)), eof).map(|(ast, _)| ("this".to_string(), ast)),
        ))
        .parse(line)
        {
            Ok((_, (name, ast))) => {
                interrupted.store(false, Ordering::SeqCst);
                let mut term = ast.clone().desugar(&module);

                let mut ctx = Context::new();
                match type_of(&term, &mut ctx) {
                    Ok(ty) => {
                        println!("Type: {:?}", ty);
                    }
                    Err(e) => {
                        println!("Type error: {:?}", e);
                        continue;
                    }
                }

                let mut counter = 0;

                if trace {
                    loop {
                        if interrupted.load(Ordering::SeqCst) {
                            break;
                        }
                        let Some(next) = term.step() else {
                            break;
                        };
                        println!("--> {next}");
                        term = next;
                        std::thread::sleep(Duration::from_millis(10));
                    }
                } else {
                    while !interrupted.load(Ordering::SeqCst) {
                        match term.step() {
                            Some(next) => {
                                term = next;
                                counter += 1;
                                if counter == 1000 {
                                    println!("(Still evaluating... Press Ctrl-C to stop)")
                                }
                            }
                            None => break,
                        }
                    }
                }

                println!("{term}");
                if interrupted.load(Ordering::SeqCst) {
                    println!("(Interrupted. Saved '{name}' to globals, see :env)");
                }
                module.insert(name, ast);
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}
