// SPDX-License-Identifier: MIT

use crate::cli::{BackendCommand, IndexCommand};
use crate::session::Session;
use std::sync::Arc;
use crate::backend::{Backend, RedisBackend};
use crate::backend::types::{universal_index_name, DistanceMetric, MemoryLocator, SchemaArg, VectorAlgorithm, VectorDataType};

pub async fn handle(cmd: BackendCommand, session: &mut Session) {
    match cmd {
        BackendCommand::List => {
            handle_backend_list(session).await;
        },

        BackendCommand::Select { name } => {
            handle_backend_select(session).await;
        },
        
        BackendCommand::Info => {
            handle_backend_info(session).await;
        }
    }
}

async fn handle_backend_list(session: &mut Session) {
    println!("You can have any Backend you want, so long as it's redis");
}

async fn handle_backend_select(session: &mut Session) {
    match RedisBackend::new().await {
        Ok(backend) => {
            session.backend = Some(Arc::new(backend));
            println!("Backend created.")
        }
        Err(e) => {
            eprintln!("Failed to create Redis backend: {:?}", e);
        }
    }
}

async fn handle_backend_info(session: &mut Session) {
    match &session.backend {
        Some(backend) => {
            println!("Current backend information:");
            println!("  Type: {}", backend.name());
            println!("  Connection: {}", backend.get_connection_info());
            println!("  Status: Connected");
            // You can add more specific information about the backend if needed
        },
        None => {
            println!("No backend is currently selected.");
            println!("Use 'smictl backend select' to select a backend.");
        }
    }
}
