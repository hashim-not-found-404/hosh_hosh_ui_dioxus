//! The "Simple" style preset — the default cyberpunk look.
//!
//! Every component in this crate ships its own CSS. Mount
//! [`StyleInjector`] once near the app root; every styled
//! component works without any extra setup from the consumer.
//!
//! Colors are read from CSS custom properties, so a caller can
//! re-theme the whole library by setting `--hhs-brand`,
//! `--hhs-on-brand`, etc. on any ancestor element. Defaults live
//! in `:root` below.

use prelude::document;
use prelude::*;

// ─────────────────────────────────────────────────────────────
// Stylesheet
// ─────────────────────────────────────────────────────────────

/// All CSS shipped by `style_simple`. Mounted once via
/// [`StyleInjector`]; `document::Style` deduplicates by content
/// hash so calling it multiple times is safe.
pub const STYLES: &str = r#"
:root {
    --hhs-brand:        #6eafd2;
    --hhs-on-brand:     #161a22;
    --hhs-brand-alt:    #c89b64;
    --hhs-danger:       #cd6960;
    --hhs-success:      #7dbe91;
    --hhs-bg:           #0e1016;
    --hhs-surface:      #14171f;
    --hhs-surface-2:    #1c202a;
    --hhs-text:         #dce2ea;
    --hhs-text-muted:   #828a98;
    --hhs-border:       #343a48;
    --hhs-font-mono:    ui-monospace, "SFMono-Regular", "SF Mono", Menlo, Consolas, "Liberation Mono", monospace;
    --hhs-radius-sm:    4px;
    --hhs-radius-md:    8px;
}

/* ── Primary button ──────────────────────────────────────── */

.hhs-primary-button {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 40px;
    padding: 0 1.25rem;

    font-family: var(--hhs-font-mono);
    font-size: 0.875rem;
    font-weight: 500;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    white-space: nowrap;

    color: var(--hhs-on-brand);
    background: var(--hhs-brand);
    border: 1px solid var(--hhs-brand);
    border-radius: 0;

    /* Signature octagonal corners — cut top-left and bottom-right. */
    clip-path: polygon(
        10px 0,
        100% 0,
        100% calc(100% - 10px),
        calc(100% - 10px) 100%,
        0 100%,
        0 10px
    );

    cursor: pointer;
    user-select: none;
    transition:
        box-shadow 0.15s ease-out,
        transform 0.08s ease-out,
        filter 0.15s ease-out;
    -webkit-tap-highlight-color: transparent;
}

.hhs-primary-button:hover:not(:disabled) {
    box-shadow:
        0 0 10px var(--hhs-brand),
        0 0 24px rgb(110 175 210 / 0.45);
    filter: brightness(1.05);
    text-shadow:
        -0.5px 0 rgb(255 60 100 / 0.55),
         0.5px 0 rgb(40 220 255 / 0.55);
}

.hhs-primary-button:active:not(:disabled) {
    /* Brief glitch burst on press — matches the egui version. */
    animation: hhs-glitch-press 0.16s linear;
    box-shadow:
        0 0 6px var(--hhs-brand),
        0 0 16px rgb(110 175 210 / 0.30);
}

.hhs-primary-button:focus-visible {
    outline: 2px solid var(--hhs-brand);
    outline-offset: 3px;
}

.hhs-primary-button:disabled {
    opacity: 0.35;
    cursor: not-allowed;
    box-shadow: none;
    filter: none;
}

@keyframes hhs-glitch-press {
    0%   { transform: translate(0, 1px); }
    25%  { transform: translate(-1px, 1px); }
    50%  { transform: translate(1px, 1px); }
    75%  { transform: translate(-1px, 1px); }
    100% { transform: translate(0, 1px); }
}
"#;

/// Mount once at the app root. Idempotent.
#[component]
pub fn StyleInjector() -> Element {
    rsx! {
        document::Style { "{STYLES}" }
    }
}

// ─────────────────────────────────────────────────────────────
// PrimaryButton
// ─────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Props)]
pub struct PrimaryButtonProps {
    /// Text shown on the button.
    pub label: String,
    #[props(default = false)]
    pub disabled: bool,
    /// Click handler. Defaults to a no-op.
    #[props(default)]
    pub onclick: EventHandler<MouseEvent>,
    /// Optional extra CSS classes appended to the button.
    #[props(default)]
    pub class: Option<String>,
    /// Optional inline style override. Use sparingly — prefer
    /// overriding the CSS variables on an ancestor.
    #[props(default)]
    pub style: Option<String>,
}

/// Solid-fill primary action button.
///
/// Renders as `<button type="button">` with the `hhs-primary-button`
/// class. The visual is defined entirely by [`STYLES`] — call
/// [`StyleInjector`] once for the styles to apply.
///
/// ```ignore
/// PrimaryButton {
///     label: "SUBMIT",
///     onclick: move |_| do_something(),
/// }
/// ```
#[component]
pub fn PrimaryButton(props: PrimaryButtonProps) -> Element {
    let extra = props.class.clone().unwrap_or_default();
    let class = if extra.is_empty() {
        "hhs-primary-button".to_string()
    } else {
        format!("hhs-primary-button {extra}")
    };
    let style = props.style.clone().unwrap_or_default();

    rsx! {
        button {
            r#type: "button",
            class: "{class}",
            style: "{style}",
            disabled: props.disabled,
            onclick: move |e| props.onclick.call(e),
            "{props.label}"
        }
    }
}
