use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PrimaryButtonProps {
    pub label: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
}
