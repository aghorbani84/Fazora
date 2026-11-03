//! Button primitive.

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Surface,
    Ghost,
    Danger,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Surface)]
    variant: ButtonVariant,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    loading: bool,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    style: Option<String>,
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    label: Option<String>,
    #[props(default)]
    #[props(default)]
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let class = match props.variant {
        ButtonVariant::Primary => "fz-button fz-button-primary",
        ButtonVariant::Surface => "fz-button fz-button-surface",
        ButtonVariant::Ghost => "fz-button fz-button-ghost",
        ButtonVariant::Danger => "fz-button fz-button-danger",
    };

    let full_class = match &props.class {
        Some(c) => format!("{} {}", class, c),
        None => class.to_string(),
    };

    let disabled = props.disabled || props.loading;

    rsx! {
        button {
            class: "{full_class}",
            style: if let Some(s) = &props.style { s } else { "" },
            disabled: disabled,
            aria_disabled: disabled,
            onclick: move |evt| {
                if !disabled {
                    props.onclick.call(evt);
                }
            },
            if props.loading {
                span { class: "fz-spinner", aria_hidden: "true" }
            } else if let Some(label) = &props.label {
                {label.as_str()}
            } else {
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct IconButtonProps {
    #[props(default = false)]
    active: bool,
    #[props(default = false)]
    disabled: bool,
    #[props(default)]
    title: Option<String>,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    #[props(default)]
    children: Element,
}

#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
    let base = "fz-button fz-button-icon fz-button-surface";
    let class = match (&props.active, &props.class) {
        (true, Some(c)) => format!("{} {} fz-button-primary", base, c),
        (true, None) => format!("{} fz-button-primary", base),
        (false, Some(c)) => format!("{} {}", base, c),
        (false, None) => base.to_string(),
    };

    rsx! {
        button {
            class: "{class}",
            disabled: props.disabled,
            aria_disabled: props.disabled,
            title: props.title.as_deref().unwrap_or(""),
            onclick: move |evt| {
                if !props.disabled {
                    props.onclick.call(evt);
                }
            },
            {props.children}
        }
    }
}
