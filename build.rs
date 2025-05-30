use mdbook_gen::generate_router_build_script;
use std::{env::current_dir, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=docs/");
    make_docs("blog");
}

fn make_docs(version: &str) {
    let mdbook_dir = PathBuf::from("docs").join(version);
    let out_dir = current_dir().unwrap().join("src/blog");
    let mut out = generate_router_build_script(mdbook_dir);
    out.push('\n');
    out.push_str("use dioxus::prelude::*;\n");
    out.push_str("use crate::components::blog::code::CodeBlock;\n");
    let version_flattened = version.replace(".", "");
    let filename = format!("router_{version_flattened}.rs");

    std::fs::write(out_dir.join(filename), out).unwrap();
}
