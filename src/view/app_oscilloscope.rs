use crate::view::app_chart::*;
use dioxus::prelude::*;

#[component]
pub fn app_oscilloscope() -> Element {
    rsx!(
        div { class: "item-center text-center text-2xl font-bold mb-7 text-black", "可视化窗口" }
        div { class: "flex iter-center justify-center m-auto bg-gray-300",
            div { class: "grid grid-cols-l gap-1 justify-center iter-center",
                div { charming_app {} }
                div { charming_app_2 {} }
            }
        }
    )
}
