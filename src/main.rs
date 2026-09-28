use dioxus::prelude::*;

mod components;
mod content;
mod i18n;
mod views;
mod vault_assets;

use content::get_vault_index;
use i18n::Language;
use views::{CvPage, Home, Navbar, NotePage, NotesHome, TagPage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&mut self) {
        *self = match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }

    pub fn is_dark(&self) -> bool {
        matches!(self, Theme::Dark)
    }

    pub fn to_class(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/notes")]
        NotesHome {},
        #[route("/notes/:slug")]
        NotePage { slug: String },
        #[route("/tags/:tag")]
        TagPage { tag: String },
        #[route("/cv")]
        CvPage {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MASTER_CSS: Asset = asset!("/assets/master.css");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const HOME_CSS: Asset = asset!("/assets/styling/home.css");
const NOTES_CSS: Asset = asset!("/assets/styling/notes.css");
const CV_CSS: Asset = asset!("/assets/styling/cv.css");
const MARKDOWN_CSS: Asset = asset!("/assets/styling/markdown.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Provide vault index globally to all components
    let index = use_signal(get_vault_index);
    use_context_provider(|| index());

    // Provide language signal globally
    let lang = use_signal(|| Language::Fr);
    use_context_provider(|| lang);

    // Provide theme signal globally
    let theme = use_signal(|| Theme::Light);
    use_context_provider(|| theme);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MASTER_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: HOME_CSS }
        document::Link { rel: "stylesheet", href: NOTES_CSS }
        document::Link { rel: "stylesheet", href: CV_CSS }
        document::Link { rel: "stylesheet", href: MARKDOWN_CSS }

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
