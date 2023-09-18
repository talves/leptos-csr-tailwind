use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_tw_ui::components::{
    buttons::button::LinkButton,
    container::Container,
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
    view! {cx,
        <MenuHeader variant={MenuHeaderVariant::Default.get()}>
            <MenuBar variant={MenuBarVariant::Default.get()}>
                <Container class={MenuBarVariant::SectionRight.get().to_string()}>
                    <div class="mr-5 md:mr-8">
                    <Typography variant=TypographyVariant::Span class="font-weight-20 text-3xl text-blue-800 dark:text-gray-200">Default Menu Bar</Typography>
                    </div>
                    <div class="flex flex-row items-center gap-2">
                        <LinkButton href="/" variant={ButtonVariant::Ghost.get()}>
                            Home
                        </LinkButton>
                        <LinkButton href="/counter" variant={ButtonVariant::Ghost.get()}>
                            Counter
                        </LinkButton>
                        <ThemeToggleButton class="mx-5 text-yellow-500 dark:text-primary-400 focus:outline-none text-sm p-1"
                            icon_class="w-9 h-9 fill-orange-300 dark:fill-yellow-300 dark:hover:fill-gray-800 hover:bg-yellow-200 dark:hover:bg-yellow-300 rounded-2xl" />
                        <MenuToggleButton class="visible md:invisible focus:outline-none p-1"
                            icon_class="w-7 h-7 fill-gray-700 hover:fill-gray-500 dark:fill-gray-300 dark:hover:fill-gray-100 dark:hover:outline rounded-2xl" />
                    </div>
                </Container>
            </MenuBar>
        </MenuHeader>
    }
}
