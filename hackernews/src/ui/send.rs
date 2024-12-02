use dioxus::prelude::*;

#[component]
pub fn Send() -> Element {
    rsx!(
        section { class: "mt-6 mb-3 border rounded-xl bg-gray-50",
            textarea {
                placeholder: "Type your reply here...",
                rows: "3",
                class: "w-full p-2 bg-gray-50 rounded-xl"
            }
            div { class: "flex items-center justify-between p-2",
                button { class: "w-6 h-6 text-gray-400",
                    svg {
                        "xmlns": "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        "stroke": "currentColor",
                        "fill": "none",
                        path {
                            "stroke-linecap": "round",
                            "d": "M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13",
                            "stroke-width": "2",
                            "stroke-linejoin": "round"
                        }
                    }
                }
                button { class: "px-6 py-2 text-white bg-purple-600 rounded-xl", "Reply" }
            }
        }
    )
}