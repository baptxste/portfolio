use dioxus::prelude::*;
use crate::content::NoteMetaData;
use crate::Route;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
enum FileNode {
    File(NoteMetaData),
    Folder(BTreeMap<String, FileNode>),
}

#[component]
pub fn FileTree(notes: Vec<NoteMetaData>) -> Element {
    // Build tree structure
    let mut root_tree: BTreeMap<String, FileNode> = BTreeMap::new();

    for note in notes {
        let rel_path = note.relative_path.clone();
        let parts: Vec<&str> = rel_path.split('/').collect();
        insert_into_tree(&mut root_tree, &parts, note);
    }

    rsx! {
        aside { class: "w-full md:w-64 theme-bg-card rounded-[12px] sb-card-shadow p-5 font-sans text-xs overflow-y-auto flex-shrink-0 transition-colors duration-250",
            div { class: "font-bold theme-text-heading uppercase tracking-wider text-[11px] mb-4 px-2 flex items-center justify-between",
                span { "Explorateur Vault" }
                span { class: "w-2 h-2 rounded-full bg-[#00754A]" }
            }
            {render_tree_level(root_tree)}
        }
    }
}

fn insert_into_tree(tree: &mut BTreeMap<String, FileNode>, parts: &[&str], note: NoteMetaData) {
    if parts.is_empty() {
        return;
    }

    if parts.len() == 1 {
        tree.insert(parts[0].to_string(), FileNode::File(note));
    } else {
        let folder_name = parts[0];
        let entry = tree
            .entry(folder_name.to_string())
            .or_insert_with(|| FileNode::Folder(BTreeMap::new()));

        if let FileNode::Folder(ref mut sub_tree) = entry {
            insert_into_tree(sub_tree, &parts[1..], note);
        }
    }
}

fn render_tree_level(tree: BTreeMap<String, FileNode>) -> Element {
    rsx! {
        ul { class: "space-y-1 pl-2 ml-1",
            for (name, node) in tree {
                li { key: "{name}",
                    match node {
                        FileNode::File(note) => rsx! {
                            Link {
                                to: Route::NotePage { slug: note.slug.clone() },
                                class: "flex items-center gap-2 px-2.5 py-1.5 rounded-full theme-text-main hover:bg-[#d4e9e2] dark:hover:bg-[#24463e] hover:text-[#006241] dark:hover:text-[#d4e9e2] transition duration-150 truncate group font-medium",
                                span { class: "truncate", "{note.title}" }
                            }
                        },
                        FileNode::Folder(sub_tree) => rsx! {
                            details { open: true, class: "group/folder",
                                summary { class: "flex items-center gap-2 px-2.5 py-1.5 font-semibold theme-text-main hover:text-[#006241] dark:hover:text-[#d4e9e2] cursor-pointer select-none rounded-lg hover:bg-slate-200/50 dark:hover:bg-[#1f3c35] transition duration-150",
                                    span { class: "truncate tracking-tight", "{name}" }
                                }
                                {render_tree_level(sub_tree)}
                            }
                        }
                    }
                }
            }
        }
    }
}
