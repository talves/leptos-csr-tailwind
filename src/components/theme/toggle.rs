use leptos::*;
use crate::{components::{buttons::button::Button, icons::{IconSun, IconMoon}}, OptionMaybeSignal};
use wasm_bindgen::JsCast;


#[component]
pub fn ThemeToggleButton(
    cx: Scope,
    #[prop(into, optional)] disabled: OptionMaybeSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: OptionMaybeSignal<String>,
    #[prop(into, optional)] icon_class: OptionMaybeSignal<String>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    // children: Children,
) -> impl IntoView {
    let (theme, set_theme) = create_signal(cx, theme_mode(false));
    let is_light = Signal::derive(cx, move || theme.get() == "light");

    fn theme_mode(toggle: bool) -> &'static str {
        // Resolves theme_mode to a splice light|dark for default light
        fn resolve_toggle(theme_mode: String, toggle: bool) -> &'static str {
            let theme_mode = if theme_mode != "dark" {"light"} else {"dark"};
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
        let body = document.body().expect("document expect to have have a body").dyn_into::<web_sys::HtmlBodyElement>().unwrap();
        let local_storage = window.local_storage().expect("No access to local storage");
        let mut stored_theme_mode= match local_storage {
            Some(storage) => match storage.get("theme-mode") {
                Ok(value) => {
                    let new_theme_mode: &str = match value {
                        Some(theme_mode) => {
                            resolve_toggle(theme_mode, toggle)
                        },
                        // No theme-mode found in local storage
                        None => {
                            // Check for class name on body
                            resolve_toggle(body.class_name(), toggle)
                        },
                    };
                    let _ = storage.set("theme-mode", new_theme_mode);
                    new_theme_mode
                },
                Err(_) => "",
            },
            None => "",
        };
        if stored_theme_mode == "" {
            stored_theme_mode = resolve_toggle(body.class_name(), toggle);
            // There was an error accessing local storage, so use the body class name
            body.set_class_name(stored_theme_mode);
        } else {
            // Stored theme mode has been resolved and set in local storage, so set the class name
            body.set_class_name(stored_theme_mode);
        }
        stored_theme_mode
        
        // web_sys::console::log_1(&body); 
    }
    
    let light_class = icon_class.clone();
    let dark_class = icon_class.clone();

    view! { cx,
        <Button
            id=id.unwrap_or(Box::new(""))
            class=class
            style=style.unwrap_or(Box::new(""))
            on_click={move |_e| {set_theme(theme_mode(true));}}
            disabled=disabled
        >
            <Show when=move || is_light.get() fallback=move |cx| view! {cx, <IconMoon class=dark_class.get() /> }>
                <IconSun class=light_class.get() />
            </Show>
        </Button>
    }
}