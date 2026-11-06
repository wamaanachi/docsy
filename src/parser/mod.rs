use scraper::{Html, Selector};

use crate::models::{DocItem, ItemKind};

pub fn extract_decl(html: &str) -> Option<String> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("pre.item-decl").ok()?;

    let element = fragment.select(&selector).next()?;
    let text = element.text().collect::<Vec<_>>().join(" ");

    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    Some(cleaned)
}

pub fn extract_kind_and_name(html: &str) -> Option<(ItemKind, String)> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse(".main-heading h1").ok()?;

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

pub fn extract_mod_path(html: &str) -> Option<String> {
    let frag = Html::parse_fragment(html);
    let bread_selector = Selector::parse(".rustdoc-breadcrumbs a").ok()?;

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

pub fn extract_description(html: &str) -> Option<String> {
    let frag = Html::parse_fragment(html);
    let selector = Selector::parse("details.top-doc div.docblock > p").ok()?;

    let text = frag
        .select(&selector)
        .map(|p| p.text().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n\n");

    Some(text.trim().to_string())
}

pub fn parse_doc_html(html: &str) -> Option<DocItem> {
    let (kind, name) = extract_kind_and_name(html)?;
    let decl = extract_decl(html)?;
    let mod_path = extract_mod_path(html)?;
    let description = extract_description(html)?;

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
    fn test_extract_top_description() {
        let html = r#"
        <details class="toggle top-doc" open>
            <div class="docblock">
                <p>A builder for creating directories in various manners.</p>
                <p>This is a specialized version of std::fs::DirBuilder.</p>
            </div>
        </details>
        "#;

        let desc = extract_description(html);

        let expected = "A builder for creating directories in various manners.\n\nThis is a specialized version of std::fs::DirBuilder.".to_string();
        assert_eq!(desc, Some(expected));
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
        let expect_desc = "A builder for creating directories in various manners.\n\nThis is a specialized version of std::fs::DirBuilder for usage on the Tokio runtime.";

        assert_eq!(parsed_doc.name, "DirBuilder".to_string());
        assert_eq!(parsed_doc.kind, ItemKind::Struct);
        assert_eq!(parsed_doc.path, "tokio::fs::DirBuilder");
        assert_eq!(parsed_doc.description, expect_desc);
    }
}
