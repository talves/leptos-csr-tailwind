use leptos::{ev::MouseEvent, *};

use crate::components::variants::base::ClassVariant;
use crate::OptionMaybeSignal;

#[component]
pub fn Button<F>(
    cx: Scope,
    on_click: F,
    #[prop(into, optional)] variant: OptionMaybeSignal<ClassVariant>,
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

#[component]
pub fn LinkButton(
    cx: Scope,
    #[prop(into, optional)] variant: OptionMaybeSignal<ClassVariant>,
    #[prop(into, optional)] disabled: OptionMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    href: &'static str,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <a
            id=id
            href=href // format!("{}", )
            class=format!("{} {}", variant.get(), class.get())
            style=style
            aria-disabled=move || disabled.get()
        >
            { children(cx) }
        </a>
    }
}
