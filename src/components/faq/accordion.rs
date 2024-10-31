use crate::components::faq::item::AccordionItem;
use dioxus::prelude::*;

#[component]
pub fn Accordion() -> Element {
    let mut active_item = use_signal(|| Some(0));

    let questions_answers = vec![
        ("What is Open SASS?",
         "Open SASS is an open-source company providing plug-and-play Rust WebAssembly templates for popular frameworks like Dioxus, Yew, and Leptos. Our goal is to help developers easily build secure SaaS applications from scratch using only Rust; No JavaScript required."),
        ("How do Open SASS templates help in building SaaS apps?",
         "Open SASS templates streamline the process of creating SaaS applications by providing a secure, Rust-only foundation. With minimal setup, you can focus on customizing your app while avoiding JavaScript dependencies, ensuring performance and security."),
        ("Which frameworks are supported by Open SASS?",
         "We offer templates for various WebAssembly frameworks including Dioxus, Yew, and Leptos. This allows Rust developers to choose a template that best suits their project requirements and get started quickly."),
        ("Is it difficult to switch to Open SASS templates from a JavaScript stack?",
         "Not at all! Open SASS templates are designed to be developer-friendly, with clear documentation and modular components. You can seamlessly transition from JavaScript to Rust WebAssembly, enjoying Rust's memory safety and performance benefits."),
        ("How do Open SASS templates enhance app security?",
         "By using Rust, Open SASS templates provide inherent memory safety, preventing common vulnerabilities such as buffer overflows. Additionally, our templates are rigorously tested for secure WebAssembly integration, giving you a solid base for a secure SaaS application."),
        ("Is Open SASS free to use?",
         "Yes, Open SASS is completely open source and free to use. We believe in empowering developers by providing high-quality, secure templates that make it easy to build and deploy SaaS applications."),
        ("How can I contribute to Open SASS?",
         "We welcome contributions from the community! Whether you’re interested in adding features, fixing bugs, or improving documentation, head over to our GitHub repositories to get started. Your support helps make Open SASS better for everyone."),
        ("Can I customize Open SASS templates to fit my specific needs?",
         "Absolutely. Our templates are highly customizable, allowing you to tailor them to your specific project requirements. This flexibility means you can focus on building the unique aspects of your SaaS app, without reinventing the wheel."),
        ("What advantages does using Rust for SaaS development offer?",
         "Rust offers strong memory safety, exceptional performance, and a growing ecosystem for WebAssembly. By building with Rust and Open SASS, you’ll benefit from a highly efficient, secure, and modern foundation for your SaaS application."),
    ];

    rsx! {
        div { class: "flex flex-col space-y-4",
            for (index, (question, answer)) in questions_answers.iter().enumerate() {
                AccordionItem {
                    question: question,
                    answer: answer,
                    is_active: active_item() == Some(index),
                    onclick: move |_| active_item.set(Some(index)),
                }
            }
        }
    }
}
