use leptos::{ev::MouseEvent, *};
use std::fmt::{Display, Formatter};

use crate::components::variants::base::ClassVariant;
use crate::OptionMaybeSignal;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonClassVariant {
    #[default]
    Unstyled,
    Str(&'static str),
    Vec(&'static Vec<&'static str>),
}

impl ClassVariant for ButtonClassVariant {
    fn as_vec(&self) -> Vec<&'static str> {
        match self {
            ButtonClassVariant::Unstyled => Vec::<&'static str>::new(), // "".split(" ").collect::<Vec<&str>>(),
            ButtonClassVariant::Str(x) => x.split(" ").collect::<Vec<&str>>(),
            ButtonClassVariant::Vec(x) => x.to_vec(),
        }
    }
}

impl Display for ButtonClassVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&ClassVariant::to_string(self))
    }
}

#[component]
pub fn Button<F>(
    cx: Scope,
    on_click: F,
    #[prop(into, optional)] variant: OptionMaybeSignal<ButtonClassVariant>,
    #[prop(into, optional)] disabled: OptionMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { cx,
        <button
            id=id
            class=format!("{} {}", variant.get(), class.get())
            style=style
            aria-disabled=move || disabled.get()
            on:click=move |e| {
                if !disabled.get_untracked() {
                    e.stop_propagation();
                    on_click(e);
                }
            }
        >
            { children(cx) }
        </button>
    }
}
