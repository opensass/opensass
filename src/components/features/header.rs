use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div { class: "max-w-[600px] mb-20 bg-white justify-center text-center",
            h2 { class: "text-4xl md:text-5xl font-bold text-black leading-tight mt-4 mb-6",
                "Discover the Open SASS Community"
            }
            p { class: "text-lg text-black leading-relaxed mb-8",
                "Join a dynamic community of open-source Rust developers leveraging SASS to create and innovate together."
            }
        }
    }
}
