use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PrimaryButtonProps {
    pub label: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
}

#[derive(Clone, PartialEq, Props)]
pub struct SecondaryButtonProps {
    pub label: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
}

#[derive(Clone, PartialEq, Props)]
pub struct TertiaryButtonProps {
    pub label: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
}

#[derive(Clone, PartialEq, Props)]
pub struct InputFieldProps {
    pub value: String,

    #[props(default)]
    pub label: String,

    #[props(default)]
    pub placeholder: String,

    #[props(default)]
    pub error: Option<String>,

    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub oninput: EventHandler<FormEvent>,
}

#[derive(Clone, PartialEq, Props)]
pub struct InputFieldWithListProps<const N: usize> {
    pub value: String,

    #[props(default)]
    pub label: String,

    #[props(default)]
    pub placeholder: String,

    #[props(default)]
    pub error: Option<String>,

    #[props(default)]
    pub list: Vec<[String; N]>,

    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub oninput: EventHandler<FormEvent>,
    #[props(default)]
    pub onselect: EventHandler<usize>,
}

#[derive(Clone, PartialEq, Props)]
pub struct CheckBoxProps {
    pub value: bool,

    #[props(default)]
    pub label: String,

    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
}

#[derive(Clone, PartialEq, Props)]
pub struct ComboBoxProps {
    pub value: Option<usize>,

    #[props(default)]
    pub list: Vec<String>,

    #[props(default)]
    pub label: String,

    #[props(default)]
    pub error: Option<String>,

    #[props(default = false)]
    pub disabled: bool,

    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
}
