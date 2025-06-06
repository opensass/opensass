use crate::components::admin::link::SidebarLink;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Tab {
    Subscriptions,
}

#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps {
    active_tab: Signal<Tab>,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    rsx! {
        div { class: "flex flex-col w-1/4 border-r min-h-screen p-6 bg-gray-800 shadow-md",
            h2 { class: "text-2xl font-bold mb-6 text-white", "Admin" },
            nav { class: "flex flex-col space-y-4",
                SidebarLink { title: "Subscriptions", tab: Tab::Subscriptions, active_tab: props.active_tab }
            }
        }
    }
}
