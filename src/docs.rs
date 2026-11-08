use std::path::Path;
use crate::models::{DocItem, DocBlock};

// complicated ahh code
pub fn load(dir: &Path) -> Vec<DocItem> {
    let mut docs = Vec::new();
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir(){
            docs.extend(load(&path));
        }
        else if let Ok(doc) = crate::parser::parse_doc_file(&path){
            docs.push(doc);
        }
    }
    docs
}

// worst code ever?
pub fn text(blocks: &[DocBlock]) -> String {
    blocks
        .iter()
        .map(|block| match block {
            DocBlock::Heading(text) => format!("# {text}"),
            DocBlock::Code(text) => format!("    {text}"),
            DocBlock::Paragraph(text) => text.clone(),
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
