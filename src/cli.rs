// SPDX-License-Identifier: MIT

use clap::{Parser, Subcommand};
use crate::backend::types::{DistanceMetric, FieldType, SchemaArg, SchemaField, VectorAlgorithm, VectorDataType};

use std::str::FromStr;

#[derive(Parser)]
#[command(name = "smictl", version, about = "Semantic Memory Index Commander")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Backend(BackendCommand),
    #[command(subcommand)]
    Embedder(EmbedderCommand),
    #[command(subcommand)]
    Source(SourceCommand),
}

#[derive(Subcommand)]
pub enum BackendCommand {
    List,
    Select { name: String },
    Index(IndexArgs),
}

#[derive(clap::Args)]
pub struct IndexArgs {
    #[command(subcommand)]
    pub cmd: IndexCommand,
}

#[derive(Subcommand)]
pub enum IndexCommand {
    Create {
        prefix: String,
        model: String,
        version: Option<String>,

        #[clap(long)]
        dim: Option<usize>,

        #[clap(long)]
        algorithm: Option<VectorAlgorithm>, // ← renamed from vector_type

        #[clap(long)]
        distance: Option<DistanceMetric>,

        #[clap(long)]
        dtype: Option<VectorDataType>, // ← new: FLOAT32 or FLOAT64

        #[clap(long, value_parser = SchemaArg::from_str)]
        schema: Option<SchemaArg>,
    },
    List,
    Delete {
        name: String,
    },
}

#[derive(Subcommand)]
pub enum EmbedderCommand {
    List,
    Select { name: String },
}

#[derive(Subcommand)]
pub enum SourceCommand {
    Select { path: String },
    GenerateEmbeddings,
    Store,
}
