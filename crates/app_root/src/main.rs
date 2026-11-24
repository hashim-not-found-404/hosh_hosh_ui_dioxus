use hosh_hosh_ui_dioxus::dioxus::launch;
use hosh_hosh_ui_dioxus::style_simple::*;
use hosh_hosh_ui_dioxus::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut name = use_signal(|| String::from("Hashem"));
    let mut list_input = use_signal(|| String::new());
    let mut check1 = use_signal(|| false);
    let mut check2 = use_signal(|| true);
    let combo_val = use_signal(|| Some(1usize));
    let mut choice = use_signal(|| None::<usize>);

    let list = vec![
        ["Rust".to_string(), "Web".to_string()],
        ["Dioxus".to_string(), "UI".to_string()],
    ];

    rsx! {
        StyleInjector {}

        div { style: "
            display: flex;
            flex-direction: column;
            align-items: flex-start;
            gap: 24px;
            padding: 32px;
            max-width: 400px;
            margin: 0 auto;
            ",

            h2 { style: "margin: 0; color: #fff; font-family: var(--font-mono);", "Buttons" }
            div { style: "display: flex; gap: 12px; flex-wrap: wrap;",
                PrimaryButton { label: "Primary", onclick: move |_| {} }
                PrimaryButton { label: "Disabled", disabled: true, onclick: move |_| {} }

                SecondaryButton { label: "Secondary", onclick: move |_| {} }
                SecondaryButton { label: "Disabled", disabled: true, onclick: move |_| {} }

                TertiaryButton { label: "Tertiary", onclick: move |_| {} }
                TertiaryButton { label: "Disabled", disabled: true, onclick: move |_| {} }
            }

            h2 { style: "margin: 0; color: #fff; font-family: var(--font-mono);", "Input Field" }
            InputField {
                label: "Username",
                value: name(),
                placeholder: "Enter username",
                oninput: move |e: Event<FormData>| name.set(e.value()),
            }
            InputField {
                label: "Disabled Input",
                value: "Cannot edit".to_string(),
                placeholder: "Disabled",
                disabled: true,
                oninput: move |_| {},
            }
            InputField {
                label: "With Error",
                value: name(),
                placeholder: "Error state",
                error: Some("This field is invalid".to_string()),
                oninput: move |e: Event<FormData>| name.set(e.value()),
            }

            h2 { style: "margin: 0; color: #fff; font-family: var(--font-mono);",
                "Input Field With List"
            }
            InputFieldWithList::<2> {
                label: "Tags",
                value: list_input(),
                placeholder: "Add tag",
                list: list.clone(),
                oninput: move |e: Event<FormData>| list_input.set(e.value()),
                onselect: move |e: usize| list_input.set(list[e][0].clone()),
            }
            InputFieldWithList::<1> {
                label: "Disabled List",
                value: "".to_string(),
                placeholder: "Disabled",
                list: vec![["Item".to_string()]],
                disabled: true,
                oninput: move |_| {},
            }

            h2 { style: "margin: 0; color: #fff; font-family: var(--font-mono);", "Checkboxes" }
            div { style: "display: flex; flex-direction: column; gap: 12px;",
                CheckBox {
                    label: "Unchecked",
                    value: check1(),
                    onclick: move |_| check1.toggle(),
                }
                CheckBox {
                    label: "Checked",
                    value: check2(),
                    onclick: move |_| check2.toggle(),
                }
                CheckBox {
                    label: "Disabled",
                    value: true,
                    disabled: true,
                    onclick: move |_| {},
                }
            }

            h2 { style: "margin: 0; color: #fff; font-family: var(--font-mono);", "ComboBox" }
            ComboBox {
                label: "Role",
                value: choice(),
                list: vec!["Admin".to_string(), "Editor".to_string(), "Viewer".to_string()],
                onselect: move |i: usize| choice.set(Some(i)),
            }
            ComboBox {
                label: "Select Option",
                value: combo_val(),
                list: vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string()],
            }
            ComboBox {
                label: "Select Option",
                value: None,
                list: vec!["Option A".to_string(), "Option B".to_string(), "Option C".to_string()],
            }
            ComboBox {
                label: "Disabled Combo",
                value: Some(0),
                list: vec!["Only One".to_string()],
                disabled: true,
            }
        }
    }
}
