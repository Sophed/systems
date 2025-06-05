use maud::{DOCTYPE, Markup, html};

pub mod index;
pub mod photography;
pub mod posts;
pub mod resume;

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
            link rel="stylesheet" href="https://sophed.github.io/iosevka-webfont/7.0.2/iosevka.css";
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

fn navbar() -> Markup {
    html!(
        ul."nav" {
            li."left" {
                a href="/" { "home" }
            }
            li."left" {
                a href="/resume" { "resume" }
            }
            li."right" {
                a href="/posts" { "posts" }
            }
            li."right" {
                a href="/photography" { "photography" }
            }
        }
    )
}
