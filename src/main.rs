use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use maud::Markup;

mod components;
mod pages;

const OUTPUT_DIR: &str = "build";
const STATIC_DIR: &str = "static";

fn main() -> io::Result<()> {
    // init build dir
    if Path::new(OUTPUT_DIR).exists() {
        fs::remove_dir_all(OUTPUT_DIR)?;
    }
    fs::create_dir_all(format!("{OUTPUT_DIR}/static"))?;
    copy_dir_all(STATIC_DIR, format!("{OUTPUT_DIR}/static/"))?;

    // generate static pages
    generate_page(false, "index", pages::index::index())?;

    Ok(())
}

fn generate_page(blog_post: bool, file_name: &str, page: Markup) -> io::Result<()> {
    let output_file = match blog_post {
        true => format!("./{OUTPUT_DIR}/posts/{file_name}.html"),
        false => format!("./{OUTPUT_DIR}/{file_name}.html"),
    };
    let mut file = File::create(output_file)?;
    let content = page.into_string();
    write!(file, "{}", content)?;
    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
