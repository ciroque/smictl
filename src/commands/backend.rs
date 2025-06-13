use crate::cli::ProvidersCommand;
use crate::session::Session;

pub fn handle(cmd: ProvidersCommand, session: &mut Session) {
    match cmd {
        ProvidersCommand::List => println!("Listing providers..."),
        ProvidersCommand::Select { name } => {
            println!("Selected provider: {}", name);
            session.provider = Some(name);
        }
    }
}