use charming::{
    component::{Axis, Title},
    element::{formatter::FormatterFunction, AreaStyle, AxisType, LineStyle, Tooltip},
    series::Line,
    theme::Theme,
    wasm_renderer::WasmRenderer,
    Chart,
};
use dioxus::prelude::*;

const CHART_STYLE: &str = "bg-gray-300 p-7 rounded-2xl";

#[component]
pub fn charming_app() -> Element {
    let renderer = use_signal(|| WasmRenderer::new(700, 200).theme(Theme::Westeros));
    use_effect(move || {
        let chart = Chart::new()
            .tooltip(Tooltip::new().formatter(FormatterFunction::new_with_args(
                "params",
                r#"
                    var tooltip = "Value: ".concat(String(params.value));
                    return tooltip;
                "#,
            )))
            .title(Title::new().text("传感器1").left("center"))
            .x_axis(
                Axis::new()
                    .name("时间")
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .y_axis(Axis::new().name("值").type_(AxisType::Value))
            .series(
                Line::new()
                    .smooth(true)
                    .area_style(AreaStyle::new().color("rgba(0, 0, 9, 0.3)"))
                    .line_style(LineStyle::new().color("rgba(0, 0, 9, 0.9)"))
                    .data(vec![150, 230, 224, 218, 135, 147, 260]),
            );
        renderer.read_unchecked().render("chart", &chart).unwrap();
    });

    rsx! (
        div { class: "{CHART_STYLE}",
            div { id: "chart" }
        }
    )
}
#[component]
pub fn charming_app_2() -> Element {
    let renderer = use_signal(|| WasmRenderer::new(700, 200).theme(Theme::Roma));
    use_effect(move || {
        let chart = Chart::new()
            .tooltip(Tooltip::new().formatter(FormatterFunction::new_with_args(
                "params",
                r#"
                    var tooltip = "Value: ".concat(String(params.value));
                    return tooltip;
                "#,
            )))
            .title(Title::new().text("传感器2").left("center"))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
                    .name("日期"),
            )
            .y_axis(Axis::new().type_(AxisType::Value).name("值"))
            .series(
                Line::new()
                    .smooth(true)
                    .area_style(AreaStyle::new().color("rgba(0, 0, 9, 0.3)"))
                    .line_style(LineStyle::new().color("rgba(0, 0, 9, 0.9)"))
                    .data(vec![150, 230, 224, 218, 135, 147, 260]),
            );
        renderer.read_unchecked().render("chart2", &chart).unwrap();
    });

    rsx! (
        div { class: "{CHART_STYLE}",
            div { id: "chart2", style: "display: inline-block;" }
        }
    )
}
