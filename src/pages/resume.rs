use maud::{Markup, html};

pub fn resume() -> Markup {
    super::page_template(
        Some("resume"),
        html!(
            h2 { "Experience" }
            p { "Most of my experience is in development with Go and Java, alongside online community management. I've worked with a wide range of modern technologies, chosen carefully for the best performance and developer experience." }
            h2 { "Stray.gg - Administrator & Back-End Developer (July 2022 - August 2024)" }
            p { "My role at Stray involved frequent community interactions and consistent gathering of feedback to improve the service we provided and ensure player safety. I was involved in the development of several projects such as back-end APIs, Discord integrations and contributing to core server plugins." }
            h2 { "RoyalKits - Game Developer & SysAdmin (October 2023)" }
            p { "During the time RoyalKits was active, I developed core game functionality and managed cloud infrastructure to allow for changes to be made easily by other project maintainers and community managers." }
            h2 { "HydraClient - Website Developer & Designer (June 2023)" }
            p { "Using design tools like Figma, I created mockups for the landing page and produced a functional version using modern web technologies." }
            h2 { "OmniFlow - Graphics Designer (July 2024)" }
            p { "I created UI mockups and branding for the project, taking into consideration feedback from project managers and adjusting accordingly." }
        ),
    )
}
