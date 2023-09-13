use leptos::*;
use leptos_meta::*;
use crate::components::{container::{Container, Main, ContainerFromProp}, buttons::button::{ButtonClassVariant, Button}, theme::toggle::ThemeToggleButton};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // let (value, set_value) = create_signal(cx, 0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {cx,
        <Title text="Leptos + Tailwindcss"/>
        <Main id="content">
            //   <!-- Announcement Banner -->
            <Container class="bg-gradient-to-r from-red-500 via-purple-400 to-blue-500 ">
                <div class="max-w-[85rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto" class:min-h-screen=true>
                //   <!-- Grid -->
                <div class="grid justify-center md:grid-cols-2 md:justify-between md:items-center gap-2">
                    <div class="text-center md:text-left">
                    <p class="text-sm text-white/[.8] uppercase tracking-wider">
                        Preview
                    </p>
                    <ContainerFromProp render_view=|| view! { cx, <p class="mt-1 text-white font-medium">
                        {"This is content from a render_view property"}
                    </p>} />
                    </div>
                    // <!-- End Col -->

                    <div class="mt-3 text-center md:text-left md:flex md:justify-end md:items-center">
                    <a class="py-3 px-6 inline-flex justify-center items-center gap-2 rounded-full font-medium bg-white text-gray-700 shadow-sm align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 transition-all text-sm" href="/counter">
                        Click for Counter
                    </a>
                    </div>
                    // <!-- End Col -->
                    <div class="border rounded-xl shadow-sm p-6 dark:bg-gray-800 dark:border-gray-700">
                        <div class="inline-block">
                            <Button class="ml-0 mx-1 bg-blue-400" on_click={|e| println!("{}", e.to_string())}>{"Default (Unstyled)"}</Button>
                            <Button class="ml-0 mx-1" on_click={|e| web_sys::console::log_1(&e.target().expect("missing target"))} variant={ButtonClassVariant::Solid}>{"Solid"}</Button>
                            <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonClassVariant::Outline}>{"Outline"}</Button>
                            <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonClassVariant::Ghost}>{"Ghost"}</Button>
                            <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonClassVariant::Soft}>{"Soft"}</Button>
                            <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonClassVariant::White}>{"White"}</Button>
                            <Button class="ml-0 mx-1" on_click={|e| println!("{}", e.to_string())} variant={ButtonClassVariant::Link}>{"Link"}</Button>
                            <ThemeToggleButton class="ml-0 mx-1 dark:text-gray-300">{"Toggle Light/Dark"}</ThemeToggleButton>
                        </div>

                    </div>
                </div>
                //   <!-- End Grid -->
                </div>
            </Container>
        //   <!-- End Announcement Banner -->
        </Main>
        // An empty Fragment is not created on the DOM
        {Fragment::new(vec![])}
      }

}
