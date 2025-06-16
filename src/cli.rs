// SPDX-License-Identifier: MIT

use clap::{Parser, Subcommand};

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
