pub mod docs;
pub mod models;

use std::{io, path::Path};

use scraper::{ElementRef, Html, Selector};

use models::{DocBlock, DocItem, ItemKind, MethodItem};


/// Allows the frontend to pass only the path and parse get all info, rather than having to read the
/// file and pass the text.
pub(crate) fn parse_doc_file(path: &Path) -> io::Result<DocItem> {
    let html = std::fs::read_to_string(path)?;

    parse_doc_html(&html)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse rustdoc HTML"))
}


/// extracts the signature of the said thingy from HTML file
fn extract_decl(html: &str) -> Option<String> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("pre.item-decl").unwrap();

    let element = fragment.select(&selector).next()?;
    let text = element.text().collect::<Vec<_>>().join(" ");

    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    Some(cleaned)
}

/// A kinda ugly function, which classifies text from HTML into a better enum type
/// And yeah... it returns a tuple with (kind, name)
fn extract_kind_and_name(html: &str) -> Option<(ItemKind, String)> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse(".main-heading h1").unwrap();

    let element = fragment.select(&selector).next()?;
    let text = element.text().collect::<Vec<_>>().join(" ");
    let text = text.trim();

    let kind = get_kind(text);

    let span_selector = Selector::parse("h1 > span").ok()?;
    let span_elem = element.select(&span_selector).next()?;

    let name = span_elem.text().collect::<Vec<_>>().join(" ");

    Some((kind, name.trim().to_string()))
}

