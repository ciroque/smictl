# smictl

**Semantic Memory Index Commander** – a CLI and REPL tool for managing semantic memory systems.

`smictl` allows developers and researchers to interactively configure, test, and populate vector-based semantic memory backends using embedding models and raw text sources.

---

## 🚀 Features

* REPL and direct CLI modes
* Pluggable **backends** (e.g., Redis, Pinecone, Weaviate)
* Selectable **embedding models** (OpenAI, Hugging Face, etc.)
* File-based **source** ingestion for embedding
* Clean, idiomatic Rust architecture

---

## 🧠 Concepts

| Term       | Meaning                                                          |
| ---------- | ---------------------------------------------------------------- |
| `backend`  | Semantic memory store (e.g., Redis, Pinecone, Weaviate)          |
| `embedder` | Embedding model + configuration (e.g., `text-embedding-3-small`) |
| `source`   | Raw data to be embedded (e.g., text file, document)              |

---

## 🛠 Usage

### CLI Mode

```bash
smictl backend list
smictl backend select redis

smictl embedder list
smictl embedder select openai/ada

smictl source select ./docs/example.md
smictl source generate-embeddings
smictl source store
```

### REPL Mode

```bash
smictl
```

Once inside:

```text
smictl> backend list
smictl> embedder select openai/ada
smictl> source select ./notes.txt
smictl> source generate-embeddings
smictl> source store
```

---

## 🧱 Project Structure

```
src/
├── main.rs         # CLI entrypoint
├── cli.rs          # clap command parser
├── repl.rs         # REPL input loop
├── session.rs      # Runtime session state
└── commands/
    ├── mod.rs
    ├── backend.rs
    ├── embedder.rs
    └── source.rs
```

---

## 🧪 Coming Soon

* Integration with Redis, Pinecone, Weaviate
* Model management via OpenAI / Hugging Face APIs
* Tokenizer-aware chunking
* Batch embedding + dry-run modes

---

## 🔧 Development

Requires:

* Rust 1.70+ (recommended)
* A working backend (Redis etc.) for full functionality

```bash
cargo build
cargo run
```

---

## 📄 License

MIT

---

Made with frustration and clarity.
