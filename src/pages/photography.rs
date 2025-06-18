use std::fs;

use maud::{Markup, html};

use crate::PHOTOS_DIR;

pub fn photography() -> Markup {
    super::page_template(Some("photography"), html!((gallery())))
}

pub fn gallery() -> Markup {
    let dir = fs::read_dir(PHOTOS_DIR).expect("no photos dir");
    let mut files: Vec<String> = vec![];
    for entry in dir {
        let file_name = entry.unwrap().file_name().into_string().unwrap();
        files.push(file_name);
    }
    html!(
        div {
            @for name in &files {
                (photo(name))
            }
        }
    )
}

pub fn photo(file_name: &str) -> Markup {
    html!(
        img src=(format!("/photos/{file_name}")) {}
    )
}
