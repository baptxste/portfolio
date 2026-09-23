use dioxus::prelude::*;

mod components;
mod content;
mod views;

use content::get_vault_index;
use views::{Home, Navbar, NotePage, TagPage};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/notes/:slug")]
        NotePage { slug: String },
        #[route("/tags/:tag")]
        TagPage { tag: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const MARKDOWN_CSS: Asset = asset!("/assets/styling/markdown.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Provide vault index globally to all components
    let index = use_signal(get_vault_index);
    use_context_provider(|| index());

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: MARKDOWN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // KaTeX Math Stylesheet & JS Auto-render script
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/katex.min.css"
        }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/katex.min.js"
        }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/katex@0.16.8/dist/contrib/auto-render.min.js"
        }

        Router::<Route> {}
    }
}
