use leptos::*;

use super::variants::base::ClassVariant;

pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Paragraph,
    SpanInline,
    Span { inline: bool },
    CodeInline,
    Code { inline: bool },
}

#[component]
pub fn Typography(
    cx: Scope,
    variant: TypographyVariant,
    #[prop(into, optional)] id: Option<AttributeValue>,
    /// Implement trait TypographyClass for TypographyClassVariant
    #[prop(into, optional)]
    class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    match variant {
        TypographyVariant::H1 => H1(
            cx,
            H1Props {
                id,
                class,
                style,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H2 => H2(
            cx,
            H2Props {
                id,
                class,
                style,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H3 => H3(
            cx,
            H3Props {
                id,
                class,
                style,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H4 => H4(
            cx,
            H4Props {
                id,
                class,
                style,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H5 => H5(
            cx,
            H5Props {
                id,
                class,
                style,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H6 => H6(
            cx,
            H6Props {
                id,
                class,
                style,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::Paragraph => P(
            cx,
            PProps {
                id,
                class,
                style,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::Span { inline } => Span(
            cx,
            SpanProps {
                id,
                class,
                style,
                inline: Some(inline),
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::SpanInline => Span(
            cx,
            SpanProps {
                id,
                class,
                style,
                inline: Some(true),
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::Code { inline } => Code(
            cx,
            CodeProps {
                id,
                class,
                style,
                inline: Some(inline),
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::CodeInline => Code(
            cx,
            CodeProps {
                id,
                class,
                style,
                inline: Some(true),
                children,
            },
        )
        .into_view(cx),
    }
}

#[component]
pub fn H1(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <h1 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children(cx)}
        </h1>
    }
}

#[component]
pub fn H2(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <h2 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children(cx)}
        </h2>
    }
}

#[component]
pub fn H3(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <h3 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children(cx)}
        </h3>
    }
}

#[component]
pub fn H4(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <h4 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children(cx)}
        </h4>
    }
}

#[component]
pub fn H5(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <h5 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children(cx)}
        </h5>
    }
}

#[component]
pub fn H6(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <h6 id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children(cx)}
        </h6>
    }
}

#[component]
pub fn P(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <p id=id class=class.unwrap_or(ClassVariant::Unstyled).to_string() style=style>
            {children(cx)}
        </p>
    }
}

#[component]
pub fn Span(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(optional)] inline: Option<bool>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <span id=id class=format!{"{} {}", class.unwrap_or(ClassVariant::Unstyled).to_string(), {if inline.is_some() {if inline.unwrap() {"inline-flex inline"} else {"block"}} else {"block"}}} style=style>
            {children(cx)}
        </span>
    }
}

#[component]
pub fn Code(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<ClassVariant>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(optional)] inline: Option<bool>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <code id=id class=format!{"{} {}", class.unwrap_or(ClassVariant::Unstyled).to_string(), {if inline.is_some() {if inline.unwrap() {"inline-flex inline"} else {"block"}} else {"block"}}} style=style >
            {children(cx)}
        </code>
    }
}
