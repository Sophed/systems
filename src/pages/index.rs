use maud::{Markup, html};

pub fn index() -> Markup {
    super::page_template(
        None,
        html!(
            h2 { "Software Engineer, Designer, & Linux Evangelist" }
            p { "Status: -" }
            p { strong { "Hey! " } "I'm Sophia (she/her), A student software engineer from the UK learning systems and back-end development." }
            p {
                a target="_blank" href="https://github.com/sophed" { "GitHub" }
                " - "
                a target="_blank" href="https://tech.lgbt/@null" { "Mastodon" }
                " - "
                a target="_blank" href="https://bsky.app/profile/soph.cat" { "Bluesky" }
            }
            h2 { "Projects" }
            ul {
                (project("mkpr", "https://github.com/Sophed/mkpr", "Fast Zig CLI tool to create projects from a set of opinionated language templates"))
                (project("dotfiles", "https://github.com/sophed/dotfiles", "My personal Linux desktop and terminal configuration files"))
                (project("oci-claimer", "https://github.com/sophed/oci-claimer", "Automatically claim Oracle Cloud free-tier instances"))
            }
            p { "The best way to contact me is via my discord - " strong { "@sophed" } }
            h2 { "Latest post" }
            p { "-" }
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
