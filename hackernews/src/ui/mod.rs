#![allow(non_snake_case)]

mod comments;
mod stories;
mod send;

use crate::StoryData;
use comments::Comments;
use dioxus::prelude::*;
use stories::Stories;

#[derive(Debug, Clone)]
pub enum CommentsState {
    Unset,
    Loading,
    Loaded(StoryData),
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(CommentsState::Unset));
    rsx! {
        style {
            dangerous_inner_html: include_str!("../../assets/tailwind.css")
        }
        div {
            class: "h-screen w-screen flex flex-col overflow-hidden",
            div {
                class: "sticky top-0 z-50 flex items-center justify-center w-full h-12 bg-white shadow-md",
                h1 {
                    class: "text-2xl font-semibold",
                    "Hacker News"
                }
            }
            main {
                class: "flex h-[0px] w-full grow shadow-lg rounded-3xl overflow-hidden",
                section {
                    class: "flex flex-col w-4/12 pt-3 overflow-y-scroll bg-gray-50 overflow-auto",
                    Stories {}
                }
                section {
                    class: "flex flex-col w-8/12 px-4 bg-white rounded-r-3xl overflow-auto",
                    Comments {}
                }
            }
            div {
                class: "flex items-center justify-center w-full h-12 bg-white shadow-md",
                p {
                    class: "text-sm text-gray-400",
                    "Copyright © 2024 "
                }
            }
        }
    }
}
