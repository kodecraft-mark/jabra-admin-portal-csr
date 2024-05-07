use leptos::*;

use super::{component_size::ComponentSize, component_type::ComponentType};

#[allow(non_snake_case)]
#[component]
pub fn Spinners(size: ComponentSize, _type: ComponentType) -> impl IntoView {
    let size_css = match size {
        ComponentSize::EXTRASMALL => "loading-xs",
        ComponentSize::SMALL => "loading-sm",
        ComponentSize::MEDIUM => "loading-md",
        ComponentSize::LARGE => "loading-lg",
        ComponentSize::EXTRALARGE => "loading-xl",
    };
    let type_css = match _type {
        ComponentType::PRIMARY => "text-primary",
        ComponentType::SECONDARY => "text-secondary",
        ComponentType::ACCENT => "text-accent",
        ComponentType::NEUTRAL => "text-neutral",
        ComponentType::SUCCESS => "text-success",
        ComponentType::ERROR => "text-danger",
        ComponentType::WARNING => "text-warning",
        ComponentType::INFO => "text-info",
        ComponentType::BASE => ""
    };

    let css = move || format!("loading loading-spinner {} {}", size_css, type_css);
    view! {
        <span class = {css()} ></span>
    }
}