/// This function extracts the Path from breadcrumbs in HTML file
/// NOT to be confused with file path, this is for import path like std::vec::Vec
fn extract_mod_path(html: &str) -> Option<String> {
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
fn extract_docblock(html: &str) -> Vec<DocBlock> {
    let frag = Html::parse_fragment(html);
    let selector = Selector::parse("details.top-doc div.docblock").unwrap();

    let Some(docblock) = frag.select(&selector).next() else {
        return Vec::new();
    };

    parse_docblock_element(docblock)
}


///Parses the docblock (the thing that has description and code examples) into a vector of DocBlock
///enum, helps a lot with rendering stuff.
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


///get the info of methods on structs or enums into vector of MethodItems, which has structured way
///info about name, signature and desctiption.
fn extract_method(html: &str) -> Vec<MethodItem> {
    let frag = Html::parse_fragment(html);
    let method_selector = Selector::parse(
        "#implementations-list details.method-toggle, \
            #trait-implementations-list details.method-toggle, \
            div[id*='deref-methods'] details.method-toggle",
    )
        .unwrap();

    let sig_selector = Selector::parse("h4.code-header").unwrap();
    let name_selector = Selector::parse("h4.code-header a.fn").unwrap();
    let description_selector = Selector::parse(".docblock").unwrap();

    let mut methods = Vec::new();

    for toggle in frag.select(&method_selector) {
        let Some(sig) = toggle.select(&sig_selector).next() else {
            continue;
        };

        let Some(name_elem) = toggle.select(&name_selector).next() else {
            continue;
        };

        let raw_signature = sig.text().collect::<String>().trim().to_string();
        let signature = raw_signature
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let name = name_elem.text().collect::<String>().trim().to_string();

        let description = if let Some(doc_elem) = toggle.select(&description_selector).next() {
            parse_docblock_element(doc_elem)
        } else {
            Vec::new()
        };

        methods.push(MethodItem {
            name,
            signature,
            description,
        });
    }
    methods
}


/// The Ultimate Assembler! Takes everything from all functions and wraps it into final DocItem
/// datastrucuture.
fn parse_doc_html(html: &str) -> Option<DocItem> {
    let (kind, name) = extract_kind_and_name(html)?;
    let decl = extract_decl(html)?;
    let mod_path = extract_mod_path(html)?;
    let description = extract_docblock(html);
    let methods = extract_method(html);

    let full_path = format!("{}::{}", mod_path, name);

    Some(DocItem {
        name,
        path: full_path,
        kind,
        declaration: decl,
        description,
        methods,
    })
}


pub(crate) fn get_kind(text: &str) -> ItemKind {
    let kind = if text.starts_with("struct") {
        ItemKind::Struct
    } else if text.starts_with("module") {
        ItemKind::Module
    } else if text.starts_with("enum") {
        ItemKind::Enum
    } else if text.starts_with("trait") {
        ItemKind::Trait
    } else if text.starts_with("function") || text.starts_with("fn") {
        ItemKind::Function
    } else if text.starts_with("macro") {
        ItemKind::Macro
    } else if text.starts_with("constant") {
        ItemKind::Constant
    } else if text.starts_with("type alias") {
        ItemKind::TypeAlias
    } else {
        ItemKind::Other
    };

    kind
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

    #[test]
    fn test_extract_methods() {
        let html = r##"<div id="implementations-list"><details class="toggle method-toggle" open=""><summary><section id="method.new" class="method"><a class="src rightside" href="../../src/tokio/fs/dir_builder.rs.html#33-35">Source</a><h4 class="code-header">pub fn <a href="#method.new" class="fn">new</a>() -&gt; Self</h4></section></summary><div class="docblock"><p>Creates a new set of options with default mode/security settings for all
platforms and also non-recursive.</p>
<p>This is an async version of <a href="https://doc.rust-lang.org/nightly/std/fs/struct.DirBuilder.html#method.new" title="associated function std::fs::DirBuilder::new"><code>std::fs::DirBuilder::new</code></a>.</p>
<h5 id="examples"><a class="doc-anchor" href="#examples">§</a>Examples</h5>
<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>tokio::fs::DirBuilder;

<span class="kw">let </span>builder = DirBuilder::new();</code></pre><div class="button-holder"><button class="copy-button" title="Copy code to clipboard"></button></div></div></div></details><details class="toggle method-toggle" open=""><summary><section id="method.recursive" class="method"><a class="src rightside" href="../../src/tokio/fs/dir_builder.rs.html#52-55">Source</a><h4 class="code-header">pub fn <a href="#method.recursive" class="fn">recursive</a>(&amp;mut self, recursive: <a class="primitive" href="https://doc.rust-lang.org/nightly/std/primitive.bool.html">bool</a>) -&gt; &amp;mut Self</h4></section></summary><div class="docblock"><p>Indicates whether to create directories recursively (including all parent directories).
Parents that do not exist are created with the same security and permissions settings.</p>
<p>This option defaults to <code>false</code>.</p>
<p>This is an async version of <a href="https://doc.rust-lang.org/nightly/std/fs/struct.DirBuilder.html#method.recursive" title="method std::fs::DirBuilder::recursive"><code>std::fs::DirBuilder::recursive</code></a>.</p>
<h5 id="examples-1"><a class="doc-anchor" href="#examples-1">§</a>Examples</h5>
<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>tokio::fs::DirBuilder;

<span class="kw">let </span><span class="kw-2">mut </span>builder = DirBuilder::new();
builder.recursive(<span class="bool-val">true</span>);</code></pre><div class="button-holder"><button class="copy-button" title="Copy code to clipboard"></button></div></div></div></details></div>"##;

        let methods = extract_method(html);
        assert_eq!(methods.len(), 2);

        assert_eq!(methods[0].name, "new");
        assert_eq!(methods[1].name, "recursive");
        assert_eq!(methods[0].signature, "pub fn new() -> Self".to_string());
        assert_eq!(
            methods[1].signature,
            "pub fn recursive(&mut self, recursive: bool) -> &mut Self".to_string()
        );
        // Sorry I used AI to generate structure for this test :sob: I don't wanna parse HTML to Vec
        // in my brain :hs:
        assert_eq!(
            methods[0].description,
            vec![
                DocBlock::Paragraph("Creates a new set of options with default mode/security settings for all\nplatforms and also non-recursive.".to_string()),
                DocBlock::Paragraph("This is an async version of std::fs::DirBuilder::new.".to_string()),
                DocBlock::Heading("Examples".to_string()),
                DocBlock::Code("use tokio::fs::DirBuilder;\n\nlet builder = DirBuilder::new();".to_string()),
            ]
        );
    }

    #[test]
    fn test_real_html_file() {
        let path = std::path::Path::new("tests/fixtures/dirbuilder.html");

        assert!(path.exists(), "Data file not found");

        let doc = parse_doc_file(path).expect("Failed to parse real data");
        assert_eq!(doc.name, "DirBuilder");
        assert!(!doc.methods.is_empty());
        println!(
            "Successfully parsed real doc with {} methods!",
            doc.methods.len()
        );
    }
}

