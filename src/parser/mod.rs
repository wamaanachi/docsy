use std::{io, path::Path};

use scraper::{ElementRef, Html, Selector};

use crate::models::{DocBlock, DocItem, ItemKind, MethodItem};

/// extracts the signature of the said thingy from HTML file
pub fn extract_decl(html: &str) -> Option<String> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("pre.item-decl").unwrap();

    let element = fragment.select(&selector).next()?;
    let text = element.text().collect::<Vec<_>>().join(" ");

    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    Some(cleaned)
}

/// A kinda ugly function, which classifies text from HTML into a better enum type
/// And yeah... it returns a tuple with (kind, name)
pub fn extract_kind_and_name(html: &str) -> Option<(ItemKind, String)> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse(".main-heading h1").unwrap();

    let element = fragment.select(&selector).next()?;
    let text = element.text().collect::<Vec<_>>().join(" ");
    let text = text.trim();

    let kind = if text.starts_with("Struct") {
        ItemKind::Struct
    } else if text.starts_with("Module") {
        ItemKind::Module
    } else if text.starts_with("Enum") {
        ItemKind::Enum
    } else if text.starts_with("Trait") {
        ItemKind::Trait
    } else if text.starts_with("Function") || text.starts_with("Fn") {
        ItemKind::Function
    } else if text.starts_with("Macro") {
        ItemKind::Macro
    } else if text.starts_with("Constant") {
        ItemKind::Constant
    } else if text.starts_with("Type Alias") {
        ItemKind::TypeAlias
    } else {
        ItemKind::Other
    };

    let span_selector = Selector::parse("h1 > span").ok()?;
    let span_elem = element.select(&span_selector).next()?;

    let name = span_elem.text().collect::<Vec<_>>().join(" ");

    Some((kind, name.trim().to_string()))
}

/// This function extracts the Path from breadcrumbs in HTML file
/// NOT to be confused with file path, this is for import path like std::vec::Vec
pub fn extract_mod_path(html: &str) -> Option<String> {
    let frag = Html::parse_fragment(html);
    let bread_selector = Selector::parse(".rustdoc-breadcrumbs a").unwrap();

    let parts: Vec<String> = frag
        .select(&bread_selector)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if parts.is_empty() {
        None
    } else {
        Some(parts.join("::"))
    }
}

///extracts top-level descriptions
pub fn extract_docblock(html: &str) -> Vec<DocBlock> {
    let frag = Html::parse_fragment(html);
    let selector = Selector::parse("details.top-doc div.docblock").unwrap();

    let Some(docblock) = frag.select(&selector).next() else {
        return Vec::new();
    };

    parse_docblock_element(docblock)
}

fn parse_docblock_element(docblock: ElementRef) -> Vec<DocBlock> {
    let selector = Selector::parse("p, h1, h2, h3, h4, h5, h6, div.example-wrap pre").unwrap();

    docblock
        .select(&selector)
        .filter_map(|e| {
            let text = e.text().collect::<String>().trim().to_string();
            if text.is_empty() {
                return None;
            }

            match e.value().name() {
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    let raw = text.replace("§", "");
                    let clean = raw.trim().to_string();
                    if !clean.is_empty() {
                        Some(DocBlock::Heading(clean))
                    } else {
                        None
                    }
                }
                "p" => Some(DocBlock::Paragraph(text)),
                "pre" => Some(DocBlock::Code(text)),
                _ => None,
            }
        })
        .collect()
}

pub fn extract_method(html: &str) -> Vec<MethodItem> {
    let frag = Html::parse_fragment(html);
    let method_selector = Selector::parse(
    "#implementations-list details.method-toggle, div[id*='deref-methods'] details.method-toggle").unwrap();

    let sig_selector = Selector::parse("h4.code-header").unwrap();
    let name_selector = Selector::parse("h4.code-header a.fn").unwrap();
    let description_selector = Selector::parse("h4.code-header a.fn").unwrap();

    let mut methods = Vec::new();

    for toggle in frag.select(&method_selector) {
        let Some(sig) = toggle.select(&sig_selector).next() else {
            continue;
        };

        let Some(name) = toggle.select(&name_selector).next() else {
            continue;
        };

        let sig_text = sig.text().collect::<Vec<_>>().join(" ").trim().to_string();
        let name_text = name.text().collect::<String>().trim().to_string();

        let description = if let Some(doc_elem) = toggle.select(&description_selector).next() {
            parse_docblock_element(doc_elem)
        } else {
            Vec::new()
        };

        methods.push(MethodItem {
            name: name_text,
            signature: sig_text,
            description,
        });
    }
    methods
}

pub fn parse_doc_html(html: &str) -> Option<DocItem> {
    let (kind, name) = extract_kind_and_name(html)?;
    let decl = extract_decl(html)?;
    let mod_path = extract_mod_path(html)?;
    let description = extract_docblock(html);

    let full_path = format!("{}::{}", mod_path, name);

    Some(DocItem {
        name,
        path: full_path,
        kind,
        declaration: decl,
        description,
        examples: Vec::new(),
    })
}

