use maud::{Markup, html};

pub fn posts() -> Markup {
    super::page_template(
        Some("posts"),
        html!(
            p { "no posts yet :c" }
        ),
    )
}
