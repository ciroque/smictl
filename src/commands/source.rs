// SPDX-License-Identifier: MIT

use crate::cli::SourceCommand;
use crate::session::Session;

pub fn handle(cmd: SourceCommand, session: &mut Session) {
    match cmd {
        SourceCommand::Select { path } => {
            println!("Selected document: {}", path);
            session.document = Some(path);
        }
        SourceCommand::GenerateEmbeddings => println!("Generating embeddings..."),
        SourceCommand::Store => println!("Storing embeddings..."),
    }
}
