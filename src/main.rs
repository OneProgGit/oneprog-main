use crate::{footer::Footer, navbar::Navbar};
use dioxus::prelude::*;

mod footer;
mod navbar;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "flex flex-col min-h-screen gap-4 mx-2 items-start justify-start",
            Navbar {}
            Footer {}
        }
    }
}
