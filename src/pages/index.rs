use maud::{Markup, html};

use crate::PostData;

pub fn index(latest_post: Option<&PostData>) -> Markup {
    let post_info = match latest_post {
        None => html!(
            p { "No posts available :(" }
        ),
        Some(p) => super::post_link(p),
    };
    super::page_template(
        None,
        html!(
            h2 hx-get="https://api.soph.cat/status" hx-trigger="every 5s" hx-target="#status" { "Software Engineer, Designer, & Linux Evangelist" }
            p id="status" hx-get="https://api.soph.cat/status" hx-trigger="load" { "Status: Fetching..." }
            p { strong { "Hey! " } "I'm Sophia (she/her), A student software engineer from the UK learning systems and back-end development." }
            p {
                a target="_blank" href="https://github.com/sophed" { "GitHub" }
                " - "
                a target="_blank" href="https://tech.lgbt/@null" { "Mastodon" }
                " - "
                a target="_blank" href="https://bsky.app/profile/soph.cat" { "Bluesky" }
                " - "
                a target="_blank" href="https://www.last.fm/user/sophsystems" { "LastFM" }
            }
            h2 { "Projects" }
            ul {
                (project("soph.systems", "https://soph.systems", "This website is built from a fully custom static site generator written in Rust"))
                (project("mkpr", "https://github.com/Sophed/mkpr", "Fast Zig CLI tool to create projects from a set of opinionated language templates"))
                (project("dotfiles", "https://github.com/sophed/dotfiles", "My personal Linux desktop and terminal configuration files"))
                (project("oci-claimer", "https://github.com/sophed/oci-claimer", "Automatically claim Oracle Cloud free-tier instances"))
            }
            p { "The best way to contact me is via my discord - " strong { "@sophed" } }
            h2 { "Latest post" }
            (post_info)
        ),
    )
}

fn project(title: &str, link: &str, description: &str) -> Markup {
    html!(
        li {
            p {
                a href=(link) target="_blank" { (title) }
                " : " (description)
            }
        }
    )
}
