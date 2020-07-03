use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=docs");

    let mdbook_result = Command::new("mdbook").args(&["build", "docs"]).status().expect("could not execute mdbook");
    if !mdbook_result.success() {
        panic!("mdbook did not run successfully");
    }

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("docs.zip");

    let zip_result = Command::new("zip").arg("-r").arg(&dest_path).arg(".").current_dir("docs/book").status().expect("could not execute zip");
}

