use crate::theme::TypographyClass;
use leptos::*;
use leptos_meta::*;
use leptos_tw_ui::components::{
    container::{ContainerFromProp, Main},
    typography::*,
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {cx,
      <Title text="Leptos + Tailwindcss"/>
      <Main id="content" class="max-w-[85rem] px-4 py-4 sm:px-6 lg:px-8 mx-auto">
              <div class="">
              //   <!-- Grid -->
              <div class="grid justify-center md:grid-cols-2 md:justify-between md:items-center gap-2">
                  <div class="text-center md:text-left text-gray/[.8] dark:text-white/[.8]">
                  <Typography variant=TypographyVariant::H1 class={TypographyClass::H1.get()}>Leptos Site Using Tailwindcss</Typography>
                  <p class="text-sm uppercase tracking-wider">
                      Preview
                  </p>
                  <ContainerFromProp render_view=|| view! { cx, <p class="mt-1 font-medium">
                      {"This is content from a render_view property"}
                  </p>} />
                  </div>
                  // <!-- End Col -->

                  <div class="mt-3 text-center md:text-left md:flex md:justify-end md:items-center">
                     <Typography variant=TypographyVariant::H1 class={TypographyClass::H1.get()}>Column 2 on this Grid</Typography>
                  </div>
                  // <!-- End Col -->
                  </div>
                  //   <!-- End Grid -->
              </div>
      </Main>
      // An empty Fragment is not created on the DOM
      {Fragment::new(vec![])}
    }
}
