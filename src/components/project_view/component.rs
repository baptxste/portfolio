use dioxus::prelude::*;
use crate::Route;

// --- Data types (owned here, used by both ProjectView and ProjectCard) ---

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectLink {
    pub label: String,
    pub url: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectImage {
    pub url: String,
    pub alt: String,
    pub caption: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectSection {
    pub title: String,
    pub content: String,
    pub images: Vec<ProjectImage>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectItem {
    pub slug: String,
    pub title: String,
    pub tageline: String,
    pub summary: String,
    pub cover_image: String,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub status: Option<String>,
    pub links: Vec<ProjectLink>,
    pub highlights: Vec<String>,
    pub sections: Vec<ProjectSection>,
}

#[component]
pub fn ProjectView(project: ProjectItem) -> Element {
    rsx! {
        div { class: "project-detail-container",
            // Back button
            Link {
                to: Route::ProjectsHome {},
                class: "project-back-btn",
                span { "←" }
                span { "Retour à tous les projets" }
            }

            // Header
            header { class: "project-detail-header",
                div { class: "project-detail-meta-row",
                    if let Some(status) = &project.status {
                        span { class: "project-status-badge", "{status}" }
                    }
                    if let Some(date) = &project.date {
                        span { style: "color: var(--text-soft); font-weight: 600;", "{date}" }
                    }
                }

                h1 { class: "project-detail-title", "{project.title}" }
                p { class: "project-detail-tagline", "{project.tageline}" }

                div { class: "project-card-tags",
                    for tag in &project.tags {
                        span { key: "{tag}", class: "project-tag", "#{tag}" }
                    }
                }

                if !project.links.is_empty() {
                    div { class: "project-detail-actions",
                        for link in &project.links {
                            a {
                                key: "{link.url}",
                                href: "{link.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "sb-pill-green",
                                style: "display: inline-flex; align-items: center; gap: 0.5rem; padding: 0.6rem 1.2rem; font-size: 0.85rem; text-decoration: none;",
                                svg {
                                    width: "16",
                                    height: "16",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    path {
                                        d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                                    }
                                }
                                span { "{link.label}" }
                                span { "↗" }
                            }
                        }
                    }
                }
            }

            // Cover image
            div { class: "project-detail-cover",
                img {
                    src: "{project.cover_image}",
                    alt: "{project.title}",
                }
            }

            // Body
            div { class: "project-detail-content",
                // Overview
                section { class: "project-detail-section",
                    h2 { "Vue d'ensemble" }
                    p { "{project.summary}" }
                }

                // Highlights
                if !project.highlights.is_empty() {
                    section { class: "project-detail-section",
                        h2 { "Points Clés & Réalisations" }
                        ul { class: "project-highlights-list",
                            for item in &project.highlights {
                                li { key: "{item}", "{item}" }
                            }
                        }
                    }
                }

                // Sections
                for section in &project.sections {
                    section { key: "{section.title}", class: "project-detail-section",
                        h2 { "{section.title}" }
                        p { "{section.content}" }

                        if !section.images.is_empty() {
                            div { class: "project-gallery-grid",
                                for img in &section.images {
                                    figure { key: "{img.url}", class: "project-gallery-item",
                                        img {
                                            src: "{img.url}",
                                            alt: "{img.alt}",
                                            loading: "lazy",
                                        }
                                        if let Some(caption) = &img.caption {
                                            figcaption { "{caption}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
