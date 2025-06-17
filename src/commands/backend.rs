// SPDX-License-Identifier: MIT

use crate::backend::{universal_index_name, Backend, IndexParams, MemoryLocator, RedisBackend};
use crate::cli::{BackendCommand, IndexCommand};
use crate::session::Session;
use std::sync::Arc;

pub async fn handle(cmd: BackendCommand, session: &mut Session) {
    match cmd {

        BackendCommand::Index(index_args) => match index_args.cmd {
            IndexCommand::Create { prefix, model, version } => { 
                handle_index_create(prefix, model, version, session).await
            }
            IndexCommand::List => {
                handle_index_list(session).await;
            }
            IndexCommand::Delete { name } => {
                handle_index_delete(name, session).await;
            }
        },

        BackendCommand::List => {
            handle_backend_list(session).await;
        },

        BackendCommand::Select { name } => {
            handle_backend_select(session).await;
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

async fn handle_index_create(
    prefix: String,
    model: String,
    version: Option<String>,
    session: &mut Session,
) {
    use crate::backend::IndexParams;

    let memory_locator = MemoryLocator {
        index: prefix,
        model: model.clone(),
        version,
    };
    
    with_selected_backend(session, |backend| async move {
        let index_params = IndexParams {
            name: universal_index_name(&memory_locator, 0),
            model,
            dimensions: 1536,
        };
        
        match backend.create_index(index_params).await {
            Ok(_) => println!("Index created successfully."),
            Err(e) => eprintln!("Failed to create index: {:?}", e),
        }
    }).await
}

async fn handle_index_delete(name: String, session: &mut Session) {
    with_selected_backend(session, |backend| async move {
        match backend.delete_index(&*name).await {
            Ok(_) => println!("Index delete successfully."),
            Err(e) => eprintln!("Failed to delete index: {:?}", e),
        }
    }).await
}

async fn handle_index_list(session: &mut Session) {
    with_selected_backend(session, |backend| async move {
        match backend.list_indexes().await {
            Ok(indexes) => {
                if indexes.is_empty() {
                    println!("No indexes found.");
                } else {
                    for idx in indexes {
                        println!("{}", idx);
                    }
                }
            }
            Err(e) => eprintln!("Error listing indexes: {:?}", e),
        }
    }).await;
}

async fn with_selected_backend<F, Fut>(session: &Session, f: F)
where
    F: FnOnce(Arc<dyn Backend + Send + Sync>) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    if let Some(backend) = &session.backend {
        f(Arc::clone(backend)).await;
    } else {
        eprintln!("No backend selected. Use `backend select <name>`.");
    }
}

