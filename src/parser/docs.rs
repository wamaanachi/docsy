use crate::parser::models::{DocBlock, DocItem};
use crate::parser::parse_doc_file;

use std::fs;
use std::path::Path;

/// Formats a list of DocBlocks into clean readable text for the TUI
pub fn text(blocks: &[DocBlock]) -> String {
    let mut out = String::new();
    for block in blocks {
        match block {
            DocBlock::Heading(h) => {
                out.push_str(&format!("\n# {}\n\n", h));
            }
            DocBlock::Paragraph(p) => {
                out.push_str(&format!("{}\n\n", p));
            }
            DocBlock::Code(c) => {
                out.push_str(&format!("    {}\n\n", c.replace('\n', "\n    ")));
            }
        }
    }
    out.trim().to_string()
}

/// Recursively collects all .html files in a directory
fn collect_html_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    // If we already have 20 items, stop crawling immediately!
    if files.len() >= 20 {
        return;
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if files.len() >= 20 {
                return;
            }

            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if dir_name != "static.files" && dir_name != "search.index" && dir_name != "src" {
                    collect_html_files(&path, files);
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("html") {
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                if stem.starts_with("struct.")
                    || stem.starts_with("fn.")
                    || stem.starts_with("enum.")
                    || stem.starts_with("trait.")
                {
                    files.push(path);
                }
            }
        }
    }
}

/// Loads and parses all documentation items from a target doc directory.
pub fn load(path: &Path) -> Vec<DocItem> {
    let mut files = Vec::new();
    collect_html_files(path, &mut files);

    // FIX: Cap to 50 items so the TUI opens instantly (<0.3s) instead of freezing!
    files.truncate(50);

    let mut docs = Vec::new();
    for file in files {
        if let Ok(doc) = parse_doc_file(&file) {
            docs.push(doc);
        }
    }

    docs
}
