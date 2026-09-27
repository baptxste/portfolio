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
    let mut root_tree: BTreeMap<String, FileNode> = BTreeMap::new();

    for note in notes {
        let rel_path = note.relative_path.clone();
        let parts: Vec<&str> = rel_path.split('/').collect();

        insert_into_tree(&mut root_tree, &parts, note);
    }

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("./style.css")
        }

        document::Link {
            rel: "stylesheet",
            href: asset!("../../../assets/master.css")
        }

        aside {
            class: "file-tree",

            div {
                class: "file-tree-header",
                span { "Explorateur Vault" }
            }

            {render_tree_level(root_tree)}
        }
    }
}

fn insert_into_tree(
    tree: &mut BTreeMap<String, FileNode>,
    parts: &[&str],
    note: NoteMetaData,
) {
    if parts.is_empty() {
        return;
    }

    if parts.len() == 1 {
        tree.insert(parts[0].to_string(), FileNode::File(note));
    } else {
        let entry = tree
            .entry(parts[0].to_string())
            .or_insert_with(|| FileNode::Folder(BTreeMap::new()));

        if let FileNode::Folder(sub_tree) = entry {
            insert_into_tree(sub_tree, &parts[1..], note);
        }
    }
}

fn render_tree_level(tree: BTreeMap<String, FileNode>) -> Element {
    rsx! {
        ul {
            class: "file-tree-list",

            for (name, node) in tree {
                li {
                    key: "{name}",

                    match node {
                        FileNode::File(note) => rsx! {
                            Link {
                                to: Route::NotePage {
                                    slug: note.slug.clone()
                                },

                                class: "file-tree-file",

                                span {
                                    "{note.title}"
                                }
                            }
                        },

                        FileNode::Folder(sub_tree) => rsx! {
                            details {
                                class: "file-tree-folder",
                                open: true,

                                summary {
                                    class: "file-tree-folder-summary",

                                    span {
                                        class: "folder-name",
                                        "{name}"
                                    }
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