use leptos::*;

#[component]
pub fn Container(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView
{
    view! {cx, 
        <div id=id class=class style=style>
            {children(cx)}
        </div>
    }
}

#[component]
pub fn ContainerFromProp<F, IV>(
    cx: Scope,
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_view: F,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView
{
    view! {cx, 
        <>{render_view()}</>
    }
}

#[component]
pub fn Main(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView
{
    view! {cx, 
        <main id=id role="main" class=class style=style>
            {children(cx)}
        </main>
    }
}
