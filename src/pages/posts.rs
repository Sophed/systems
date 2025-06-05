use maud::{Markup, PreEscaped, html};

use crate::{
    PostData,
    pages::{post_link, reading_time},
};

pub fn posts(post_list: Vec<PostData>) -> Markup {
    let list = match post_list.is_empty() {
        true => html!(
            p { "No posts yet :(" }
        ),
        false => {
            let mut links: Vec<Markup> = vec![];
            for post in post_list {
                links.push(post_link(&post));
            }
            html!(
                @for link in &links {
                    (link)
                    br;
                }
            )
        }
    };
    super::page_template(Some("posts"), list)
}

pub fn post_template(post: &PostData) -> Markup {
    let content = markdown::to_html(&post.content);
    let info = format!(
        "Published: {} ~ {}m read",
        &post.date,
        reading_time(&post.content.to_string())
    );
    super::page_template(
        Some(&post.title),
        html!(
            a href="/posts" style="text-decoration: none" { "<- back to posts" }
            p { (info) }
            (PreEscaped(content))
        ),
    )
}
