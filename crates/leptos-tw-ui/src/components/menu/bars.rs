use crate::{components::variants::base::ClassVariant, OptionMaybeSignal};
use leptos::*;

#[component]
pub fn MenuBar(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] variant: OptionMaybeSignal<ClassVariant>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <nav
            id=id
            class=format!("{} {}", variant.get(), class.get())
            style=style
        >
            { children(cx) }
        </nav>
    }
}

#[component]
pub fn MenuHeader(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] variant: OptionMaybeSignal<ClassVariant>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <header
            id=id
            class=format!("{} {}", variant.get(), class.get())
            style=style
        >
            { children(cx) }
        </header>
    }
}
