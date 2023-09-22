use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_tw_ui::components::{
    buttons::button::LinkButton,
    menu::bars::{MenuBar, MenuHeader},
    theme::toggle::{theme_mode, MenuToggleButton, ThemeToggleSwitch},
};

use crate::{
    pages::{counter::Counter, example::ExamplePage, home::Home},
    theme::{
        default_page_class, ButtonVariant, MenuBarVariant, MenuHeaderVariant,
        ToggleSwitchClassVariant,
    },
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {cx,
        <Layout>
            // <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
            <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Router>
                <Routes>
                    <Route path="/" view=move |cx| view! {cx, <Home/> }/>
                    <Route path="/counter" view=move |cx| view! {cx, <Counter/> }/>
                    <Route path="/example" view=move |cx| view! {cx, <ExamplePage/> }/>
                </Routes>
            </Router>
        </Layout>
    }
}

#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <LayoutWrapper>
            <Menu />
            {children(cx)}
            <Footer />
        </LayoutWrapper>
    }
}

#[component]
fn LayoutWrapper(cx: Scope, children: Children) -> impl IntoView {
    let default_class = default_page_class();

    view! {cx,
        <div class=default_class.wrapper class:min-h-screen=true>
        {children(cx)}
        </div>
    }
}

#[component]
fn Menu(cx: Scope) -> impl IntoView {
    let (active, set_active) = create_signal(cx, false);
    log!("loading Menu");

    view! {cx,
        <MenuHeader variant={MenuHeaderVariant::Default.get()}>
            <MenuBar variant={MenuBarVariant::Default.get()}>
                    <div class="flex items-center justify-between">
                        <span class="flex-none font-weight-20 text-3xl text-blue-800 dark:text-gray-200">Default Menu Bar</span>
                        <div class="sm:hidden">
                            <MenuToggleButton on_change=set_active class="focus:outline-none p-1"
                                icon_class="w-7 h-7 fill-gray-700 hover:fill-gray-500 dark:fill-gray-300 dark:hover:fill-gray-100 dark:hover:outline rounded-2xl" />
                        </div>
                    </div>
                    <div class={move || match active(){ true => "transition-all duration-300 basis-full grow sm:block", false => "hidden transition-all duration-300 basis-full grow sm:block"}}>
                        <div class="flex flex-col gap-5 mt-5 sm:flex-row sm:items-center sm:justify-end sm:mt-0 sm:pl-5">
                            <LinkButton href="/" variant={ButtonVariant::Ghost.get()}>
                                Home
                            </LinkButton>
                            <LinkButton href="/counter" variant={ButtonVariant::Ghost.get()}>
                                Counter
                            </LinkButton>
                            <LinkButton href="/example" variant={ButtonVariant::Ghost.get()}>
                                Examples
                            </LinkButton>
                            <ThemeToggleSwitch mode_fn={theme_mode} class={ToggleSwitchClassVariant::Encased.get()} />
                        </div>
                    </div>
            </MenuBar>
        </MenuHeader>
    }
}

// Sticky footer
#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! {cx,
        <footer class="fixed bottom-0 left-0 z-20 w-full p-4 bg-white border-t border-gray-200 shadow md:flex md:items-center md:justify-between md:p-6 dark:bg-slate-900 dark:border-gray-600">
            <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">{"© 2023 "} <a href="https://example.com/" class="hover:underline">{"Footer™"}</a>. All Rights Reserved.
            </span>
            <ul class="flex flex-wrap items-center mt-3 text-sm font-medium text-gray-500 dark:text-gray-400 sm:mt-0">
                <li>
                    <LinkButton href="#" class="mr-4 hover:underline md:mr-6">
                        About
                    </LinkButton>
                </li>
                <li>
                    <LinkButton href="#" class="mr-4 hover:underline md:mr-6">
                        Privacy Policy
                    </LinkButton>
                </li>
                <li>
                    <LinkButton href="#" class="mr-4 hover:underline md:mr-6">
                        Licensing
                    </LinkButton>
                </li>
                <li>
                    <LinkButton href="#" class="mr-4 hover:underline md:mr-6">
                       Contact
                    </LinkButton>

                </li>
            </ul>
        </footer>
    }
}
