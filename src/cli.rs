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
    Backend(ProvidersCommand),
    #[command(subcommand)]
    Embedder(ModelsCommand),
    #[command(subcommand)]
    Source(DocumentCommand),
}

#[derive(Subcommand)]
pub enum ProvidersCommand {
    List,
    Select { name: String },
}

#[derive(Subcommand)]
pub enum ModelsCommand {
    List,
    Select { name: String },
}

#[derive(Subcommand)]
pub enum DocumentCommand {
    Select { path: String },
    GenerateEmbeddings,
    Store,
}
