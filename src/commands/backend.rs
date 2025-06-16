// SPDX-License-Identifier: MIT

use crate::cli::{BackendCommand, IndexCommand};
use crate::session::Session;

pub fn handle(cmd: BackendCommand, session: &mut Session) {
    match cmd {

        BackendCommand::Index(index_args) => match index_args.cmd {
            IndexCommand::Create { prefix, model, version } => { /* ... */ }
            IndexCommand::List => { /* ... */ }
            IndexCommand::Delete { name } => { /* ... */ }
        },
        
        BackendCommand::List => println!("Listing providers..."),
        BackendCommand::Select { name } => {
            println!("Selected provider: {}", name);
            session.provider = Some(name);
        }
    }
}