use dioxus::prelude::*;

#[component]
pub fn app_footer() -> Element {
    rsx!(
        footer { class: "footer footer-center bg-base-300 text-base-content p-4",
            p { "Copyright © 2025 - All right reserved by ACME Industries Ltd<" }
        }
    )
}
