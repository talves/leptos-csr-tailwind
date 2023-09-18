use crate::theme::ButtonVariant;
use leptos::*;
use leptos_meta::*;
use leptos_tw_ui::components::{
    buttons::button::Button,
    container::{Container, ContainerFromProp, Main},
    typography::*,
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {cx,
      <Title text="Leptos + Tailwindcss"/>
      <Main id="content">
          //   <!-- Announcement Banner -->
          <Container class="bg-gradient-to-br from-teal-200 via-violet-400 to-blue-500 dark:from-teal-700 dark:via-violet-800 dark:to-blue-900">
              <div class="max-w-[85rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto" class:min-h-screen=true>
              //   <!-- Grid -->
              <div class="grid justify-center md:grid-cols-2 md:justify-between md:items-center gap-2">
                  <div class="text-center md:text-left text-gray/[.8] dark:text-white/[.8]">
                  <Typography variant=TypographyVariant::H1 class="font-weight-20 text-3xl text-blue-800 dark:text-gray-200">Typography H1</Typography>
                  <p class="text-sm uppercase tracking-wider">
                      Preview
                  </p>
                  <ContainerFromProp render_view=|| view! { cx, <p class="mt-1 font-medium">
                      {"This is content from a render_view property"}
                  </p>} />
                  </div>
                  // <!-- End Col -->

                  <div class="mt-3 text-center md:text-left md:flex md:justify-end md:items-center">
                     <Typography variant=TypographyVariant::H1 class="font-weight-20 text-3xl text-blue-800 dark:text-gray-200">Column 2 on this Grid</Typography>
                  </div>
                  // <!-- End Col -->
                  </div>
                  //   <!-- End Grid -->
                  <Typography variant=TypographyVariant::H1 class="font-weight-20 text-3xl text-blue-800 dark:text-gray-200">Buttons</Typography>
                  <div class="border rounded-xl shadow-sm dark:bg-gray-800 dark:border-gray-700">
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
              </div>
          </Container>
      //   <!-- End Announcement Banner -->
      </Main>
      // An empty Fragment is not created on the DOM
      {Fragment::new(vec![])}
    }
}
