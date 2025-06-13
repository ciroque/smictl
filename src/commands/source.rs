// SPDX-License-Identifier: MIT

use crate::cli::DocumentCommand;
use crate::session::Session;

pub fn handle(cmd: DocumentCommand, session: &mut Session) {
    match cmd {
        DocumentCommand::Select { path } => {
            println!("Selected document: {}", path);
            session.document = Some(path);
        }
        DocumentCommand::GenerateEmbeddings => println!("Generating embeddings..."),
        DocumentCommand::Store => println!("Storing embeddings..."),
    }
}
