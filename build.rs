use std::process::Command;
use walkdir::WalkDir;
use zip::{ZipWriter, write::FileOptions};

fn main() {
    println!("cargo:rerun-if-changed=docs");

    let mdbook_result = Command::new("mdbook").args(&["build", "docs"]).status().expect("could not execute mdbook");
    if !mdbook_result.success() {
        panic!("mdbook did not run successfully");
    }

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("docs.zip");

    let files = WalkDir::new("docs/book").into_iter().collect::<Result<Vec<_>, _>>();
    let mut archive = ZipWriter::new(std::fs::File::create(dest_path).unwrap());
    for file in files.unwrap().into_iter().filter(|e| e.file_type().is_file()) {
        archive.start_file_from_path(file.path().strip_prefix("docs/book").unwrap_or(file.path()), FileOptions::default().compression_method(zip::CompressionMethod::Deflated)).unwrap();
        
        std::io::copy(&mut std::fs::File::open(file.path()).unwrap(), &mut archive);
    }

    archive.finish().unwrap();
}

