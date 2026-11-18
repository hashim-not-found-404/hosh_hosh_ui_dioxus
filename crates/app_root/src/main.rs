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
            style: "min-height: 100vh; padding: 3rem; \
                    background: var(--hhs-bg); \
                    display: flex; flex-direction: column; \
                    align-items: center; justify-content: center; \
                    gap: 2rem;",
            h1 {
                style: "font-family: var(--hhs-font-mono); \
                        color: var(--hhs-text); \
                        letter-spacing: 0.1em; \
                        text-transform: uppercase; \
                        font-weight: 500;",
                "hosh hosh ui · simple"
            }

            div {
                style: "display: flex; gap: 1rem; flex-wrap: wrap; \
                        justify-content: center;",
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

            p {
                style: "font-family: var(--hhs-font-mono); \
                        color: var(--hhs-text-muted); \
                        font-size: 0.85rem;",
                "Hover · press · tab-focus — all handled by CSS."
            }
        }
    }
}
