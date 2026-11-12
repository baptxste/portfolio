use dioxus::prelude::*;
use crate::views::projects::{
    DimensionalityReductionProject,
    NeuralNetworkProject,
    PortfolioWebsiteProject,
    ReproductionStudyProject,
};

#[component]
pub fn ProjectPage(slug: String) -> Element {
    match slug.as_str() {
        "reproduction-study" => rsx! {
            ReproductionStudyProject {}
        },
        "portfolio-website" => rsx! {
            PortfolioWebsiteProject {}
        },
        "neural-network-from-scratch" => rsx! {
            NeuralNetworkProject {}
        },
        "dimensionality-reduction-suite" => rsx! {
            DimensionalityReductionProject {}
        },
        _ => rsx! {
            div { class: "main-content-card",
                crate::views::NotFound { route: vec!["projects".to_string(), slug] }
            }
        },
    }
}
