#![allow(non_snake_case)]

use dioxus::prelude::*;

use flamingo::view::app_header::*;
use flamingo::view::app_main::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("assets/tailwind.css") }
        div { class: "flex flex-col items-center gap-6 md:gap-0 rounded-2xl bg-gray-100 h-screen",
            app_header {}
            app_main {}
        }
    }
}
