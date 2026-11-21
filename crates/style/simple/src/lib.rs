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
            class: "button button__primary",
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
            class: "button button__secondary",
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
            class: "button button__tertiary",
            disabled: props.disabled,
            onclick: props.onclick,
            "{props.label}"
        }
    }
}

#[component]
pub fn InputField(props: InputFieldProps) -> Element {
    let value = props.value;
    let placeholder = props.placeholder.clone();

    rsx! {
        input {
            class: "input input__field",
            r#type: "text",
            placeholder: "{placeholder}",
            value: "{value}",
            disabled: props.disabled,
            oninput: props.oninput,
        }
    }
}
