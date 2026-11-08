use scraper::{Html, Selector};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, ExitStatus};
use crate::models::ItemKind;

// --- HTML PARSING STUFF ---
pub(crate) fn find_all(package: String) -> Vec<(String, String)>{
    let full_path = Path::new("target/doc")
        .join(package)
        .join("all.html");

    let html_content = fs::read_to_string(full_path).unwrap();
    let document = Html::parse_document(&html_content);

    let selector = Selector::parse("ul.all-items li a").unwrap();

    let mut elements: Vec<(String, String)> = Vec::new();

    for element in document.select(&selector) {
        let name = element.text().collect::<String>();
        let href = element.value().attr("href").unwrap_or_default().to_string();

        elements.push((name, href));
    }

    elements.sort_by(|a, b| a.0.cmp(&b.0));

    elements
}


// pub(crate) fn show_one(package: String, item_href: String) -> Result<(), Box<dyn std::error::Error>> {
//     let full_path: PathBuf = Path::new("target/doc")
//         .join(package)
//         .join(item_href);
//
//     let html_content = match fs::read_to_string(&full_path) {
//         Ok(content) => content,
//         Err(_) => {
//             eprintln!("Couldnt find file at {:?}", full_path);
//             return Err(Box::new(std::io::Error::new(ErrorKind::NotFound, "Module not found")));
//         }
//     };
//
//     let document = Html::parse_document(&html_content);
//
//     let decl_selector = Selector::parse("pre.rust.item-decl")?;
//     if let Some(item) = document.select(&decl_selector).next() {
//         let name = item.text().collect::<String>();
//         println!("Name: {}", name.trim());
//     }
//
//     let desc_selector = Selector::parse("section#main-content .docblock")?;
//     if let Some(item) = document.select(&desc_selector).next() {
//         let desc = item.text().collect::<String>();
//         println!("Documentation: ---\n{}", desc);
//     }
//
//     let field_selector = Selector::parse(".structfield")?;
//     let fields: Vec<String> = document
//         .select(&field_selector)
//         .map(|er| er.text().collect::<String>())
//         .collect();
//
//     if !fields.is_empty() {
//         println!("Fields: ---\n{:?}", fields);
//         for field in fields {
//             println!("Field: {}", field.trim());
//         }
//         println!()
//     }
//
//     let variant_selector = Selector::parse(".variant")?;
//     let variants: Vec<String> = document
//         .select(&variant_selector)
//         .map(|er| er.text().collect::<String>())
//         .collect();
//
//     if !variants.is_empty() {
//         println!("Variants: ---\n{:?}", variants);
//         for variant in variants {
//             println!("Variant: {}", variant.trim());
//         }
//         println!()
//     }
//
//     Ok(())
// }


// --- SPECIFICS ---
pub(crate) fn show_all_instances(item: ItemKind, package: String) -> Result<(), Box<dyn std::error::Error>> {
    let elements: Vec<(String, String)> = find_all(package);
    let mut specific_elements: Vec<(String, String)> = Vec::new();

    for (name, href) in &elements {
        let kind = crate::parser::get_kind(&href.split('/').last().unwrap());

        if kind == item {
            specific_elements.push((name.to_string(), href.to_string()));
        }
    }

    let count = specific_elements.len();
    println!("{count} Number of {item:?} FOUND!");

    for (name, _href) in specific_elements {
        println!("- {name}")
    }

    Ok(())
}


pub(crate) fn classify(things: Vec<(String, String)>) -> Result<(), Box<dyn std::error::Error>> {
    // pub enum ItemKind {
    //     Module,
    //     Struct,
    //     Enum,
    //     Trait,
    //     Function,
    //     TypeAlias,
    //     Macro,
    //     Constant,
    //     Other,
    // }

    let mut module_things: Vec<(String, String)> = Vec::new();
    let mut struct_things: Vec<(String, String)> = Vec::new();
    let mut enum_things: Vec<(String, String)> = Vec::new();
    let mut trait_things: Vec<(String, String)> = Vec::new();
    let mut function_things: Vec<(String, String)> = Vec::new();
    let mut type_things: Vec<(String, String)> = Vec::new();
    let mut macro_things: Vec<(String, String)> = Vec::new();
    let mut const_things: Vec<(String, String)> = Vec::new();
    let mut other_things: Vec<(String, String)> = Vec::new();

    for (name, href) in things {
        let kind = crate::parser::get_kind(&href.split('/').last().unwrap());

        println!("{href} is a {kind:?}");

        match kind {
            ItemKind::Module => {
                module_things.push((name, href));
            }
            ItemKind::Enum => {
                enum_things.push((name, href));
            }
            ItemKind::Struct => {
                struct_things.push((name, href));
            }
            ItemKind::Trait => {
                trait_things.push((name, href));
            }
            ItemKind::Function => {
                function_things.push((name, href));
            }
            ItemKind::TypeAlias => {
                type_things.push((name, href));
            }
            ItemKind::Macro => {
                macro_things.push((name, href));
            }
            ItemKind::Constant => {
                const_things.push((name, href));
            }
            ItemKind::Other => {
                other_things.push((name, href));
            }
        }
    }

    if module_things.len() > 0 {
        module_things.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{} Modules found:", module_things.len());
        for (name, _href) in module_things {
            println!("- {name}")
        }
        println!()
    }
    if struct_things.len() > 0 {
        struct_things.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{} Structs found:", struct_things.len());
        for (name, _href) in struct_things {
            println!("- {name}")
        }
        println!()
    }
    if enum_things.len() > 0 {
        enum_things.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{} Enums found:", enum_things.len());
        for (name, _href) in enum_things {
            println!("- {name}")
        }
        println!()
    }
    if trait_things.len() > 0 {
        trait_things.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{} Traits found:", trait_things.len());
        for (name, _href) in trait_things {
            println!("- {name}")
        }
        println!()
    }
    if function_things.len() > 0 {
        function_things.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{} Functions found:", function_things.len());
        for (name, _href) in function_things {
            println!("- {name}")
        }
        println!()
    }
    if type_things.len() > 0 {
        type_things.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{} Type Aliases found:", type_things.len());
        for (name, _href) in type_things {
            println!("- {name}")
        }
        println!()
    }
    if macro_things.len() > 0 {
        macro_things.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{} Macros found:", macro_things.len());
        for (name, _href) in macro_things {
            println!("- {name}")
        }
        println!();
    }
    if const_things.len() > 0 {
        const_things.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{} Constant found:", const_things.len());
        for (name, _href) in const_things {
            println!("- {name}")
        }
        println!()
    }
    if other_things.len() > 0 {
        other_things.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{} Other things found:", other_things.len());
        for (name, _href) in other_things {
            println!("- {name}")
        }
        println!()
    }

    Ok(())
}


// --- GENERATE DOCS ---
pub(crate) fn generate_docs(dir: Option<String>) {
    let status: ExitStatus;
    if dir.is_some() {
        status = Command::new("cargo")
            .arg("doc")
            .current_dir(dir.unwrap())
            .status()
            .expect("Something went wrong");
    } else {
        status = Command::new("cargo")
            .arg("doc")
            .status()
            .expect("Something went wrong");
    }
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

