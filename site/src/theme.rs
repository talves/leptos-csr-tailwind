use leptos_tw_ui::components::variants::base::ClassVariant;

pub enum ButtonVariant {
    Solid,
    Outline,
    Ghost,
    Soft,
    White,
    Link,
}

impl ButtonVariant {
    pub fn get(&self) -> ClassVariant {
        const SOLID: &'static [&'static str] = &[
            "p-3",
            "justify-center",
            "items-center",
            "rounded-md",
            "border",
            "border-transparent",
            "font-semibold",
            "bg-blue-500",
            "text-white",
            "hover:bg-blue-600",
            "focus:outline-none",
            "focus:ring-2",
            "focus:ring-blue-500",
            "focus:ring-offset-2",
            "transition-all",
            "text-sm",
            "dark:focus:ring-offset-gray-800",
        ];
        match self {
            ButtonVariant::Solid => ClassVariant::Vec(SOLID),
            ButtonVariant::Outline => ClassVariant::Str("p-[.688rem] justify-center items-center rounded-md border-2 border-gray-200 font-semibold text-blue-500 hover:text-white hover:bg-blue-500 hover:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm dark:border-gray-700 dark:hover:border-blue-500"),
            ButtonVariant::Ghost => ClassVariant::Str("p-3 justify-center items-center rounded-md border border-transparent font-semibold text-blue-500 hover:bg-blue-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"),
            ButtonVariant::Soft => ClassVariant::Str("p-3 justify-center items-center rounded-md bg-blue-100 border border-transparent font-semibold text-blue-500 hover:text-white hover:bg-blue-500 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"),
            ButtonVariant::White => ClassVariant::Str("p-3 justify-center items-center rounded-md border font-medium bg-white text-gray-700 shadow-sm align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 transition-all text-sm dark:bg-slate-900 dark:hover:bg-slate-800 dark:border-gray-700 dark:text-gray-400 dark:hover:text-white dark:focus:ring-offset-gray-800"),
            ButtonVariant::Link => ClassVariant::Str("p-3 justify-center items-center rounded-md border border-transparent font-semibold text-blue-500 hover:text-blue-700 focus:outline-none focus:ring-2 ring-offset-white focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm"),
        }
    }
}

pub enum MenuHeaderVariant {
    Default,
}

impl MenuHeaderVariant {
    pub fn get(&self) -> ClassVariant {
        const DEFAULT: &'static [&'static str] = &[
            "flex",
            "flex-wrap",
            "sm:justify-start",
            "sm:flex-nowrap",
            "z-50",
            "w-full",
            "bg-white",
            "border-b",
            "text-sm",
            "py-2.5",
            "sm:py-4",
            "dark:bg-slate-900",
            "dark:border-gray-700",
        ];
        match self {
            MenuHeaderVariant::Default => ClassVariant::Vec(DEFAULT),
        }
    }
}

pub enum MenuBarVariant {
    Default,
    SectionRight,
    SectionLeft,
}

impl MenuBarVariant {
    pub fn get(&self) -> ClassVariant {
        const DEFAULT: &'static [&'static str] = &[
            "relative",
            "max-w-[85rem]",
            "w-full",
            "mx-auto",
            "px-4",
            "sm:flex",
            "sm:items-center",
            "sm:justify-between",
            "sm:px-6",
            "lg:px-8",
        ];
        const SECTION_R: &'static [&'static str] = &[
            "w-full",
            "flex",
            "items-center",
            "justify-end",
            "ml-auto",
            "sm:justify-between",
            "sm:gap-x-3",
            "sm:order-3",
        ];
        const SECTION_L: &'static [&'static str] = &[
            "w-full",
            "flex",
            "items-center",
            "justify-start",
            "mr-auto",
            "sm:gap-x-3",
            "sm:order-3",
        ];
        match self {
            MenuBarVariant::Default => ClassVariant::Vec(DEFAULT),
            MenuBarVariant::SectionRight => ClassVariant::Vec(SECTION_R),
            MenuBarVariant::SectionLeft => ClassVariant::Vec(SECTION_L),
        }
    }
}
