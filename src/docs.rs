use std::path::Path;

use crate::models::DocItem;

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