pub fn parse_doc_file(path: &Path) -> io::Result<DocItem> {
    let html = std::fs::read_to_string(path).unwrap();

    parse_doc_html(&html)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse rustdoc HTML"))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_html() {
        let html = r#"
            <section id="main-content"> 
                <div class="main-heading"> Randos </div>
                <pre class="rust item-decl">
                    <code>
                        pub struct Vec&lt;T, A: Allocator = Global&gt;
                    </code>
                </pre>
            </section>
        "#;

        let data = extract_decl(html);
        assert_eq!(
            data,
            Some("pub struct Vec<T, A: Allocator = Global>".to_string())
        );
    }

    #[test]
    fn test_extract_title_kind() {
        let html = r##"
        <div class="main-heading">
            <div id="sidebar-button"><a href="../../tokio/all.html" title="show sidebar"></a></div>
            <div class="rustdoc-breadcrumbs"><a href="../index.html">tokio</a>::<wbr><a href="index.html">fs</a></div>
            <h1>Struct <span class="struct">DirBuilder</span>&nbsp;<button id="copy-path" title="Copy item path to clipboard">Copy item path</button></h1>
        </div>
    "##;

        let (kind, name) = extract_kind_and_name(html).expect("Failed to extract kind and title");

        assert_eq!(kind, ItemKind::Struct);
        assert_eq!(name, "DirBuilder");
    }

    #[test]
    fn test_extract_breadcrumb() {
        let html = r##"
        <div class="main-heading">
            <div id="sidebar-button"><a href="../../tokio/all.html" title="show sidebar"></a></div>
            <div class="rustdoc-breadcrumbs"><a href="../index.html">tokio</a>::<wbr><a href="index.html">fs</a></div>
            <h1>Struct <span class="struct">DirBuilder</span>&nbsp;<button id="copy-path" title="Copy item path to clipboard">Copy item path</button></h1>
        </div>
        "##;

        let path = extract_mod_path(html);

        assert_eq!(path, Some("tokio::fs".to_string()));
    }

    #[test]
    fn test_extract_top_description_blocks() {
        let html = r#"
    <details class="toggle top-doc" open>
        <div class="docblock">
            <p>A builder for creating directories.</p>
            <div class="example-wrap"><pre class="rust rust-example-rendered"><code>let x = 5;</code></pre></div>
            <p>End of docs.</p>
        </div>
    </details>
    "#;

        let blocks = extract_docblock(html);

        assert_eq!(
            blocks,
            vec![
                DocBlock::Paragraph("A builder for creating directories.".to_string()),
                DocBlock::Code("let x = 5;".to_string()),
                DocBlock::Paragraph("End of docs.".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_doc_html() {
        let html = r#"<div class="main-heading"><div id="sidebar-button"><a href="../../tokio/all.html" title="show sidebar"></a></div><div class="rustdoc-breadcrumbs"><a href="../index.html">tokio</a>::<wbr><a href="index.html">fs</a></div><h1>Struct <span class="struct">DirBuilder</span>&nbsp;<button id="copy-path" title="Copy item path to clipboard">Copy item path</button></h1><span class="sub-heading"><a class="src" href="../../src/tokio/fs/dir_builder.rs.html#11-18">Source</a> </span><rustdoc-toolbar>
        <div id="search-button" tabindex="-1">
            <a href="https://docs.rs/tokio/latest/tokio/fs/struct.DirBuilder.html?search="><span class="label">Search</span></a>
        </div>
        <div class="settings-menu" tabindex="-1">
            <a href="../../settings.html"><span class="label">Settings</span></a>
        </div>
        <div class="help-menu" tabindex="-1">
            <a href="../../help.html"><span class="label">Help</span></a>
        </div>
        <button id="toggle-all-docs" title="Collapse sections (shift-click to also collapse impl blocks)"><span class="label">Summary</span></button></rustdoc-toolbar></div>
        <pre class="rust item-decl"><code>pub struct DirBuilder { <span class="comment">/* private fields */</span> }</code></pre>
        <details class="toggle top-doc" open=""><summary class="hideme"><span>Expand description</span></summary><div class="docblock"><p>A builder for creating directories in various manners.</p>
        <p>This is a specialized version of <a href="https://doc.rust-lang.org/nightly/std/fs/struct.DirBuilder.html" title="struct std::fs::DirBuilder"><code>std::fs::DirBuilder</code></a> for usage on the Tokio runtime.</p>
        </div></details>
        "#;

        let parsed_doc: DocItem = parse_doc_html(html).unwrap();

        assert_eq!(parsed_doc.name, "DirBuilder".to_string());
        assert_eq!(parsed_doc.kind, ItemKind::Struct);
        assert_eq!(parsed_doc.path, "tokio::fs::DirBuilder");
        assert!(!parsed_doc.description.is_empty());
    }
}
