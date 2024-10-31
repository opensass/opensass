use crate::components::navbar::dropdown::Dropdown;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
enum NavLink {
    HomePage,
    Features,
    Roadmap,
    Faq,
    Testimonial,
    Team,
    Blog,
}

#[component]
pub fn NavLinks() -> Element {
    let mut active_link = use_signal(|| NavLink::HomePage);

    let is_active = |link: &NavLink| {
        if active_link() == *link {
            "active-underline"
        } else {
            ""
        }
    };

    let nav_links = vec![
        (NavLink::HomePage, "#home", "Home"),
        (NavLink::Features, "#features", "Features"),
        (NavLink::Roadmap, "#roadmap", "Roadmap"),
        (NavLink::Faq, "#faq", "Faq"),
        (NavLink::Testimonial, "#testimonial", "Testimonial"),
        (NavLink::Team, "#team", "Team"),
        (NavLink::Blog, "#blog", "Blog"),
    ];

    rsx! {
        div { class: "flex flex-col md:flex-row gap-4 md:gap-8",
            for (link, href, label) in nav_links {
                Link {
                    to: href,
                    class: format!("text-black text-lg hover:decoration-gray-500 {}", is_active(&link)),
                    onclick: move |_| active_link.set(link.clone()),
                    "{label}"
                }
            }
            Dropdown {}
        }
    }
}
