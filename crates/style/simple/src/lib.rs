use prelude::document::Style;
use prelude::*;

const STYLES: &str = include_str!("../assets/style.css");

#[component]
pub fn StyleInjector() -> Element {
    rsx! {
        Style { "{STYLES}" }
    }
}

#[component]
pub fn PrimaryButton(
    label: String,
    #[props(default = false)] disabled: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "hhs-button hhs-button--primary",
            r#type: "button",
            disabled: disabled,
            onclick: move |e| onclick.call(e),
            span { class: "hhs-button__label", "{label}" }
        }
    }
}
