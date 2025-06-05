use maud::{Markup, html};

pub fn index() -> Markup {
    super::page_template(
        None,
        html!(
            p { "index!!" }
        ),
    )
}
