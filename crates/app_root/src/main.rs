use hosh_hosh_ui_dioxus::dioxus::launch;
use hosh_hosh_ui_dioxus::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut clicks = use_signal(|| 0u32);

    rsx! {
        StyleInjector {}

        div {
            div {
                PrimaryButton {
                    label: "Click me",
                    onclick: move |_| clicks += 1,
                }
                PrimaryButton {
                    label: "Disabled",
                    disabled: true,
                }
                PrimaryButton {
                    label: format!("Clicks: {clicks}"),
                }
            }
        }
    }
}
