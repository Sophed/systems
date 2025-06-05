use maud::{DOCTYPE, Markup, html};

use crate::components::navbar;

pub mod index;

pub fn page_template(page_title: Option<&str>, content: Markup) -> Markup {
    let page_title = match page_title {
        Some(s) => format!("{} / soph.systems", s),
        None => "soph.systems".to_string(),
    };
    html!(
        (DOCTYPE)
        head {
            title { (page_title) }
            script src="/static/htmx.min.js" {}
            link rel="icon" type="image/x-icon" href="/static/favicon.png";
            link rel="stylesheet" href="https://sophed.github.io/iosevka-webfont/3.4.1/iosevka.css";
            link rel="stylesheet" href="/static/global.css";
        }
        body {
            div."container" {
                div."titles" {
                    h2 style="float: left" { "soph.systems" }
                    h2 style="float: right" { "ᓚᘏᗢ" }
                }
                (navbar())
                hr;
                div."content" {
                    (content)
                }
            }
        }
    )
}
