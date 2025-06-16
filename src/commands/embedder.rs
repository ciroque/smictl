// SPDX-License-Identifier: MIT

use crate::cli::EmbedderCommand;
use crate::session::Session;

pub fn handle(cmd: EmbedderCommand, session: &mut Session) {
    match cmd {
        EmbedderCommand::List => println!("Listing models..."),
        EmbedderCommand::Select { name } => {
            println!("Selected model: {}", name);
            session.model = Some(name);
        }
    }
}
 