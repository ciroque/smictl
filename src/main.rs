mod cli;
mod repl;
mod session;
mod commands;

use cli::Cli;
use repl::run_repl;
use session::Session;
use clap::Parser;
use commands::dispatch;

fn main() {
    let args = Cli::parse();
    if args.command.is_some() {
        let mut session = Session::new();
        dispatch(args, &mut session);
    } else {
        println!("Entering REPL mode (type 'exit' to quit)");
        run_repl();
    }
}