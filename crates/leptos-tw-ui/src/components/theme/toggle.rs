use crate::{
    components::{
        buttons::button::Button,
        icons::{IconMenuOff, IconMenuOn, IconMoon, IconSun},
    },
    OptionMaybeSignal,
};
use leptos::*;
use wasm_bindgen::JsCast;

/// A function to return light/dark based on system pref or local storage (theme-mode)
pub fn theme_mode(toggle: bool) -> &'static str {
    // Resolves theme_mode to a splice light|dark for default light
    fn resolve_toggle(
        theme_mode: String,
        toggle: bool,
        system_dark_preferred: bool,
    ) -> &'static str {
        let theme_mode = if theme_mode == "" && !toggle {
            if system_dark_preferred {
                "dark".to_string()
            } else {
                "light".to_string()
            }
        } else {
            theme_mode
        };
        let theme_mode = if theme_mode != "dark" {
            "light"
        } else {
            "dark"
        };
        if !&toggle {
            // Resolve to the splice value
            theme_mode
        } else {
            // Toggle the theme mode
            if theme_mode == "dark" {
                "light"
            } else {
                "dark"
            }
        }
    }
    // Accessing the Dom and toggle/set the class name on body!
    let window = web_sys::window().expect("missing global window");
    let document = window.document().expect("expected document on window");
    let body = document
        .body()
        .expect("document expect to have have a body")
        .dyn_into::<web_sys::HtmlBodyElement>()
        .unwrap();
    let local_storage = window.local_storage().expect("No access to local storage");
    let media = window
        .match_media("(prefers-color-scheme: dark)")
        .expect("No access to media query");
    let system_dark_preferred = match media {
        Some(query) => query.matches(),
        None => false,
    };
    // web_sys::console::log_1(&wasm_bindgen::JsValue::from(&system_dark_preferred.to_string()));
    let mut stored_theme_mode = match local_storage {
        Some(storage) => match storage.get("theme-mode") {
            Ok(value) => {
                let new_theme_mode: &str = match value {
                    Some(theme_mode) => resolve_toggle(theme_mode, toggle, false),
                    // No theme-mode found in local storage
                    None => {
                        // Check for class name on body
                        resolve_toggle(body.class_name(), toggle, system_dark_preferred)
                    }
                };
                let _ = storage.set("theme-mode", new_theme_mode);
                new_theme_mode
            }
            Err(_) => "",
        },
        None => "",
    };
    if stored_theme_mode == "" {
        stored_theme_mode = resolve_toggle(body.class_name(), toggle, system_dark_preferred);
        // There was an error accessing local storage, so use the body class name
        body.set_class_name(stored_theme_mode);
    } else {
        // Stored theme mode has been resolved and set in local storage, so set the class name
        body.set_class_name(stored_theme_mode);
    }
    stored_theme_mode

    // web_sys::console::log_1(&body);
}

#[component]
pub fn ThemeToggleButton<F>(
    cx: Scope,
    #[prop(into, optional)] disabled: OptionMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] icon_class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    /// Required: Fn(bool) -> &'static str
    /// * (use leptos_tw_ui::components::theme::toggle::theme_mode)
    /// * Or write your own function
    mode_fn: F,
) -> impl IntoView
where
    // Requires a function that takes a boolean value for toggle or just value
    F: Fn(bool) -> &'static str + 'static,
{
    let (theme, set_theme) = create_signal(cx, mode_fn(false));
    let is_light = Signal::derive(cx, move || theme.get() == "light");

    let light_class = icon_class.clone();
    let dark_class = icon_class.clone();

    view! { cx,
        <Button
            id=id.unwrap_or(Box::new(""))
            class=class
            style=style.unwrap_or(Box::new(""))
            on_click={move |_e| {set_theme(mode_fn(true));}}
            disabled=disabled
        >
            <Show when=move || is_light.get() fallback=move |cx| view! {cx, <IconMoon class=dark_class.get() /> }>
                <IconSun class=light_class.get() />
            </Show>
        </Button>
    }
}

pub struct ThemeToggleSwitchClass {
    pub wrapper: &'static str,
    pub switch: &'static str,
    pub bar: &'static str,
    pub sun_fill: &'static str,
    pub moon_fill: &'static str,
    pub sun: &'static str,
    pub moon: &'static str,
}

