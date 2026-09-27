pub(crate) mod card;
pub(crate) mod social_links;

use crate::components::team::card::TeamCard;
use crate::components::team::card::TeamMember;
use crate::components::team::social_links::SocialLink;
use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn Team() -> Element {
    let theme_ctx = use_theme();
    let is_light = matches!((theme_ctx.theme)(), Theme::Light);
    let title_color = if is_light {
        "text-gray-800"
    } else {
        "text-white"
    };
    let subtitle_color = if is_light {
        "text-gray-500"
    } else {
        "text-gray-400"
    };
    let team_members = vec![TeamMember {
        name: "Mahmoud Harmouch",
        position: "Full Stack Rust Developer",
        image: asset!("/assets/team_1.webp"),
        link: "https://www.github.com/wiseaidev",
        social_links: vec![SocialLink {
            link: "https://www.github.com/wiseaidev",
            icon: rsx! {i {
                class: "text-2xl fa-brands fa-github",
            }},
        }],
    }];

    rsx! {
        section { id: "team", class: if is_light { "bg-gray-100 py-16 flex items-center justify-center min-h-screen" } else { "py-16 flex items-center justify-center min-h-screen" }, style: if is_light { "" } else { "background: var(--bg-secondary);" },
            div { class: "container mx-auto px-4",
                div { class: "flex flex-col items-center",
                    div { class: "w-full mb-12",
                        div { class: "text-center", "data-aos": "fade-up", "data-aos-duration": "800",
                            h2 { class: format!("text-5xl font-bold mb-4 tracking-wide {}", title_color), "Our Skilled Professionals" }
                            p { class: format!("text-lg {}", subtitle_color), "Dedicated innovators committed to driving your success forward." }
                        }
                    }
                    div { class: "w-full max-w-xs",
                        // TODO: set 2 cols on md, 3 cols on lg when team grows
                        div { class: "grid grid-cols-1 md:grid-cols-1 lg:grid-cols-1 gap-10",
                            for member in team_members.iter() {
                                TeamCard { member: member.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}
