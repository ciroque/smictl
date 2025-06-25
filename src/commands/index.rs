// SPDX-License-Identifier: MIT

use std::sync::Arc;
use crate::backend::Backend;
use crate::cli::IndexCommand;
use crate::session::Session;
use crate::backend::types::{universal_index_name, DistanceMetric, MemoryLocator, SchemaArg, VectorAlgorithm, VectorDataType, IndexParams, SchemaField};

pub async fn handle(cmd: IndexCommand, session: &mut Session) {
    match cmd {
        IndexCommand::Create { prefix, model, version, dim, algorithm, distance, dtype, schema } => {
            handle_index_create(prefix, model, version, dim, algorithm, distance, dtype, schema, session).await;
        }
        IndexCommand::List => {
            handle_index_list(session).await;
        }
        IndexCommand::Delete { name } => {
            handle_index_delete(name, session).await;
        }
    }
}

async fn handle_index_create(
    prefix: String,
    model: String,
    version: Option<String>,
    dim: Option<usize>,
    algorithm: Option<VectorAlgorithm>,
    distance: Option<DistanceMetric>,
    dtype: Option<VectorDataType>,
    mut schema: Option<SchemaArg>,
    session: &mut Session,
) {
    use crate::backend::types::{IndexParams, universal_index_name};

    let memory_locator = MemoryLocator {
        index: prefix,
        model: model.clone(),
        version: version.clone(),
    };

    if let Some(ref mut s) = schema {
        s.apply_vector_params(algorithm, distance, dtype, dim);
    }

    let index_params = IndexParams {
        name: universal_index_name(&memory_locator, 0),
        schema: schema.map(|s| s.0).unwrap_or_default(),
    };

    with_selected_backend(session, |backend| async move {
        match backend.create_index(index_params).await {
            Ok(_) => println!("Index created successfully."),
            Err(e) => eprintln!("Failed to create index: {:?}", e),
        }
    }).await
}

async fn handle_index_delete(name: String, session: &mut Session) {
    with_selected_backend(session, |backend| async move {
        match backend.delete_index(&*name).await {
            Ok(_) => println!("Index '{}' deleted successfully.", name),
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
