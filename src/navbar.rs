use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    rsx! {
        div { class: "flex flex-1 flex-col gap-4 items-start justify-start",
            div {
                class: "navbar bg-base-200 flex flex-col gap-5 items-start justify-start",
                id: "navbar",
                h1 { class: "text-4xl font-bold", "OneProg Главная v{VERSION}" }
            }
            div { class: "flex flex-row gap-5 m-2 items-start jusify-start",
                a {
                    class: "btn btn-outline btn-primary w-auto",
                    href: "https://git.oneprog.org",
                    "Github"
                }
                a {
                    class: "btn btn-primary w-auto",
                    href: "https://contest.oneprog.org",
                    "Контест"
                }
            }
        }
    }
}
