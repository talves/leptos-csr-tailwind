use leptos::*;
use leptos_meta::Title;
use leptos_tw_ui::components::{
    buttons::button::Button,
    container::Main,
    typography::{Typography, TypographyVariant},
};

use crate::theme::{ButtonVariant, TypographyClass};

#[component]
pub fn ExamplePage(cx: Scope) -> impl IntoView {
    view! {cx,
        <Title text="Leptos + Tailwindcss | Examples"/>
        <Main id="main" class="max-w-[85rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
            <div>
                //   <!-- Announcement Banner -->
                <Typography variant=TypographyVariant::H1 class={TypographyClass::H1.get()}>Examples with Explanations</Typography>
                //   <!-- End Announcement Banner -->
            </div>
            <ButtonSection/>
            <TypographySection/>
        </Main>
        // An empty Fragment is not created on the DOM
        {Fragment::new(vec![])}
    }
}

#[component]
fn ButtonSection(cx: Scope) -> impl IntoView {
    view! {cx,
        <section class="max-w-[70rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
            <Typography variant=TypographyVariant::H2 class={TypographyClass::H2.get()}>Buttons</Typography>
                <div class="border rounded-xl shadow-sm dark:bg-slate-800 dark:border-slate-600 p-5">
                    <div class="p-6 inline-block">
                        <Button class="ml-0 mx-1 bg-blue-400" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().unwrap())} variant={ButtonVariant::Solid.get()}>{"Solid"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Outline.get()}>{"Outline"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Ghost.get()}>{"Ghost"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Soft.get()}>{"Soft"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::White.get()}>{"White"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Link.get()}>{"Link"}</Button>
                    </div>
                    <div class="p-6 grid grid-cols-6 gap-4">
                        <Button class="ml-0 mx-1 bg-blue-400" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().unwrap())} variant={ButtonVariant::Solid.get()}>{"Solid"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Outline.get()}>{"Outline"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Ghost.get()}>{"Ghost"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Soft.get()}>{"Soft"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::White.get()}>{"White"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Link.get()}>{"Link"}</Button>
                    </div>
                    <div class="flex gap-2">
                        <Button class="ml-0 mx-1 bg-blue-400" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().unwrap())} variant={ButtonVariant::Solid.get()}>{"Solid"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Outline.get()}>{"Outline"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Ghost.get()}>{"Ghost"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Soft.get()}>{"Soft"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::White.get()}>{"White"}</Button>
                        <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonVariant::Link.get()}>{"Link"}</Button>
                    </div>
                </div>
        </section>
    }
}

#[component]
fn TypographySection(cx: Scope) -> impl IntoView {
    view! {cx,
        <section class="max-w-[70rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
            <Typography variant=TypographyVariant::H2 class={TypographyClass::H2.get()}>Typography</Typography>
        </section>
    }
}
