use dioxus::prelude::*;
use dioxus_primitives::scroll_area::{self, ScrollAreaProps};
pub use dioxus_primitives::scroll_area::ScrollDirection;

#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    scroll_area::ScrollArea(props)
}
