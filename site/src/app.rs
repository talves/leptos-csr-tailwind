use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_tw_ui::components::{
    buttons::button::LinkButton,
    menu::bars::{MenuBar, MenuHeader},
    theme::toggle::{MenuToggleButton, ThemeToggleButton},
    typography::{Typography, TypographyVariant},
};

use crate::{
    pages::{counter::Counter, home::Home},
    theme::{ButtonVariant, MenuBarVariant, MenuHeaderVariant},
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
                </Routes>
            </Router>
        </Layout>
    }
}

#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <Menu />
        {children(cx)}
    }
}

#[component]
fn Menu(cx: Scope) -> impl IntoView {
    let (active, set_active) = create_signal(cx, false);

    view! {cx,
        <MenuHeader variant={MenuHeaderVariant::Default.get()}>
            <MenuBar variant={MenuBarVariant::Default.get()}>
                    <div class="flex items-center justify-between">
                        <Typography variant=TypographyVariant::Span class="flex-none font-weight-20 text-3xl text-blue-800 dark:text-gray-200">Default Menu Bar</Typography>
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
                            <ThemeToggleButton class="text-yellow-500 dark:text-primary-400 focus:outline-none text-sm p-1"
                                icon_class="w-9 h-9 fill-orange-300 dark:fill-yellow-300 dark:hover:fill-gray-800 hover:bg-yellow-200 dark:hover:bg-yellow-300 rounded-2xl" />
                        </div>
                    </div>
            </MenuBar>
        </MenuHeader>
    }
}
