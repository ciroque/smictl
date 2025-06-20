// SPDX-License-Identifier: MIT

use crate::cli::Cli;
use crate::commands::dispatch;
use crate::session::Session;
use rustyline::Editor;
use rustyline::history::DefaultHistory;

use clap::Parser;

pub async fn run_repl(rl: &mut Editor<(), DefaultHistory>) {
    const HISTORY_FILE: &str = "history.txt";
    let mut rl = Editor::<(), DefaultHistory>::new().expect("Failed to initialize line editor");
    rl.load_history(HISTORY_FILE).unwrap_or_default();
    let mut session = Session::new();

    loop {
        match rl.readline("smictl> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(&line);
                let trimmed = line.trim();
                if trimmed == "exit" || trimmed == "quit" {
                    break;
                }
                let tokens: Vec<&str> = trimmed.split_whitespace().collect();
                if tokens.is_empty() {
                    continue;
                }
                let args = std::iter::once("smictl").chain(tokens.iter().copied());
                match Cli::try_parse_from(args) {
                    Ok(parsed) => dispatch(parsed, &mut session).await,
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(_) => break,
        }

    }

    rl.save_history(HISTORY_FILE).unwrap();
}
