// SPDX-License-Identifier: MIT

use crate::cli::BackendCommand;
use crate::session::Session;

pub fn handle(cmd: BackendCommand, session: &mut Session) {
    match cmd {
        BackendCommand::List => println!("Listing providers..."),
        BackendCommand::Select { name } => {
            println!("Selected provider: {}", name);
            session.provider = Some(name);
        }
    }
}