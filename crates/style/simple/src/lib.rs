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
        div { class: "input", class: if props.disabled { "input--disabled" },

            div { class: "input__control",
                input {
                    class: "input__field",
                    class: if has_value { "input__field--has-value" },
                    class: if props.error.is_some() { "input__field--error" },
                    r#type: "text",
                    placeholder: "{placeholder}",
                    value: "{value}",
                    disabled: props.disabled,
                    oninput: props.oninput,
                }

                if !label.is_empty() {
                    label {
                        class: "input__label",
                        class: if has_value { "input__label--has-value" },
                        class: if props.error.is_some() { "input__label--error" },
                        "{label}"
                    }
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
    let mut open = use_signal(|| false);
    let cols = N;

    let value = props.value.clone();
    let placeholder = props.placeholder.clone();
    let label = props.label.clone();
    let has_value = !value.is_empty();
    let show_list = open() && !props.list.is_empty() && !props.disabled;

    rsx! {
        div { class: "input", class: if props.disabled { "input--disabled" },

            div { class: "input__control",
                input {
                    class: "input__field",
                    class: if has_value { "input__field--has-value" },
                    class: if props.error.is_some() { "input__field--error" },
                    r#type: "text",
                    placeholder: "{placeholder}",
                    value: "{value}",
                    disabled: props.disabled,
                    oninput: move |e| {
                        open.set(true);
                        props.oninput.call(e);
                    },
                    onfocus: move |_| open.set(true),
                    onblur: move |_| open.set(false),
                    onkeydown: move |e: KeyboardEvent| {
                        if e.key() == Key::Escape {
                            open.set(false);
                        }
                    },
                }

                if !label.is_empty() {
                    label {
                        class: "input__label",
                        class: if has_value { "input__label--has-value" },
                        class: if props.error.is_some() { "input__label--error" },
                        "{label}"
                    }
                }

                if show_list {
                    div {
                        class: "input__dropdown",
                        style: "grid-template-columns: repeat({cols}, auto);",
                        for (index, item) in props.list.iter().enumerate() {
                            div {
                                class: "input__dropdown-item",
                                onmousedown: move |e: MouseEvent| {
                                    e.prevent_default();
                                    props.onselect.call(index);
                                    open.set(false);
                                },
                                for part in item.iter() {
                                    span { class: "input__dropdown-part", "{part}" }
                                }
                            }
                        }
                    }
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
    let has_value = props.value.is_some();
    let selected = props.value.map(|v| v.to_string()).unwrap_or_default();

    rsx! {
        div { class: "combo", class: if props.disabled { "combo--disabled" },

            div { class: "combo__control",
                select {
                    class: "combo__select",
                    class: if props.error.is_some() { "combo__select--error" },
                    disabled: props.disabled,
                    onclick: props.onclick,
                    value: "{selected}",

                    // empty placeholder so nothing is pre-selected when value is None
                    option {
                        class: "combo__item",
                        value: "", hidden: true, disabled: true, selected: !has_value }

                    for (i, item) in props.list.iter().enumerate() {
                        option {
                            class: "combo__item",
                            value: "{i}", selected: props.value == Some(i), "{item}" }
                    }
                }

                if !props.label.is_empty() {
                    label {
                        class: "combo__label",
                        class: if has_value { "combo__label--has-value" },
                        class: if props.error.is_some() { "combo__label--error" },
                        "{props.label}"
                    }
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
