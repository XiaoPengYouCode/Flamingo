use dioxus::prelude::*;

#[component]
pub fn app_header() -> Element {
    rsx!(
        div { class: "navbar bg-base-200",
            div { class: "ps-4",
                div { class: "flex items-center text-lg font-bold",
                    img { src: asset!("assets/logo.png"), class: "w-16 h-16" }
                    "血氧饱和度/心率可视化工具"
                }
            }
            div { class: "flex flex-grow justify-end px-2",
                label { class: "toggle text-base-content my-auto mx-5",
                    input {
                        r#type: "checkbox",
                        value: "synthwave",
                        class: "theme-controller",
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        g {
                            stroke_linejoin: "round",
                            stroke_linecap: "round",
                            stroke_width: "2",
                            fill: "none",
                            stroke: "currentColor",
                            circle { cx: "12", cy: "12", r: "4" }
                            path { d: "M12 2v2" }
                            path { d: "M12 20v2" }
                            path { d: "m4.93 4.93 1.41 1.41" }
                            path { d: "m17.66 17.66 1.41 1.41" }
                            path { d: "M2 12h2" }
                            path { d: "M20 12h2" }
                            path { d: "m6.34 17.66-1.41 1.41" }
                            path { d: "m19.07 4.93-1.41 1.41" }
                        }
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        g {
                            stroke_linejoin: "round",
                            stroke_linecap: "round",
                            stroke_width: "2",
                            fill: "none",
                            stroke: "currentColor",
                            path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                        }
                    }
                }
                div { class: "btn btn-secondary my-auto mr-2", "健康管理" }
                details { class: "flex-none dropdown",
                    summary { class: "btn btn-primary", "其他" }
                    ul { class: "menu dropdown-content bg-base-100 w-16 shadow-sm rounded-box z-10 p-2 ",
                        li { "关于" }
                        li { "帮助" }
                    }
                }
            }
        }
    )
}
