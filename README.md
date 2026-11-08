# manr

A terminal documentation viewer and pager for Rust crates.

Opening a web browser just to check a function signature or read a quick doc comment pulls you out of your editor. `manr` parses locally generated `cargo doc` HTML files and presents them directly in your terminal, either as a split-screen explorer or a minimal, keyboard-driven pager.

---

## Features

- **Offline and Local**: Reads directly from your local `target/doc` directory without needing internet access.
- **Structured Extraction**: Extracts type declarations, module paths, top-level docblocks, code examples, and methods.
- **Vim Navigation**: Full keyboard control using standard `less` and Vim conventions.
- **Two Views**:
  - **Split View**: Sidebar index on the left, document reader on the right.
  - **Pager View**: Fullscreen, minimal reading view focused purely on the manual.
- **CLI Utilities**: Search, inspect, and generate documentation directly from the command line.

---

## Getting Started

### Prerequisites

You need Rust and Cargo installed on your system.

### Installation

Clone the repository and build from source:

```bash
git clone https://github.com/wamaanachi/manr.git
cd manr
cargo build --release
```

The resulting binary will be in `target/release/manr`. You can move it into your `$PATH` (e.g. `~/.local/bin/`) to run it from anywhere.

---

## Usage

### Interactive TUI

To launch the interactive documentation browser:

```bash
manr
```

Ensure your project documentation has been generated first by running `cargo doc` or using the built-in generator:

```bash
manr generate
```

### Command Line Interface

You can also query docs directly without entering the interactive interface:

- **Generate docs**: `manr generate`
- **List available items**: `manr list`
- **Search symbols**: `manr search <query>`
- **View specific item**: `manr get struct <Name>`

---

## How It Works

1. **Extraction**: Uses the `scraper` crate to inspect local rustdoc HTML files, extracting declarations (`.item-decl`), module breadcrumbs (`.rustdoc-breadcrumbs`), docblocks (`.docblock`), and methods (`.method-toggle`).
2. **Intermediate Representation**: Deconstructs HTML into an internal AST of headings, text paragraphs, and code blocks rather than dumping unformatted raw text.
3. **Rendering**: Feeds the parsed document model into a custom `ratatui` interface with dynamic scroll calculation and keyboard navigation.

---

## Contributors

Built for Hack Club Third Space by:

- **Chish** - HTML parsing engine, AST extraction, and unit tests
- **Wam** - Ratatui TUI, split-view layout, and pager controls
- **Amaan** - CLI argument handling, search discovery, and cargo automation
