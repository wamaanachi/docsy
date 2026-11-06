use scraper::{Html, Selector};

use crate::models::ItemKind;

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
}
