use prelude::document::Stylesheet;
use prelude::*;

const STYLES: Asset = asset!("/assets/style.css");

#[component]
pub fn StyleInjector() -> Element {
    rsx! {
        Stylesheet { href: STYLES }
    }
}

#[component]
pub fn PrimaryButton(props: PrimaryButtonProps) -> Element {
    rsx! {
        button {
            class: "button button--primary",
            r#type: "button",
            disabled: props.disabled,
            onclick: props.onclick,
            span { class: "button__label", "{props.label}" }
        }
    }
}