#[component]
pub fn ThemeToggleSwitch<F>(
    cx: Scope,
    #[prop(into, optional)] disabled: OptionMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    class: ThemeToggleSwitchClass,
    /// Required: Fn(bool) -> &'static str
    /// * (use leptos_tw_ui::components::theme::toggle::theme_mode)
    /// * Or write your own function
    mode_fn: F,
) -> impl IntoView
where
    F: Fn(bool) -> &'static str + 'static,
{
    view! { cx,
        <Button
            id=id.unwrap_or(Box::new(""))
            class=class.wrapper
            style=style.unwrap_or(Box::new(""))
            on_click={move |_e| {mode_fn(true);}}
            disabled=disabled
        >
            <div class=class.bar /><span class="sr-only">Switch theme</span>
            <span aria-hidden="true" class=class.switch>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" class=class.moon>
                        <path class=class.moon_fill
                            d="M7.914 0a6.874 6.874 0 00-1.26 3.972c0 3.875 3.213 7.017 7.178 7.017.943 0 1.843-.178 2.668-.5C15.423 13.688 12.34 16 8.704 16 4.174 16 .5 12.41.5 7.982.5 3.814 3.754.389 7.914 0z"
                            fill-rule="evenodd"></path>
                    </svg>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" class=class.sun>
                        <path class=class.sun_fill
                            d="M3.828 5.243L2.343 3.757a1 1 0 011.414-1.414l1.486 1.485a5.027 5.027 0 00-1.415 1.415zM7 3.1V1a1 1 0 112 0v2.1a5.023 5.023 0 00-2 0zm3.757.728l1.486-1.485a1 1 0 111.414 1.414l-1.485 1.486a5.027 5.027 0 00-1.415-1.415zM12.9 7H15a1 1 0 010 2h-2.1a5.023 5.023 0 000-2zm-.728 3.757l1.485 1.486a1 1 0 11-1.414 1.414l-1.486-1.485a5.027 5.027 0 001.415-1.415zM9 12.9V15a1 1 0 01-2 0v-2.1a5.023 5.023 0 002 0zm-3.757-.728l-1.486 1.485a1 1 0 01-1.414-1.414l1.485-1.486a5.027 5.027 0 001.415 1.415zM3.1 9H1a1 1 0 110-2h2.1a5.023 5.023 0 000 2zM8 11a3 3 0 110-6 3 3 0 010 6z"
                            fill-rule="evenodd"></path>
                    </svg>
            </span>
        </Button>
    }
}

#[component]
pub fn MenuToggleButton<F>(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] disabled: OptionMaybeSignal<bool>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] icon_class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(into, optional)] active: Option<bool>,
    on_change: F,
    // children: Children,
) -> impl IntoView
where
    F: Fn(bool) + 'static,
{
    let (on_off, set_on_off) = create_signal(cx, active.unwrap_or(false));

    let light_class = icon_class.clone();
    let dark_class = icon_class.clone();

    view! { cx,
        <Button
            id=id.unwrap_or(Box::new(""))
            class=class
            style=style.unwrap_or(Box::new(""))
            on_click={move |_e| {set_on_off(!on_off.get());on_change(on_off.get());}}
            disabled=disabled
        >
            <Show when={on_off} fallback=move |cx| view! {cx, <IconMenuOff class=dark_class.get() /> }>
                <IconMenuOn class=light_class.get() />
            </Show>
        </Button>
    }
}

// <button
//     class="relative inline-flex h-[24px] w-[34px] shrink-0 cursor-pointer border-2 border-transparent focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75">
//     <div
//         class="bg-gray-200/60 dark:bg-white/10 rounded-full absolute left-0 right-0 h-[0.65rem] top-1/2 translate-y-[-50%]">
//     </div><span class="sr-only">Switch theme</span><span aria-hidden="true"
//         class="translate-x-0 dark:translate-x-2.5 shadow shadow-gray-700/10 border border-gray-200 bg-background-light dark:border-primary dark:bg-background-dark p-[3px] pointer-events-none inline-block h-5 w-5 transform rounded-full ring-0 transition-transform duration-200 ease-in-out"><svg
//             xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" class="dark:hidden">
//             <path class="fill-yellow-500"
//                 d="M3.828 5.243L2.343 3.757a1 1 0 011.414-1.414l1.486 1.485a5.027 5.027 0 00-1.415 1.415zM7 3.1V1a1 1 0 112 0v2.1a5.023 5.023 0 00-2 0zm3.757.728l1.486-1.485a1 1 0 111.414 1.414l-1.485 1.486a5.027 5.027 0 00-1.415-1.415zM12.9 7H15a1 1 0 010 2h-2.1a5.023 5.023 0 000-2zm-.728 3.757l1.485 1.486a1 1 0 11-1.414 1.414l-1.486-1.485a5.027 5.027 0 001.415-1.415zM9 12.9V15a1 1 0 01-2 0v-2.1a5.023 5.023 0 002 0zm-3.757-.728l-1.486 1.485a1 1 0 01-1.414-1.414l1.485-1.486a5.027 5.027 0 001.415 1.415zM3.1 9H1a1 1 0 110-2h2.1a5.023 5.023 0 000 2zM8 11a3 3 0 110-6 3 3 0 010 6z"
//                 fill-rule="evenodd"></path>
//         </svg><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" class="hidden dark:block">
//             <path class="fill-primary"
//                 d="M7.914 0a6.874 6.874 0 00-1.26 3.972c0 3.875 3.213 7.017 7.178 7.017.943 0 1.843-.178 2.668-.5C15.423 13.688 12.34 16 8.704 16 4.174 16 .5 12.41.5 7.982.5 3.814 3.754.389 7.914 0z"
//                 fill-rule="evenodd"></path>
//         </svg></span>
// </button>
