pub(crate) mod card;
pub(crate) mod social_links;

use crate::components::team::card::TeamCard;
use crate::components::team::card::TeamMember;
use crate::components::team::social_links::SocialLink;
use dioxus::prelude::*;

#[component]
pub fn Team() -> Element {
    let team_members = vec![TeamMember {
        name: "Mahmoud Harmouch",
        position: "Full Stack Rust Developer",
        image: "/team_1.jpeg",
        link: "/teams/1",
        social_links: vec![
            SocialLink {
                icon: "./down_arrow.svg",
                link: "#",
            },
            SocialLink {
                icon: "./down_arrow.svg",
                link: "#",
            },
            SocialLink {
                icon: "./down_arrow.svg",
                link: "#",
            },
        ],
    }];

    rsx! {
        section { class: "py-16 bg-white flex items-center justify-center min-h-screen",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-col items-center",
                    div { class: "w-full mb-12",
                        div { class: "text-center", "data-aos": "fade-up", "data-aos-duration": "800",
                            h2 { class: "text-5xl font-bold text-gray-800 mb-4 tracking-wide", "Our Skilled Professionals" }
                            p { class: "text-gray-500 text-lg", "Dedicated innovators committed to driving your success forward." }
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
