use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocBlock {
    Heading(String),
    Paragraph(String),
    Code(String),
}

/// This enum holds type of a symbol
/// Wam can use this to color code different symbols
#[derive(Clone, Copy, Serialize, Deserialize, ValueEnum, PartialEq, Eq, Hash, Debug)]
pub enum ItemKind {
    Module,
    Struct,
    Enum,
    Trait,
    Function,
    TypeAlias,
    Macro,
    Constant,
    Other,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct MethodItem {
    pub name: String,
    pub signature: String,
    pub description: Vec<DocBlock>,
}

/// This is a lightweight search return for Search List and sidebar
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SearchResult {
    pub name: String,
    pub path: String, //Don't confuse with file path, this path is for import paths like
    //"std::vec::Vec"
    pub kind: ItemKind,
    pub file_path: PathBuf,
    //Maybe a optional 1-line summary, feel free to change
    pub summary: Option<String>,
}

//The final fully parsed documentation to be rendered.
// Produced by Parser, consumed by TUI/Pager thingy
#[derive(Debug)]
pub struct DocItem {
    pub name: String,
    pub path: String,
    pub kind: ItemKind,
    //The code declaration signature like pub struct Vec<T>
    pub declaration: String,
    //The real stuff, it also includes examples now
    pub description: Vec<DocBlock>,
    pub methods: Vec<MethodItem>,
}
