use std::fs::{self, DirEntry, File};
use std::io::{self, Write};
use std::path::Path;

use chrono::NaiveDate;
use maud::Markup;

mod pages;

const OUTPUT_DIR: &str = ".build";
const STATIC_DIR: &str = "static";
const POSTS_DIR: &str = "posts";

#[derive(Clone)]
struct PostData {
    file_name: String,
    title: String,
    date: String,
    content: String,
}

fn main() -> io::Result<()> {
    // init build dir
    if Path::new(OUTPUT_DIR).exists() {
        fs::remove_dir_all(OUTPUT_DIR)?;
    }
    fs::create_dir_all(format!("{OUTPUT_DIR}/static"))?;
    fs::create_dir(format!("{OUTPUT_DIR}/posts"))?;
    copy_dir_all(STATIC_DIR, format!("{OUTPUT_DIR}/static"))?;

    // generate static pages
    generate_page(false, "resume", pages::resume::resume())?;
    generate_page(false, "photography", pages::photography::photography())?;

    // generate posts from markdown
    let mut post_list: Vec<PostData> = vec![];
    let dir = fs::read_dir(POSTS_DIR)?;
    for entry in dir {
        let post_data = get_post_data(entry.unwrap())?;
        generate_page(
            true,
            post_data.file_name.as_str(),
            pages::posts::post_template(&post_data),
        )?;
        post_list.push(post_data);
    }

    post_list.sort_by(|x, y| {
        let x_date = NaiveDate::parse_from_str(&x.date, "%Y-%m-%d").unwrap();
        let y_date = NaiveDate::parse_from_str(&y.date, "%Y-%m-%d").unwrap();
        x_date.cmp(&y_date)
    });
    post_list.reverse();

    generate_page(false, "posts", pages::posts::posts(post_list.clone()))?;
    generate_page(false, "index", pages::index::index(post_list.first()))?;

    Ok(println!("succesfully generated static pages"))
}

fn get_post_data(file: DirEntry) -> io::Result<PostData> {
    // format file name
    let raw_file_name = file.file_name().into_string().unwrap();
    let parts: Vec<&str> = raw_file_name.split(".md").collect();
    let file_name = parts.first().unwrap();

    let content = fs::read_to_string(file.path())?;
    let parts: Vec<&str> = content.split("\n---\n").collect();

    let meta = parts.first().unwrap().to_string();
    let md = parts.last().unwrap();

    let parts: Vec<&str> = meta.split("\n").collect();
    Ok(PostData {
        file_name: file_name.to_string(),
        title: parts.first().unwrap().to_string(),
        date: parts.last().unwrap().to_string(),
        content: md.to_string(),
    })
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
