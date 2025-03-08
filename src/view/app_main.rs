use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use futures_util::StreamExt;
use gloo_net::websocket::{futures::WebSocket, Message};

use crate::view::app_oscilloscope::*;
use crate::view::device_info::*;
use crate::view::info_panel::*;
use crate::web_socket_conm::*;

#[component]
pub fn app_main() -> Element {
    let mut test_value = use_signal(|| WebSocketData::new(0f32, 0f32, "".to_string()));
    let web_socket_device =
        use_signal(|| WebSocketDevice::new("*.*.*.*".to_string(), "*.*.*.*.*.*".to_string()));
    use_context_provider(|| web_socket_device);
    use_context_provider(|| test_value);
    let is_start = use_signal(|| false);
    let is_connected = use_signal(|| false);
    let _ = use_resource(move || async move {
        if *is_start.read() {
            info!("start to connet ws");
            if let Ok(ws) = WebSocket::open("ws://127.0.0.1:5800/ws?id=123") {
                info!("success to connet ws");
                let (_write, mut read) = ws.split();
                let mut frame = 0;
                while let Some(msg) = read.next().await {
                    frame += 1;
                    info!("frame: {}", frame);
                    let mmsg = match msg {
                        Ok(msg) => match msg {
                            Message::Text(text) => text,
                            Message::Bytes(bytes) => bytes[0].to_string(),
                        },
                        Err(e) => {
                            info!("error: {:?}", e);
                            continue;
                        }
                    };
                    let new_data = mmsg.parse::<u32>().unwrap() as f32;
                    let new_data2 = mmsg.parse::<u32>().unwrap() as f32;
                    test_value.set(WebSocketData::new(
                        new_data,
                        new_data2,
                        "2021-09-01".to_string(),
                    ));
                }
            } else {
                info!("error to connet ws");
            }
        } else {
            info!("Wait to start");
        }
    });

    rsx!(
        div { class: "flex flex-col items-center m-2",
            div { class: "flex gap-4",
                div { class: "basis-3/4 bg-gray-300 p-5 rounded-2xl", app_oscilloscope {} }
                div { class: "basis-1/4 grid gap-4",
                    info_panel { is_start, is_connected }
                    device_info { is_connected, is_start }
                }
            }
        }
    )
}
