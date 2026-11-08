use scraper::{Html, Selector};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub(crate) fn find_all(package: String) -> Vec<(String, String)>{
    let full_path = Path::new("target/doc")
        .join(package)
        .join("all.html");

    let html_content = fs::read_to_string(full_path).unwrap();
    let document = Html::parse_document(&html_content);

    let selector = Selector::parse("ul.all-items li a").unwrap();

    println!("{:?}\n\n", selector);

    let mut elements: Vec<(String, String)> = Vec::new();

    for element in document.select(&selector) {
        let name = element.text().collect::<String>();
        let href = element.value().attr("href").unwrap_or_default().to_string();

        elements.push((name, href));
    }

    elements.sort_by(|a, b| a.0.cmp(&b.0));

    elements
}


pub(crate) fn show_one(package: String, item_href: String) -> Result<(), Box<dyn std::error::Error>> {
    let full_path: PathBuf = Path::new("target/doc")
        .join(package)
        .join(item_href);

    let html_content = match fs::read_to_string(&full_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Couldnt find file at {:?}", full_path);
            return Err(Box::new(std::io::Error::new(ErrorKind::NotFound, "Module not found")));
        }
    };

    let document = Html::parse_document(&html_content);

    let decl_selector = Selector::parse("pre.rust.item-decl")?;
    if let Some(item) = document.select(&decl_selector).next() {
        let name = item.text().collect::<String>();
        println!("Name: {}", name.trim());
    }

    let desc_selector = Selector::parse("section#main-content .docblock")?;
    if let Some(item) = document.select(&desc_selector).next() {
        let desc = item.text().collect::<String>();
        println!("Documentation: ---\n{}", desc);
    }

    let field_selector = Selector::parse(".structfield")?;
    let fields: Vec<String> = document
        .select(&field_selector)
        .map(|er| er.text().collect::<String>())
        .collect();

    if !fields.is_empty() {
        println!("Fields: ---\n{:?}", fields);
        for field in fields {
            println!("Field: {}", field.trim());
        }
        println!()
    }

    let variant_selector = Selector::parse(".variant")?;
    let variants: Vec<String> = document
        .select(&variant_selector)
        .map(|er| er.text().collect::<String>())
        .collect();

    if !variants.is_empty() {
        println!("Variants: ---\n{:?}", variants);
        for variant in variants {
            println!("Variant: {}", variant.trim());
        }
        println!()
    }

    Ok(())
}
