use dioxus::prelude::*;

use crate::web_socket_conm::*;

#[component]
pub fn device_info(is_connected: Signal<bool>, is_start: Signal<bool>) -> Element {
    let device_info = use_context::<Signal<WebSocketDevice>>();
    let indicator_container =
        "bg-sky-300 text-white p-4 rounded-lg m-4 pl-8 pr-8 shadow-xl text-xl font-bold hover:bg-sky-400";
    let device_ip_fmt = format!("IP 地址: {}", device_info.read().get_ip());
    let device_mac_fmt = format!("MAC 地址: {}", device_info.read().get_mac());
    rsx!(
        div { class: "card bg-gray-300",
            div { class: "card-body",
                div { class: "card-title text-black", "设备信息" }
                div {
                    div { class: "{indicator_container}",
                        div { "{device_ip_fmt}" }
                        div { "{device_mac_fmt}" }
                    }
                }
                div { class: "grid grid-cols-2 gap-2 card-actions",
                    button {
                        class: "btn btn-success disabled:text-gray-400",
                        disabled: *is_connected.read(),
                        onclick: move |_| {
                            is_connected.set(true);
                            is_start.set(false);
                        },
                        "连接设备"
                    }
                    button {
                        class: "btn btn-error disabled:text-gray-400",
                        disabled: !*is_connected.read(),
                        onclick: move |_| {
                            is_connected.set(false);
                            is_start.set(false);
                        },
                        "断开设备"
                    }
                }
            }
        }
    )
}
