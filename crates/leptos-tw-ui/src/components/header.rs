use leptos::*;

#[component]
pub fn Header<F, IV>(
    cx: Scope,
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! { cx,
        <header>
        {render_prop()}
        {children(cx)}
        </header>
    }
}
