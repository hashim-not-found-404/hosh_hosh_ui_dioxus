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
            class: "btn btn--primary",
            class: if props.disabled { "btn--disabled" },
            disabled: props.disabled,
            onclick: props.onclick,
            span { class: "btn__label", "{props.label}" }
        }
    }
}

#[component]
pub fn SecondaryButton(props: SecondaryButtonProps) -> Element {
    rsx! {
        button {
            class: "btn btn--secondary",
            class: if props.disabled { "btn--disabled" },
            disabled: props.disabled,
            onclick: props.onclick,
            span { class: "btn__label", "{props.label}" }
        }
    }
}

#[component]
pub fn TertiaryButton(props: TertiaryButtonProps) -> Element {
    rsx! {
        button {
            class: "btn btn--tertiary",
            class: if props.disabled { "btn--disabled" },
            disabled: props.disabled,
            onclick: props.onclick,
            span { class: "btn__label", "{props.label}" }
        }
    }
}

#[component]
pub fn InputField(props: InputFieldProps) -> Element {
    let value = props.value.clone();
    let placeholder = props.placeholder.clone();
    let label = props.label.clone();
    let has_value = !value.is_empty();

    rsx! {
        div {
            class: "input",
            class: if has_value { "input--has-value" },
            class: if props.error.is_some() { "input--error" },
            class: if props.disabled { "input--disabled" },

            input {
                class: "input__field",
                r#type: "text",
                placeholder: "{placeholder}",
                value: "{value}",
                disabled: props.disabled,
                oninput: props.oninput,
            }

            if !label.is_empty() {
                label {
                    class: "input__label",
                    "{label}"
                }
            }

            if let Some(err) = props.error.clone() {
                if !err.is_empty() {
                    label { class: "input__error-label", "{err}" }
                }
            }
        }
    }
}

#[component]
pub fn InputFieldWithList<const N: usize>(props: InputFieldWithListProps<N>) -> Element {
    let value = props.value.clone();
    let placeholder = props.placeholder.clone();
    let label = props.label.clone();
    let has_value = !value.is_empty();

    rsx! {
        div {
            class: "input-list",
            class: if has_value { "input-list--has-value" },
            class: if props.error.is_some() { "input-list--error" },
            class: if props.disabled { "input-list--disabled" },

            input {
                class: "input-list__field",
                r#type: "text",
                placeholder: "{placeholder}",
                value: "{value}",
                disabled: props.disabled,
                oninput: props.oninput,
            }

            if !label.is_empty() {
                label { class: "input-list__label", "{label}" }
            }

            if !props.list.is_empty() && !props.disabled {
                div { class: "input-list__items",
                    for item in props.list {
                        div { class: "input-list__item",
                            for part in item {
                                "{part} "
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CheckBox(props: CheckBoxProps) -> Element {
    rsx! {
        div {
            class: "checkbox",
            class: if props.value { "checkbox--checked" },
            class: if props.disabled { "checkbox--disabled" },
            onclick: move |evt| {
                if !props.disabled {
                    props.onclick.call(evt);
                }
            },

            div { class: "checkbox__box",
                div { class: "checkbox__check" }
            }

            if !props.label.is_empty() {
                span { class: "checkbox__label", "{props.label}" }
            }
        }
    }
}

#[component]
pub fn ComboBox(props: ComboBoxProps) -> Element {
    let value_name = props.value.map(|v| v.to_string()).unwrap_or_default();

    rsx! {
        div { class: "combo", class: if props.disabled { "combo--disabled" },

            if !props.label.is_empty() {
                label { class: "combo__label", "{props.label}" }
            }

            select {
                class: "combo__select",
                disabled: props.disabled,
                onclick: props.onclick,
                value: "{value_name}",

                for (i, item) in props.list.iter().enumerate() {
                    option { value: "{i}", selected: props.value == Some(i), "{item}" }
                }
            }
        }
    }
}
