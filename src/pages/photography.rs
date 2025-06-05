use maud::{Markup, html};

pub fn photography() -> Markup {
    super::page_template(
        Some("photography"),
        html!(
            p { "nothing here yet :c" }
        ),
    )
}
