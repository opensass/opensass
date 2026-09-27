use crate::components::navbar::dropdown::Dropdown;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[derive(PartialEq, Clone)]
enum NavLink {
    HomePage,
    Features,
    Projects,
    Roadmap,
    Faq,
    Testimonial,
    Team,
    Blog,
}

#[component]
pub fn NavLinks() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let text_color = if is_light {
        "text-gray-900"
    } else {
        "text-gray-100"
    };
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
        (NavLink::Projects, "#projects", "Projects"),
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
                    class: format!("text-lg hover:decoration-gray-500 {} {}", is_active(&link), text_color),
                    onclick: move |_| active_link.set(link.clone()),
                    "{label}"
                }
            }
            Dropdown {}
        }
    }
}
