use dioxus::prelude::*;

use crate::web_socket_conm::*;

#[component]
pub fn info_panel(is_start: Signal<bool>, is_connected: Signal<bool>) -> Element {
    let data = use_context::<Signal<WebSocketData>>();
    let indicator_container =
        "bg-sky-300 text-white p-4 rounded-lg m-4 pl-8 pr-8 shadow-xl text-xl font-bold hover:bg-sky-400";
    rsx!(
        div { class: "card bg-gray-300",
            div { class: "card-body",
                div { class: "card-title text-black", "实时数据" }
                div {
                    div { class: "{indicator_container}",
                        p { "心率" }
                        span { class: "indicator-value", "{data.read().get_data()}" }
                        "  bpm\n"
                    }
                    div { class: "{indicator_container}",
                        p { "血氧饱和度" }
                        span { class: "indicator-value", "{data.read().get_data()}" }
                        "  %\n"
                    }
                }
                div { class: "grid grid-cols-2 gap-2",
                    button {
                        onclick: move |_| {
                            is_start.set(true);
                        },
                        class: "btn btn-success disabled:text-gray-400 disabled:bg-gray-300",
                        disabled: *is_start.read() | !*is_connected.read(),
                        "开始测量"
                    }
                    button {
                        onclick: move |_| {
                            is_start.set(false);
                        },
                        class: "btn btn-error disabled:text-gray-400 disabled:bg-gray-300",
                        disabled: !*is_start.read() | !*is_connected.read(),
                        "停止测量"
                    }
                }
            }
        }
    )
}
