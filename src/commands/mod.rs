pub mod backend;
pub mod embedder;
pub mod source;

use crate::cli::{Cli, Command};
use crate::session::Session;

pub fn dispatch(cli: Cli, session: &mut Session) {
    match cli.command {
        Some(Command::Backend(cmd)) => backend::handle(cmd, session),
        Some(Command::Embedder(cmd)) => embedder::handle(cmd, session),
        Some(Command::Source(cmd)) => source::handle(cmd, session),
        None => (),
    }
}