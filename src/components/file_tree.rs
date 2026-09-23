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
        aside { class: "w-full md:w-64 bg-gray-50 dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 p-4 font-mono text-sm overflow-y-auto flex-shrink-0",
            div { class: "font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider text-xs mb-3 px-2 flex items-center justify-between",
                span { "Explorateur Vault" }
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
        ul { class: "space-y-1 pl-2 border-l border-gray-200 dark:border-gray-800 ml-1",
            for (name, node) in tree {
                li { key: "{name}",
                    match node {
                        FileNode::File(note) => rsx! {
                            Link {
                                to: Route::NotePage { slug: note.slug.clone() },
                                class: "flex items-center gap-1.5 px-2 py-1 rounded text-gray-700 dark:text-gray-300 hover:bg-indigo-50 dark:hover:bg-indigo-950/50 hover:text-indigo-600 dark:hover:text-indigo-400 transition truncate",
                                span { class: "truncate", "{note.title}" }
                            }
                        },
                        FileNode::Folder(sub_tree) => rsx! {
                            details { open: true, class: "group",
                                summary { class: "flex items-center gap-1.5 px-2 py-1 font-semibold text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white cursor-pointer select-none rounded hover:bg-gray-100 dark:hover:bg-gray-800 transition",
                                    span { class: "truncate", "{name}" }
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
