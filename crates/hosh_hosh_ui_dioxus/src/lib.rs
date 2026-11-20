pub use dioxus;
pub use dioxus::prelude::*;

macro_rules! export_style {
    ($name:ident) => {
        pub mod $name {
            pub use $name::PrimaryButton;
            pub use $name::SecondaryButton;
            pub use $name::StyleInjector;
            pub use $name::TertiaryButton;
        }
    };
}

export_style!(style_simple);
