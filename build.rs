use std::process::Command;
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipWriter};

fn main() {
    println!("cargo:rerun-if-changed=docs/src");

    let mdbook_result = Command::new("mdbook")
        .args(&["build", "docs"])
        .status()
        .expect("could not execute mdbook");
    if !mdbook_result.success() {
        panic!("mdbook did not run successfully");
    }

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("docs.zip");

    let files = WalkDir::new("docs/book")
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("Could not get full path list of docs/book/");
    let mut archive = ZipWriter::new(
        std::fs::File::create(&dest_path).expect(&format!("Could not create file {:?}", dest_path)),
    );
    for entry in files.into_iter().filter(|e| e.file_type().is_file()) {
        let file = entry.path();
        archive
            .start_file_from_path(
                file.strip_prefix("docs/book").unwrap_or(file),
                FileOptions::default().compression_method(zip::CompressionMethod::Deflated),
            )
            .expect(&format!("Could not start writing file {:?}", file));

        std::io::copy(&mut std::fs::File::open(file).unwrap(), &mut archive)
            .expect(&format!("Writing {:?} to zip file failed", file));
    }

    archive
        .finish()
        .expect("Could not finish zip file. So close");
}
