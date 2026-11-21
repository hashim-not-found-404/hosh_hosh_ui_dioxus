use hosh_hosh_ui_dioxus::dioxus::launch;
use hosh_hosh_ui_dioxus::style_simple::*;
use hosh_hosh_ui_dioxus::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let name = use_signal(|| String::new());

    rsx! {
        StyleInjector {}

        div {
            style: "
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 8px;
            ",

            PrimaryButton {
                label: "Click me",
            }
            PrimaryButton {
                label: "Disabled",
                disabled: true,
            }

            SecondaryButton {
                label: "Click me",
            }
            SecondaryButton {
                label: "Disabled",
                disabled: true,
            }

            TertiaryButton {
                label: "Click me",
            }
            TertiaryButton {
                label: "Disabled",
                disabled: true,
            }

            InputField {
                value: name,
                placeholder: "Jane Doe",
            }
            InputField {
                value: name,
                placeholder: "Can't edit me",
                disabled: true,
            }

        }
    }
}
