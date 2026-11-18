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
pub fn PrimaryButton(props: PrimaryButtonProps) -> Element {
    rsx! {
        button {
            class: "hhs-button hhs-button--primary",
            r#type: "button",
            disabled: props.disabled,
            onclick: props.onclick,
            span { class: "hhs-button__label", "{props.label}" }
        }
    }
}
