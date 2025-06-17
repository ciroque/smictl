mod cli;
mod repl;
mod session;
mod commands;
mod backend;

use cli::Cli;
use repl::run_repl;
use session::Session;
use clap::Parser;
use commands::dispatch;
use rustyline::{Editor, history::DefaultHistory};

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if args.command.is_some() {
        let mut session = Session::new();
        dispatch(args, &mut session).await;
    } else {
        println!("Entering REPL mode (type 'exit' to quit)");
        let mut rl = Editor::<(), DefaultHistory>::new().expect("Failed to initialize line editor");
        let _ = rl.load_history("history.txt");

        run_repl(&mut rl).await;

        let _ = rl.save_history("history.txt");
    }
}
