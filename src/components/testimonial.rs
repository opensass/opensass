pub(crate) mod author;
pub(crate) mod card;
pub(crate) mod rating;

use crate::components::common::header::Header;
use crate::components::testimonial::card::TestimonialCard;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TestimonialData {
    quote: &'static str,
    author_name: &'static str,
    author_title: &'static str,
    author_image: Asset,
    company_logo: Asset,
    star_images: Vec<Element>,
}

#[component]
pub fn Testimonial() -> Element {
    let testimonials = vec![
        TestimonialData {
            quote: "I thought writing software in Rust was hard until I found Open SASS. Now I can build apps faster than I can compile! Bravo!",
            author_name: "Linus Torvalds",
            author_title: "Creator of Linux",
            author_image: asset!("/assets/linus.webp"),
            company_logo: asset!("/assets/linux.webp"),
            star_images: vec![rsx! {i {
                width: 30,
                height: 30,
                class: "fa-solid fa-star",
            }}; 5],
        },
        TestimonialData {
            quote: "I tried to jailbreak my own SaaS framework but ended up creating a black hole. Open SASS actually makes it easy to launch a product!",
            author_name: "George Hotz",
            author_title: "Hacker Extraordinaire",
            author_image: asset!("/assets/geohotz.webp"),
            company_logo: asset!("/assets/comma.webp"),
            star_images: vec![rsx! {i {
                width: 30,
                height: 30,
                class: "fa-solid fa-star",
            }}; 5],
        },
        TestimonialData {
            quote: "I’ve launched rockets and electric cars, but nothing has been as easy as building with Open SASS. Rust is now my first favorite language!",
            author_name: "Elon Musk",
            author_title: "CEO, SpaceX & Tesla",
            author_image: asset!("/assets/elon.webp"),
            company_logo: asset!("/assets/spacex.webp"),
            star_images: vec![rsx! {i {
                width: 30,
                height: 30,
                class: "fa-solid fa-star",
            }}; 5],
        },
    ];

    let mut current_index = use_signal(|| 0);
    let testimonial_len = testimonials.len();

    let mut go_left = move || {
        current_index.set(if current_index() == 0 {
            testimonial_len - 1
        } else {
            current_index() - 1
        });
    };

    let mut go_right = move || {
        current_index.set((current_index() + 1) % testimonial_len);
    };

    rsx! {
        section { id: "testimonial", class: "flex flex-col items-center justify-center min-h-screen space-x-4 mt-10",
            Header {
                title: "What Our Users Are Saying",
                subtitle: "Discover how Open Sass has transformed the way developers connect and create, through the voices of our vibrant community."
            }
            div { class: "flex items-center space-x-2",
                button { onclick: move |_| go_left(), "⬅️" }
                TestimonialCard { testimonial: testimonials[current_index()].clone() }
                button { onclick: move |_| go_right(), "➡️" }
            }
        }
    }
}
