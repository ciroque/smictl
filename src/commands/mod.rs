// SPDX-License-Identifier: MIT

pub mod backend;
pub mod embedder;
pub mod index;
pub mod source;

use crate::cli::{Cli, Command};
use crate::session::Session;

pub async fn dispatch(cli: Cli, session: &mut Session) {
    match cli.command {
        Some(Command::Backend(cmd)) => backend::handle(cmd, session).await,
        Some(Command::Embedder(cmd)) => embedder::handle(cmd, session),
        Some(Command::Source(cmd)) => source::handle(cmd, session),
        Some(Command::Index(cmd)) => index::handle(cmd, session).await,
        None => (),
    }
}