use maud::{Markup, html};

pub fn navbar() -> Markup {
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
