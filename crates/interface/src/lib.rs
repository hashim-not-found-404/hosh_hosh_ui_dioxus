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
