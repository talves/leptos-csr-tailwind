use leptos::{ev::MouseEvent, *};
use std::fmt::{Display, Formatter};

use crate::components::variants::base::ClassVariant;
use crate::OptionMaybeSignal;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonClassVariant {
    #[default]
    Unstyled,
    Solid,
    Outline,
    Ghost,
    Soft,
    White,
    Link,
}

// TODO: convert class str values to a css variable then resolve through a theme provider instead
impl ClassVariant for ButtonClassVariant {
    fn as_vec(&self) -> Vec<&'static str> {
        match self {
            ButtonClassVariant::Unstyled => "".split(" ").collect::<Vec<&str>>(),
            ButtonClassVariant::Solid => "py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md border border-transparent font-semibold bg-blue-500 text-white hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm dark:focus:ring-offset-gray-800".split(" ").collect::<Vec<&str>>(),
            ButtonClassVariant::Outline => "py-[.688rem] px-4 inline-flex justify-center items-center gap-2 rounded-md border-2 border-gray-200 font-semibold text-blue-500 hover:text-white hover:bg-blue-500 hover:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm dark:border-gray-700 dark:hover:border-blue-500".split(" ").collect::<Vec<&str>>(),
            ButtonClassVariant::Ghost => "py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md border border-transparent font-semibold text-blue-500 hover:bg-blue-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm".split(" ").collect::<Vec<&str>>(),
            ButtonClassVariant::Soft => "py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md bg-blue-100 border border-transparent font-semibold text-blue-500 hover:text-white hover:bg-blue-500 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm".split(" ").collect::<Vec<&str>>(),
            ButtonClassVariant::White => "py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md border font-medium bg-white text-gray-700 shadow-sm align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 transition-all text-sm dark:bg-slate-900 dark:hover:bg-slate-800 dark:border-gray-700 dark:text-gray-400 dark:hover:text-white dark:focus:ring-offset-gray-800".split(" ").collect::<Vec<&str>>(),
            ButtonClassVariant::Link => "py-3 px-4 inline-flex justify-center items-center gap-2 rounded-md border border-transparent font-semibold text-blue-500 hover:text-blue-700 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm".split(" ").collect::<Vec<&str>>(),
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
    // #[prop(into, optional)] color: OptionMaybeSignal<ButtonColor>,
    // #[prop(into, optional)] size: OptionMaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: OptionMaybeSignal<bool>,
    // #[prop(into, optional)] active: OptionMaybeSignal<bool>,
    // #[prop(into, optional)] variations: OptionMaybeSignal<View>,
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
            // class=class.get().to_string() // Same as format!{"{}", class.get()}
            class=format!("{} {}", variant.get(), class.get())
            // class:active=move || active.get()
            // data-variant=move || variant.get().to_string()
            // data-color=move || color.get().as_str()
            // data-size=move || size.get().as_str()
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
