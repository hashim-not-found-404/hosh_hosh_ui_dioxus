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
            disabled: props.disabled,
            onclick: props.onclick,
            "{props.label}"
        }
    }
}

#[component]
pub fn SecondaryButton(props: SecondaryButtonProps) -> Element {
    rsx! {
        button {
            class: "button button--secondary",
            disabled: props.disabled,
            onclick: props.onclick,
            "{props.label}"
        }
    }
}

#[component]
pub fn TertiaryButton(props: TertiaryButtonProps) -> Element {
    rsx! {
        button {
            class: "button button--tertiary",
            disabled: props.disabled,
            onclick: props.onclick,
            "{props.label}"
        }
    }
}
