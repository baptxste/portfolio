use dioxus::prelude::*;
use crate::i18n::{tr, Language};
use crate::components::project_card::ProjectCard;
use crate::views::projects::{
    reproduction_study, portfolio_website, neural_network, dimensionality_reduction,
};

fn all_projects() -> Vec<crate::components::project_view::ProjectItem> {
    vec![
        reproduction_study::project_data(),
        portfolio_website::project_data(),
        neural_network::project_data(),
        dimensionality_reduction::project_data(),
    ]
}

#[component]
pub fn ProjectsHome() -> Element {
    let lang: Signal<Language> = use_context();
    let l = lang();
    let projects = all_projects();

    rsx! {
        div { class: "projects-container",
            // Header Section
            header { class: "projects-header",
                h1 { {tr(l, "Projets & Réalisations", "Projects & Work")} }
                p {
                    {tr(
                        l,
                        "Découvrez une sélection de projets logiciels, reproductions de recherche en Deep Learning et outils open source.",
                        "Discover a selection of software projects, Deep Learning research reproductions, and open-source tools."
                    )}
                }
            }

            // Projects list with alternating image position
            div { class: "projects-list",
                for (i, project) in projects.into_iter().enumerate() {
                    ProjectCard {
                        key: "{project.slug}",
                        project: project.clone(),
                        index: i,
                    }
                }
            }
        }
    }
}
