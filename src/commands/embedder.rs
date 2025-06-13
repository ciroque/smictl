use crate::cli::ModelsCommand;
use crate::session::Session;

pub fn handle(cmd: ModelsCommand, session: &mut Session) {
    match cmd {
        ModelsCommand::List => println!("Listing models..."),
        ModelsCommand::Select { name } => {
            println!("Selected model: {}", name);
            session.model = Some(name);
        }
    }
}