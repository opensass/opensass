use crate::components::team::social_links::SocialLinks;
use crate::components::team::SocialLink;

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TeamMember {
    pub name: &'static str,
    pub position: &'static str,
    pub image: Asset,
    pub link: &'static str,
    pub social_links: Vec<SocialLink>,
}

#[component]
pub fn TeamCard(member: TeamMember) -> Element {
    rsx! {
        div { class: "text-white group relative bg-gray-800 rounded-lg overflow-hidden transition-all duration-300 hover:-translate-y-2 shadow-lg transform hover:scale-105",
            div { class: "relative w-full h-64 overflow-hidden",
                img {
                    class: "absolute inset-0 w-full h-full object-cover opacity-70 transition-opacity duration-300 group-hover:opacity-100",
                    src: "{member.image}",
                    alt: "{member.name}",
                    loading: "lazy",

                }
                div { class: "absolute inset-0 bg-gradient-to-t from-black to-transparent opacity-50 transition-opacity duration-300 group-hover:opacity-30" }
            }
            div { class: "p-6 text-center",
                h5 { class: "text-2xl font-semibold text-white mt-4 transition-colors duration-300 group-hover:text-gray-200",
                    Link { to: "{member.link}", class: "hover:underline", "{member.name}" }
                }
                p { class: "text-gray-400 text-center mt-2", "{member.position}" }
                SocialLinks { social_links: member.social_links.clone() }
            }
        }
    }
}
