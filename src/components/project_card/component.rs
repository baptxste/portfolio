use dioxus::prelude::*;
use crate::components::project_view::ProjectItem;

#[component]
pub fn ProjectCard(project: ProjectItem, index: usize) -> Element {
    // Alternate image position: even index → image left, odd → image right
    let card_class = if index % 2 == 0 {
        "project-card project-card--img-left"
    } else {
        "project-card project-card--img-right"
    };

    rsx! {
        Link {
            key: "{project.slug}",
            to: crate::Route::ProjectPage { slug: project.slug.clone() },
            class: "{card_class}",

            div { class: "project-card-image-wrap",
                img {
                    src: "{project.cover_image}",
                    alt: "{project.title}",
                    class: "project-card-image",
                    loading: "lazy",
                }
            }

            div { class: "project-card-body",
                div { class: "project-card-meta",
                    if let Some(date) = &project.date {
                        span { "{date}" }
                    }
                    if let Some(status) = &project.status {
                        span { class: "project-status-badge", "{status}" }
                    }
                }

                h2 { class: "project-card-title", "{project.title}" }
                p { class: "project-card-summary", "{project.summary}" }

                div { class: "project-card-tags",
                    for tag in project.tags.iter().take(4) {
                        span { key: "{tag}", class: "project-tag", "{tag}" }
                    }
                }

                div { class: "project-card-footer",
                    span { "Découvrir le projet" }
                    span { "→" }
                }
            }
        }
    }
}
