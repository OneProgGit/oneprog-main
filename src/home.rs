use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    rsx! {
        div { class: "flex flex-col gap-4 items-start justify-start",
            div {
                class: "navbar bg-base-200 flex flex-col gap-5 items-start justify-start",
                id: "navbar",

                h1 { class: "text-4xl font-bold", {format!("OneProg Главная v{VERSION}")} }
            }
            div { class: "flex flex-row gap-5 m-2 items-start jusify-start",
                button { class: "btn btn-primary",
                    a { href: "https://git.oneprog.org", "Github" }
                }
                button { class: "btn btn-disabled", disabled: "true",
                    "Контест (временно недоступен)"
                }
            }
        }
    }
}
