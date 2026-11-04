use scraper::{Html, Selector};

pub fn extract_decl(html: &str) -> Option<String> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("pre.item-decl").ok()?;

    let element = fragment.select(&selector).next()?;
    let text = element.text().collect::<Vec<_>>().join(" ");

    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    Some(cleaned)
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
}
