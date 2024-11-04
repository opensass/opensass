use dioxus::prelude::*;
use crate::components::admin::sidebar::Tab;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarLinkProps {
    title: &'static str,
    tab: Tab,
    active_tab: Signal<Tab>,
}

#[component]
pub fn SidebarLink(mut props: SidebarLinkProps) -> Element {
    let is_active = (props.active_tab)() == props.tab;
    rsx! {
        a {
            class: if is_active { "text-lg p-2 rounded-lg bg-blue-600 text-white shadow-lg" } else { "text-lg p-2 rounded-lg text-gray-300 hover:bg-gray-700" },
            href: "#",
            onclick: move |_| props.active_tab.set(props.tab.clone()),
            "{props.title}"
        }
    }
}
