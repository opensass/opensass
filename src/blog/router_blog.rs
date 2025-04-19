#[derive(
    Clone,
    Copy,
    dioxus_router::prelude::Routable,
    PartialEq,
    Eq,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BookRoute {
    #[route("/announcing-opensass")]
    AnnouncingOpensass {},
    #[route("/introducing-x-ai")]
    IntroducingXAi {},
    #[route("/aibook-v002-release")]
    AibookV002Release {},
    #[route("/tripper-v001-release")]
    TripperV001Release {},
    #[route("/beyond-typeScript")]
    BeyondTypeScript {},
}
impl BookRoute {
    pub fn sections(&self) -> &'static [use_mdbook::mdbook_shared::Section] {
        &self.page().sections
    }
    pub fn page(&self) -> &'static use_mdbook::mdbook_shared::Page<Self> {
        LAZY_BOOK.get_page(self)
    }
    pub fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        match self {
            BookRoute::AnnouncingOpensass {} => use_mdbook::mdbook_shared::PageId(0usize),
            BookRoute::IntroducingXAi {} => use_mdbook::mdbook_shared::PageId(1usize),
            BookRoute::AibookV002Release {} => use_mdbook::mdbook_shared::PageId(2usize),
            BookRoute::TripperV001Release {} => use_mdbook::mdbook_shared::PageId(3usize),
            BookRoute::BeyondTypeScript {} => use_mdbook::mdbook_shared::PageId(4usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::IntroducingXAi {}
    }
}
pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> =
    use_mdbook::Lazy::new(|| {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        pages
            .push((
                0usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 1 |---| Announcing Open SASS 🚀 |---| announcement |---| announcing-opensass |---| Nov 10 2024 |---| Welcome to Open SASS. Your open-source platform for building the future of SaaS with Rust and Wasm. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
                            .to_string(),
                        url: BookRoute::AnnouncingOpensass {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚨 Announcing Open SASS 🚨".to_string(),
                                id: "🚨-announcing-open-sass-🚨".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Welcome to Open SASS".to_string(),
                                id: "welcome-to-open-sass".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Makes Open SASS Special?".to_string(),
                                id: "what-makes-open-sass-special?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Introducing AIBOOK".to_string(),
                                id: "introducing-aibook".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Open Source and Transparent".to_string(),
                                id: "open-source-and-transparent".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Get Started".to_string(),
                                id: "get-started".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(0usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::AnnouncingOpensass {},
            ::use_mdbook::mdbook_shared::PageId(0usize),
        );
        pages
            .push((
                1usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 2 |---| ✨ Introducing X-AI |---| announcement |---| introducing-x-ai |---| Nov 18 2024 |---| Today, we are excited to announce the release of 𝕏-AI, your gateway to the X-AI API in Rust. |---| https://github.com/user-attachments/assets/e18b9fc2-7b7d-4125-86fe-c1b91fdb0f93"
                            .to_string(),
                        url: BookRoute::IntroducingXAi {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "𝕏-AI Rust Crate".to_string(),
                                id: "𝕏-ai-rust-crate".to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "✨ Welcome".to_string(),
                                id: "✨-welcome".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📦 Features".to_string(),
                                id: "📦-features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🛠\u{fe0f} Installation".to_string(),
                                id: "🛠\u{fe0f}-installation".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "💻 Usage Examples".to_string(),
                                id: "💻-usage-examples".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "1. Fetch API Key Information 🔑".to_string(),
                                id: "1.-fetch-api-key-information-🔑".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "2. Create Chat Completions 💬".to_string(),
                                id: "2.-create-chat-completions-💬".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🤝 Join Our Community".to_string(),
                                id: "🤝-join-our-community".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📚 Explore More".to_string(),
                                id: "📚-explore-more".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(1usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::IntroducingXAi {},
            ::use_mdbook::mdbook_shared::PageId(1usize),
        );
        pages
            .push((
                2usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 3 |---| 💡 AIBook v0.0.2 Release |---| announcement |---| aibook-v002-release |---| Nov 22 2024 |---| Welcome back to our blog! We are excited to announce the release of aibook. |---| https://github.com/user-attachments/assets/ec4e080f-37af-4e62-af40-f0bb92d28bff"
                            .to_string(),
                        url: BookRoute::AibookV002Release {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 AIBook v0.0.2 Release - Stripe Integration"
                                    .to_string(),
                                id: "🚀-aibook-v0.0.2-release---stripe-integration"
                                    .to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔗 Quick Links".to_string(),
                                id: "🔗-quick-links".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🆕 What's New in v0.0.2".to_string(),
                                id: "🆕-what's-new-in-v0.0.2".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📖 Why Stripe?".to_string(),
                                id: "📖-why-stripe?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚙\u{fe0f} Why This Matters".to_string(),
                                id: "⚙\u{fe0f}-why-this-matters".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🌍 Explore More".to_string(),
                                id: "🌍-explore-more".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(2usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::AibookV002Release {},
            ::use_mdbook::mdbook_shared::PageId(2usize),
        );
        pages
            .push((
                3usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 4 |---| Introducing Tripper ✈\u{fe0f} |---| announcement |---| tripper-v001-release |---| Nov 28 2024 |---| Welcome back to our blog! We are excited to announce the release of aibook. |---| https://github.com/user-attachments/assets/d18cb450-f4c7-4455-a9c2-b0f165889487"
                            .to_string(),
                        url: BookRoute::TripperV001Release {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "✈\u{fe0f} Introducing Tripper: Revolutionizing Travel Planning"
                                    .to_string(),
                                id: "✈\u{fe0f}-introducing-tripper:-revolutionizing-travel-planning"
                                    .to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔗 Quick Links".to_string(),
                                id: "🔗-quick-links".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🆕 What is Tripper?".to_string(),
                                id: "🆕-what-is-tripper?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "✨ Key Features".to_string(),
                                id: "✨-key-features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🌍 Why Tripper Matters".to_string(),
                                id: "🌍-why-tripper-matters".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🌎 Explore More".to_string(),
                                id: "🌎-explore-more".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(3usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::TripperV001Release {},
            ::use_mdbook::mdbook_shared::PageId(3usize),
        );
        pages
            .push((
                4usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 5 |---| Beyond TypeScript |---| blog |---| beyond-typeScript |---| Apr 20 2025 |---| Hey devs, and anyone still dealing with a 900MB node_modules folder. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
                            .to_string(),
                        url: BookRoute::BeyondTypeScript {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "1. Rust Doesn't Need ORMs Because It ".to_string(),
                                id: "1.-rust-doesn't-need-orms-because-it".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "2. Rust's Type System Actually ".to_string(),
                                id: "2.-rust's-type-system-actually".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "3. Rust Does ".to_string(),
                                id: "3.-rust-does".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "4. Rust's Compiler ".to_string(),
                                id: "4.-rust's-compiler".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "5. Memory Safety in Rust Isn't Optional. In TypeScript, You Can Leak Like a Faucet."
                                    .to_string(),
                                id: "5.-memory-safety-in-rust-isn't-optional.-in-typescript,-you-can-leak-like-a-faucet."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "6. Rust's Tooling Works Out-of-the-Box. TypeScript Devs Live in Dependency Hell."
                                    .to_string(),
                                id: "6.-rust's-tooling-works-out-of-the-box.-typescript-devs-live-in-dependency-hell."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Want to format your code? Deal!".to_string(),
                                id: "want-to-format-your-code?-deal!".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Want to lint it? Gotcha!".to_string(),
                                id: "want-to-lint-it?-gotcha!".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Want to build it? Cool!".to_string(),
                                id: "want-to-build-it?-cool!".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Want to run tests? Awesome!".to_string(),
                                id: "want-to-run-tests?-awesome!".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "7. Rust's Error Handling is Real. TypeScript Just Throws and Hopes."
                                    .to_string(),
                                id: "7.-rust's-error-handling-is-real.-typescript-just-throws-and-hopes."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "8. Rust's Enums Are Algebraic Data Types. TypeScript's Unions Are Just Strings with Hats."
                                    .to_string(),
                                id: "8.-rust's-enums-are-algebraic-data-types.-typescript's-unions-are-just-strings-with-hats."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "9. Rust's Traits Enable Powerful, Generic Code. TypeScript Has Duck Typing and Hopes."
                                    .to_string(),
                                id: "9.-rust's-traits-enable-powerful,-generic-code.-typescript-has-duck-typing-and-hopes."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "10. Rust Has WebAssembly Muscle. TypeScript Just Compiles to More JS."
                                    .to_string(),
                                id: "10.-rust-has-webassembly-muscle.-typescript-just-compiles-to-more-js."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "11. Rust Has a Strong, Growing Ecosystem. TypeScript Is Still Tied to JavaScript's Chaos."
                                    .to_string(),
                                id: "11.-rust-has-a-strong,-growing-ecosystem.-typescript-is-still-tied-to-javascript's-chaos."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Final Word, for now...".to_string(),
                                id: "final-word,-for-now...".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(4usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::BeyondTypeScript {},
            ::use_mdbook::mdbook_shared::PageId(4usize),
        );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 1 |---| Announcing Open SASS 🚀 |---| announcement |---| announcing-opensass |---| Nov 10 2024 |---| Welcome to Open SASS. Your open-source platform for building the future of SaaS with Rust and Wasm. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
                            .to_string(),
                        location: Some(BookRoute::AnnouncingOpensass {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 2 |---| ✨ Introducing X-AI |---| announcement |---| introducing-x-ai |---| Nov 18 2024 |---| Today, we are excited to announce the release of 𝕏-AI, your gateway to the X-AI API in Rust. |---| https://github.com/user-attachments/assets/e18b9fc2-7b7d-4125-86fe-c1b91fdb0f93"
                            .to_string(),
                        location: Some(BookRoute::IntroducingXAi {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 3 |---| 💡 AIBook v0.0.2 Release |---| announcement |---| aibook-v002-release |---| Nov 22 2024 |---| Welcome back to our blog! We are excited to announce the release of aibook. |---| https://github.com/user-attachments/assets/ec4e080f-37af-4e62-af40-f0bb92d28bff"
                            .to_string(),
                        location: Some(BookRoute::AibookV002Release {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 4 |---| Introducing Tripper ✈\u{fe0f} |---| announcement |---| tripper-v001-release |---| Nov 28 2024 |---| Welcome back to our blog! We are excited to announce the release of aibook. |---| https://github.com/user-attachments/assets/d18cb450-f4c7-4455-a9c2-b0f165889487"
                            .to_string(),
                        location: Some(BookRoute::TripperV001Release {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 5 |---| Beyond TypeScript |---| blog |---| beyond-typeScript |---| Apr 20 2025 |---| Hey devs, and anyone still dealing with a 900MB node_modules folder. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
                            .to_string(),
                        location: Some(BookRoute::BeyondTypeScript {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32]),
                        ),
                        nested_items: vec![],
                    }),
                ],
                suffix_chapters: vec![],
            },
            pages: pages.into_iter().collect(),
            page_id_mapping,
        }
    });
#[component(no_case_check)]
pub fn AnnouncingOpensass() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "-announcing-open-sass-",
            a { href: "#-announcing-open-sass-", class: "header", "🚨 Announcing Open SASS 🚨" }
        }
        p {
            "Welcome to Open SASS. Your open-source platform for building the future of SaaS with Rust and Wasm."
        }
        h2 { id: "welcome-to-open-sass",
            a { href: "#welcome-to-open-sass", class: "header", "Welcome to Open SASS" }
        }
        p {
            "We're excited to introduce "
            strong { "Open SASS" }
            ", a community-driven platform for developers who love "
            strong { "Rust" }
            " and "
            strong { "Wasm" }
            ". Whether you're using "
            em { "Dioxus" }
            ", "
            em { "Leptos" }
            ", or "
            em { "Yew" }
            ", Open SASS makes it seamless to create fast, secure, and scalable SaaS applications."
        }
        h2 { id: "what-makes-open-sass-special",
            a { href: "#what-makes-open-sass-special", class: "header",
                "What Makes Open SASS Special?"
            }
        }
        ul {
            li {
                strong { "🌐 Smooth Wasm Integration:" }
                " Easily integrate with your favorite Wasm frameworks for unbeatable speed and reliability."
            }
            li {
                strong { "📚 Ready-to-Use Templates:" }
                " Save time with our library of open-source templates tailored for Rust web projects."
            }
            li {
                strong { "🛠\u{fe0f} Customizable Components:" }
                " Fully adjustable components to suit your needs."
            }
            li {
                strong { "🤝 Built for the Community:" }
                " Collaborate with a vibrant community of Rust developers."
            }
            li {
                strong { "📖 Clear Documentation:" }
                " Straightforward guides to help you every step of the way."
            }
        }
        h2 { id: "introducing-aibook",
            a { href: "#introducing-aibook", class: "header", "Introducing AIBOOK" }
        }
        p {
            "Alongside Open SASS, we're also launching "
            strong { "AIBOOK" }
            ", our first SaaS application powered by AI. AIBOOK makes content generation effortless:"
        }
        ul {
            li {
                strong { "🌍 Multilingual Content:" }
                " Generate content in any language to connect with a global audience."
            }
            li {
                strong { "🤖 Powered by AI:" }
                " Create high-quality content in seconds."
            }
            li {
                strong { "🦀 Built with Rust:" }
                " Enjoy unmatched security and reliability."
            }
            li {
                strong { "📊 Advanced Analytics:" }
                " Track performance with an intuitive dashboard."
            }
        }
        h2 { id: "open-source-and-transparent",
            a { href: "#open-source-and-transparent", class: "header",
                "Open Source and Transparent"
            }
        }
        p {
            "Open SASS is "
            strong { "100% open-source" }
            ". All templates are written in Rust, and our transparent crypto funding ensures you know exactly how we're growing."
        }
        h2 { id: "get-started",
            a { href: "#get-started", class: "header", "Get Started" }
        }
        p {
            a { href: "https://opensass.org", "Visit Open SASS" }
            " | "
            a { href: "https://discord.gg/b5JbvHW5nv", "Join Our Discord" }
        }
        p {
            em { "© 2025 Open SASS | Built with ❤\u{fe0f} by and for the Rust community." }
        }
    }
}
#[component(no_case_check)]
pub fn IntroducingXAi() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "-ai-rust-crate",
            a { href: "#-ai-rust-crate", class: "header", "𝕏-AI Rust Crate" }
        }
        p {
            "Welcome to our blog! Today, we are excited to announce the release of "
            strong { "𝕏-AI" }
            ", your gateway to the X-AI API in Rust."
        }
        h2 { id: "-welcome",
            a { href: "#-welcome", class: "header", "✨ Welcome" }
        }
        p {
            "At "
            strong { "Open SASS" }
            ", we are passionate about making SaaS and AI tools accessible to Rust developers."
            strong { "x-ai" }
            " Rust crate is a major step forward in empowering the Rust community with cutting-edge AI capabilities."
        }
        p {
            "Whether you're building conversational agents, generating creative content, or working with embeddings for data analysis, "
            strong { "x-ai" }
            " has you covered."
        }
        p {
            "Our team has designed this crate with performance, safety, and ease of use in mind. By leveraging Rust's strict typing and memory safety guarantees, "
            strong { "x-ai" }
            " ensures your code is robust and thread-safe from the ground up."
        }
        h2 { id: "-features",
            a { href: "#-features", class: "header", "📦 Features" }
        }
        p {
            "The "
            strong { "x-ai" }
            " crate provides a wide range of capabilities for interacting with the X-AI API:"
        }
        ul {
            li {
                strong { "Fetch API Key Information" }
                ": Verify and retrieve your API key details seamlessly."
            }
            li {
                strong { "Chat Completions" }
                ": Create interactive chatbots and conversational agents."
            }
            li {
                strong { "Text Completions" }
                ": Generate rich, human-like text for creative and functional use cases."
            }
            li {
                strong { "Embedding Creation" }
                ": Generate embeddings to analyze textual similarity and semantic relationships."
            }
            li {
                strong { "Fetch Model Information" }
                ": Gain insights into the available AI models and their features."
            }
            li {
                strong { "List Models" }
                ": Explore all supported models for various tasks."
            }
        }
        h2 { id: "-installation",
            a { href: "#-installation", class: "header", "🛠\u{fe0f} Installation" }
        }
        p {
            "To get started with "
            strong { "x-ai" }
            ", add the following to your "
            code { "Cargo.toml" }
            ":"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">[dependencies]\n</span><span style=\"color:#f8f8f2;\">x_ai </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;0.0.1&quot;\n</span><span style=\"color:#f8f8f2;\">tokio </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ version </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;1.41.1&quot;</span><span style=\"color:#f8f8f2;\">, features </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;full&quot;</span><span style=\"color:#f8f8f2;\">] }}</span></pre>\n" }
        h2 { id: "-usage-examples",
            a { href: "#-usage-examples", class: "header", "💻 Usage Examples" }
        }
        h3 { id: "1-fetch-api-key-information-",
            a { href: "#1-fetch-api-key-information-", class: "header",
                "1. Fetch API Key Information 🔑"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::env;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">x_ai::api_key::ApiKeyRequestBuilder;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">x_ai::client::XaiClient;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">x_ai::traits::{{ApiKeyFetcher, ClientConfig}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[tokio::main]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> client </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">XaiClient::builder()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">build</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Failed to build XaiClient&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    client.</span><span style=\"color:#66d9ef;\">set_api_key</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">        env::var(</span><span style=\"color:#ffee99;\">&quot;XAI_API_KEY&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;XAI_API_KEY must be set!&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> request_builder </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">ApiKeyRequestBuilder::new(client);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> result </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> request_builder.</span><span style=\"color:#66d9ef;\">fetch_api_key_info</span><span style=\"color:#f8f8f2;\">().await;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> result {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(info) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;API Key ID: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, info.api_key_id),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">eprintln!(</span><span style=\"color:#ffee99;\">&quot;Error: {{:?}}&quot;</span><span style=\"color:#f8f8f2;\">, err),\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h3 { id: "2-create-chat-completions-",
            a { href: "#2-create-chat-completions-", class: "header",
                "2. Create Chat Completions 💬"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">std::env;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">x_ai::chat_compl::{{ChatCompletionsRequestBuilder, Message}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">x_ai::client::XaiClient;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">x_ai::traits::{{ChatCompletionsFetcher, ClientConfig}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[tokio::main]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> client </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">XaiClient::builder()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">build</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Failed to build XaiClient&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    client.</span><span style=\"color:#66d9ef;\">set_api_key</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">        env::var(</span><span style=\"color:#ffee99;\">&quot;XAI_API_KEY&quot;</span><span style=\"color:#f8f8f2;\">).</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;XAI_API_KEY must be set!&quot;</span><span style=\"color:#f8f8f2;\">)\n</span><span style=\"color:#f8f8f2;\">    );\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> messages </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">vec![\n</span><span style=\"color:#f8f8f2;\">        Message {{ role: </span><span style=\"color:#ffee99;\">&quot;system&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(), content: </span><span style=\"color:#ffee99;\">&quot;You are a helpful assistant.&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">() }},\n</span><span style=\"color:#f8f8f2;\">        Message {{ role: </span><span style=\"color:#ffee99;\">&quot;user&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(), content: </span><span style=\"color:#ffee99;\">&quot;What is Rust?&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">() }},\n</span><span style=\"color:#f8f8f2;\">    ];\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> builder </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">ChatCompletionsRequestBuilder::new(client.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ffee99;\">&quot;model-id&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(), messages);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> response </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> builder.</span><span style=\"color:#66d9ef;\">create_chat_completion</span><span style=\"color:#f8f8f2;\">(builder.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">build</span><span style=\"color:#f8f8f2;\">().</span><span style=\"color:#66d9ef;\">unwrap</span><span style=\"color:#f8f8f2;\">()).await;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> response {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(result) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Response: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, result.choices[</span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">].message.content),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">eprintln!(</span><span style=\"color:#ffee99;\">&quot;Error: {{:?}}&quot;</span><span style=\"color:#f8f8f2;\">, err),\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "-join-our-community",
            a { href: "#-join-our-community", class: "header", "🤝 Join Our Community" }
        }
        p {
            "We're excited to promote a vibrant community of developers around this project. Be sure to join "
            a { href: "https://discord.gg/b5JbvHW5nv", "our community on Discord" }
            ", where you can ask questions, share ideas, and collaborate with fellow Rustaceans 🦀."
        }
        h2 { id: "-explore-more",
            a { href: "#-explore-more", class: "header", "📚 Explore More" }
        }
        ul {
            li {
                a { href: "https://crates.io/crates/x-ai", "Crate on Crates.io" }
            }
            li {
                a { href: "https://github.com/opensass/x-ai", "GitHub Repository" }
            }
            li {
                a { href: "https://discord.gg/b5JbvHW5nv", "Join the Conversation on Discord" }
            }
        }
        p {
            em { "© 2025 Open SASS | Built with ❤\u{fe0f} by and for the Rust community." }
        }
    }
}
#[component(no_case_check)]
pub fn AibookV002Release() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "-aibook-v002-release---stripe-integration",
            a {
                href: "#-aibook-v002-release---stripe-integration",
                class: "header",
                "🚀 AIBook v0.0.2 Release - Stripe Integration"
            }
        }
        p {
            "Welcome back to our blog! We are excited to announce the release of "
            strong { "AIBook v0.0.2" }
            ", now with "
            strong { "Stripe payment integration!" }
            " 🎉"
        }
        h2 { id: "-quick-links",
            a { href: "#-quick-links", class: "header", "🔗 Quick Links" }
        }
        ul {
            li {
                a { href: "https://aibook-8syx.onrender.com/#pricing",
                    strong { "Try AIBook v0.0.2 Now" }
                }
            }
            li {
                a { href: "https://github.com/opensass/aibook",
                    strong { "GitHub Repository" }
                }
            }
            li {
                a { href: "https://discord.gg/b5JbvHW5nv",
                    strong { "Join Our Community on Discord" }
                }
            }
        }
        h2 { id: "-whats-new-in-v002",
            a { href: "#-whats-new-in-v002", class: "header", "🆕 What's New in v0.0.2" }
        }
        ul {
            li {
                strong { "Stripe Payments" }
                ": Easily integrate Stripe into your Rust apps to accept payments for your AI features."
            }
            li {
                strong { "Perfect for SaaS Projects" }
                ": Monetize your AI-driven apps with just a few lines of code!"
            }
        }
        h2 { id: "-why-stripe",
            a { href: "#-why-stripe", class: "header", "📖 Why Stripe?" }
        }
        p {
            "Stripe is trusted by millions of businesses for secure, scalable, and flexible payment processing."
            strong { "Rust" }
            " apps like "
            strong { "AIBook" }
            "."
        }
        h2 { id: "-why-this-matters",
            a { href: "#-why-this-matters", class: "header", "⚙\u{fe0f} Why This Matters" }
        }
        p {
            "Rust is rapidly becoming the "
            strong { "go-to language" }
            " for modern SaaS, secure apps, and AI-driven projects."
            strong { "Open SASS" }
            ", we're empowering "
            strong { "Rust developers" }
            " to create robust, monetizable tools with ease."
        }
        h2 { id: "-explore-more",
            a { href: "#-explore-more", class: "header", "🌍 Explore More" }
        }
        p { "Stay connected and explore more:" }
        ul {
            li {
                a { href: "https://aibook-8syx.onrender.com/#pricing",
                    strong { "Pricing Page" }
                }
            }
            li {
                a { href: "https://github.com/opensass/aibook",
                    strong { "GitHub Repo" }
                }
            }
            li {
                a { href: "https://discord.gg/b5JbvHW5nv",
                    strong { "Join Our Discord Community" }
                }
            }
        }
        p {
            em { "© 2024 Open SASS | Built with ❤\u{fe0f} by and for the Rust community." }
        }
    }
}
#[component(no_case_check)]
pub fn TripperV001Release() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h1 { id: "-introducing-tripper-revolutionizing-travel-planning",
            a {
                href: "#-introducing-tripper-revolutionizing-travel-planning",
                class: "header",
                "✈\u{fe0f} Introducing Tripper: Revolutionizing Travel Planning"
            }
        }
        p { "Your Smart Travel Companion for Effortless Trip Planning" }
        h2 { id: "-quick-links",
            a { href: "#-quick-links", class: "header", "🔗 Quick Links" }
        }
        ul {
            li {
                a { href: "https://opensass.org/tripper",
                    strong { "Explore Tripper" }
                }
            }
            li {
                a { href: "https://github.com/opensass/tripper",
                    strong { "GitHub Repository" }
                }
            }
            li {
                a { href: "https://discord.gg/b5JbvHW5nv",
                    strong { "Join Our Community on Discord" }
                }
            }
        }
        h2 { id: "-what-is-tripper",
            a { href: "#-what-is-tripper", class: "header", "🆕 What is Tripper?" }
        }
        p {
            strong { "Tripper" }
            " is a groundbreaking platform that takes the hassle out of travel planning."
            strong { "intelligent features" }
            " and "
            strong { "sleek design" }
            ", it ensures that every journey is "
            strong { "personalized, organized, and inspiring" }
            "."
        }
        h2 { id: "-key-features",
            a { href: "#-key-features", class: "header", "✨ Key Features" }
        }
        ul {
            li {
                strong { "AI-Powered Itineraries" }
                ": Automatically generate detailed plans tailored to your preferences."
            }
            li {
                strong { "Stunning Visuals" }
                ": Explore destinations with breathtaking images powered by the "
                strong { "Unsplash API" }
                "."
            }
            li {
                strong { "Secure and Fast" }
                ": Built with cutting-edge technologies like "
                strong { "Rust, Dioxus, and AWS Bedrock" }
                " for a seamless experience."
            }
            li {
                strong { "Community Collaboration (WIP)" }
                ": Share and discover ideas with a vibrant community of travel enthusiasts."
            }
        }
        h2 { id: "-why-tripper-matters",
            a { href: "#-why-tripper-matters", class: "header", "🌍 Why Tripper Matters" }
        }
        p {
            "In a world where "
            strong { "travel planning can be overwhelming" }
            ", Tripper provides a "
            strong { "stress-free, AI-enhanced" }
            " experience."
            strong { "joy of exploration rather than logistics" }
            "."
        }
        p {
            "Whether you're a "
            strong { "solo traveler" }
            " or "
            strong { "planning with friends and family" }
            ", "
            strong { "Tripper makes it easy to create unforgettable adventures" }
            "."
        }
        h2 { id: "-explore-more",
            a { href: "#-explore-more", class: "header", "🌎 Explore More" }
        }
        p { "Stay connected and explore more:" }
        ul {
            li {
                a { href: "https://opensass.org/tripper",
                    strong { "Visit Tripper" }
                }
            }
            li {
                a { href: "https://github.com/opensass/tripper",
                    strong { "GitHub Repo" }
                }
            }
            li {
                a { href: "https://discord.gg/b5JbvHW5nv",
                    strong { "Join Our Community on Discord" }
                }
            }
        }
        p {
            em { "© 2024 Open SASS | Built with ❤\u{fe0f} by and for travel enthusiasts." }
        }
    }
}
#[component(no_case_check)]
pub fn BeyondTypeScript() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p {
            "Hey devs, and anyone still dealing with a 900MB  "
            code { "node_modules" }
            " folder,"
        }
        p {
            "We're constantly being told that JavaScript and TypeScript are the \"standard\" for web development. \"Everyone uses it\", they say. \"It's easy\", they whisper. But you know what else was popular? Internet Explorer."
        }
        p {
            "Today, we break the matrix. We're not going for what's easy. We're going for what's right. And Rust, my friends, is the real deal. We're about to explore why Rust outshines TypeScript on every level."
        }
        p { "Let's cook." }
        h2 { id: "1-rust-doesnt-need-orms-because-it",
            a { href: "#1-rust-doesnt-need-orms-because-it", class: "header",
                "1. Rust Doesn't Need ORMs Because It "
            }
            em { "Is" }
            " the Data Model"
        }
        p {
            "Let's talk databases. In TypeScript, if you want to interact with a database safely and cleanly, you're often forced to use an ORM like "
            a { href: "https://www.prisma.io/", "Prisma" }
            " ("
            a { href: "https://www.prisma.io/blog/from-rust-to-typescript-a-new-chapter-for-prisma-orm",
                "They dropped Rust for TypeScript. We're not mad, just disappointed."
            }
            "), or "
            a { href: "https://typeorm.io", "TypeORM" }
            ". These ORMs generate types based on your schema, or vice versa. Sounds good in theory. But in reality, you're juggling SQL migrations, type generation, inconsistent runtime checks, and you're praying to the gods of "
            code { "npx" }
            " every time you need to change a column name."
        }
        p {
            "Rust? You define a  "
            code { "struct" }
            ", and that's your schema. Your  "
            code { "struct" }
            " is strongly typed, compile-checked, and used directly with your query code. You don't need a third-party tool to tell you what a \"User\" is. You write the shape once, and it's the source of truth. No scaffolding. No runtime surprises."
        }
        p { "Let's explore an example using MongoDB in Rust:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">bson::{{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">chrono::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">serde::{{Deserialize, Serialize}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[allow(non_snake_case)]\n</span><span style=\"color:#f8f8f2;\">#[derive(Debug, Deserialize, Serialize, Clone)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">User {{\n</span><span style=\"color:#f8f8f2;\">    #[serde(rename = &quot;_id&quot;)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">id: ObjectId,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">name: String,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">email: String,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">password: String,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">role: Role,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">photo: String,\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">verified: </span><span style=\"font-style:italic;color:#66d9ef;\">bool</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    #[serde(with = &quot;chrono_datetime_as_bson_datetime&quot;, rename = &quot;createdAt&quot;)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">created_at: DateTime&lt;Utc&gt;,\n</span><span style=\"color:#f8f8f2;\">    #[serde(with = &quot;chrono_datetime_as_bson_datetime&quot;, rename = &quot;updatedAt&quot;)]\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">pub </span><span style=\"color:#f8f8f2;\">updated_at: DateTime&lt;Utc&gt;,\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This isn't just a struct; This is your database document schema, your gatekeeper, and your compiler-approved contract with the data layer. Every field is mapped directly, every date is serialized exactly how Mongo expects it, and Rust ensures you don't mess up a single byte."
        }
        p { "Want to insert a user? Here's how clean and safe it looks:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> new_user </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> User {{\n</span><span style=\"color:#f8f8f2;\">    id: ObjectId::new(),\n</span><span style=\"color:#f8f8f2;\">    name: </span><span style=\"color:#ffee99;\">&quot;OpenSASS&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">    email: </span><span style=\"color:#ffee99;\">&quot;oss@opensass.org&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">    password: </span><span style=\"color:#ffee99;\">&quot;hashed_password&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">    role: Role::Admin,\n</span><span style=\"color:#f8f8f2;\">    photo: </span><span style=\"color:#ffee99;\">&quot;tob_tobi_tob_cat.png&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">into</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">    verified: </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    created_at: Utc::now(),\n</span><span style=\"color:#f8f8f2;\">    updated_at: Utc::now(),\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> collection </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> db.collection::&lt;User&gt;(</span><span style=\"color:#ffee99;\">&quot;users&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">collection.</span><span style=\"color:#66d9ef;\">insert_one</span><span style=\"color:#f8f8f2;\">(new_user).await</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;</span></pre>\n",
        }
        p {
            "You're still using MongoDB - a famously schemaless, ultra-flexible NoSQL database - but with Rust, "
            strong { "your schema is no longer optional" }
            ". It's enforced. It's structured. It's "
            em { "safe" }
            ". Your compiler will scream before you ever send malformed data to your database."
        }
        p {
            "Meanwhile, in TypeScript? You'll define a Prisma schema, wait for it to regenerate types, run a migration, and then cross your fingers that the app doesn't crash on an optional field because some foreign key constraint failed silently."
        }
        h2 { id: "2-rusts-type-system-actually",
            a { href: "#2-rusts-type-system-actually", class: "header",
                "2. Rust's Type System Actually "
            }
            em { "Prevents" }
            " Bugs. TypeScript's Just Says \"Good Luck\"."
        }
        p {
            "Let's stop calling TypeScript \"safe.\" It's NOT. TypeScript is like a guard dog that barks loudly... but is chained up and can't actually stop intruders. You can tell TypeScript, \"this should be a string\", and then immediately say, \"but I'm going to treat it like an  "
            code { "any" }
            " anyway\". The compiler lets you do that. The code runs. The bug lives."
        }
        p {
            "Rust, on the other hand, is strict. Rust doesn't let you cheat. It doesn't care if you're late, tired, or really need this feature pushed today. If your types don't match, your code doesn't compile. No exceptions. That's not frustrating; That's freedom. Because once it builds, it "
            strong { "works" }
            "."
        }
        p { "Let's explore a TypeScript example, broken, but it compiles:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">User </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{ name: string }};\n</span><span style=\"font-style:italic;color:#66d9ef;\">const</span><span style=\"color:#f8f8f2;\"> getUser </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">(): any </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">({{ id: </span><span style=\"color:#ff80f4;\">123 </span><span style=\"color:#f8f8f2;\">}});\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">const</span><span style=\"color:#f8f8f2;\"> user </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> getUser() </span><span style=\"color:#f92672;\">as</span><span style=\"color:#f8f8f2;\"> User;\n</span><span style=\"color:#f8f8f2;\">console.</span><span style=\"color:#66d9ef;\">log</span><span style=\"color:#f8f8f2;\">(user.name); </span><span style=\"color:#8c8c8c;\">// undefined at runtime, no error until it&#39;s too late</span></pre>\n",
        }
        p { "Rust, on the other hand, won't even compile:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">User {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_user</span><span style=\"color:#f8f8f2;\">() -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ff80f4;\">123\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> user: User </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">get_user</span><span style=\"color:#f8f8f2;\">(); </span><span style=\"color:#8c8c8c;\">// error[E0308]: mismatched types\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "That's the beauty. Rust "
            em { "stops" }
            " you before disaster. TypeScript lets you slide down the hill and then says, \"Oopsie doopsie\"."
        }
        p {
            "And if you want "
            em { "really" }
            " rich type systems? Rust has traits, generics, enums with data, pattern matching, and more. TypeScript dreams of that kind of power but settles for "
            code { "Partial<T>" }
            " and "
            code { "Record<string, T>" }
            " hacks."
        }
        h2 { id: "3-rust-does",
            a { href: "#3-rust-does", class: "header", "3. Rust Does " }
            em { "Real" }
            " Concurrency. TypeScript Just Juggles Tasks."
        }
        p {
            "JavaScript's concurrency model is basically: one thread + hope. It's like a waiter taking 20 orders at once and telling customers, \"I'll get to you eventually\". It works for small apps and frontend tasks. But for serious backends, real-time apps, or high-performance systems? It's a liability."
        }
        p {
            "Rust offers "
            strong { "true multi-threading" }
            ", shared memory, message-passing channels, and "
            strong {
                code { "async/await" }
            }
            " with zero garbage collector overhead. You can create threads, spawn async tasks, and handle millions of requests concurrently without leaking memory or blocking your core thread."
        }
        p { "Let's explore a Rust concurrency example using Tokio:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">tokio::task;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">tokio::time::{{sleep, Duration}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">futures::future::join_all;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[tokio::main]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> tasks </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">vec![\n</span><span style=\"color:#f8f8f2;\">        task::spawn(</span><span style=\"color:#66d9ef;\">compute_something</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Task A&quot;</span><span style=\"color:#f8f8f2;\">)),\n</span><span style=\"color:#f8f8f2;\">        task::spawn(</span><span style=\"color:#66d9ef;\">compute_something</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Task B&quot;</span><span style=\"color:#f8f8f2;\">)),\n</span><span style=\"color:#f8f8f2;\">    ];\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">join_all</span><span style=\"color:#f8f8f2;\">(tasks).await;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">compute_something</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">name</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\"> is working...&quot;</span><span style=\"color:#f8f8f2;\">, name);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Some heavy async work\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">sleep</span><span style=\"color:#f8f8f2;\">(Duration::from_secs(</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">)).await;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Output\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Task A is working...\n</span><span style=\"color:#8c8c8c;\">// Task B is working...</span></pre>\n",
        }
        p {
            "You get "
            strong { "true parallelism" }
            " when needed. And memory is managed safely, no race conditions, no thread panics unless you explicitly allow it. Compare that to TypeScript's:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">async function </span><span style=\"color:#66d9ef;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">  await Promise.</span><span style=\"color:#66d9ef;\">all</span><span style=\"color:#f8f8f2;\">([doSomethingA(), doSomethingB()]);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "That looks clean, but it all runs on the same thread. Under pressure, it's going to choke. Rust's runtime is optimized for massive loads and deep concurrency, and it won't leave dangling promises in the dark."
        }
        h2 { id: "4-rusts-compiler",
            a { href: "#4-rusts-compiler", class: "header", "4. Rust's Compiler " }
            em { "Teaches" }
            " You. TypeScript Just Complains."
        }
        p {
            "Let's talk compiler errors. In TypeScript, errors are vague. They're short. They make you question your life choices. Rust's compiler, on the other hand, is like a wise elder, it doesn't just reject your code, it tells you "
            strong { "why" }
            ", where, how to fix it, and sometimes even what you probably meant."
        }
        p { "Let's take another look at our previous Rust example:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">User {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_user</span><span style=\"color:#f8f8f2;\">() -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ff80f4;\">123\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> user: User </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">get_user</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Output\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// error[E0308]: mismatched types\n</span><span style=\"color:#8c8c8c;\">//   --&gt; src/main.rs:10:22\n</span><span style=\"color:#8c8c8c;\">//    |\n</span><span style=\"color:#8c8c8c;\">// 10 |     let user: User = get_user(); // error[E0308]: mismatched types\n</span><span style=\"color:#8c8c8c;\">//    |               ----   ^^^^^^^^^^ expected `User`, found `i32`\n</span><span style=\"color:#8c8c8c;\">//    |               |\n</span><span style=\"color:#8c8c8c;\">//    |               expected due to this\n</span><span style=\"color:#8c8c8c;\">// \n</span><span style=\"color:#8c8c8c;\">// For more information about this error, try `rustc --explain E0308`.\n</span><span style=\"color:#8c8c8c;\">// error: could not compile `playground` (bin &quot;playground&quot;) due to 1 previous error</span></pre>\n",
        }
        p {
            "Not only does it tell you the line, the variable, and the mismatch. It shows you exactly what to fix. This makes Rust a "
            em { "learning experience" }
            ". It's hard at first, but over time it levels you up."
        }
        p { "In TypeScript, you'll often encounter:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">Type </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">string</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\"> is not assignable to </span><span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">number</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">.</span></pre>\n" }
        p { "Cool. Butt... which line? What variable? How do I fix it?" }
        p { "Rust wants you to get better. TypeScript wants you to suffer quietly." }
        h2 { id: "5-memory-safety-in-rust-isnt-optional-in-typescript-you-can-leak-like-a-faucet",
            a {
                href: "#5-memory-safety-in-rust-isnt-optional-in-typescript-you-can-leak-like-a-faucet",
                class: "header",
                "5. Memory Safety in Rust Isn't Optional. In TypeScript, You Can Leak Like a Faucet."
            }
        }
        p {
            "Rust is built around "
            strong { "ownership" }
            " and "
            strong { "lifetimes" }
            ". These are core concepts that guarantee memory is safely handled. When a variable goes out of scope, it's dropped. You don't need a garbage collector. You don't need to "
            code { "delete" }
            " things manually. And you don't need to worry about slow runtime memory leaks."
        }
        p {
            "TypeScript? You've got a GC. Cool. But GCs are unpredictable. Memory leaks happen when references aren't cleaned up properly, closures hold onto state, event listeners pile up, unused variables chill in RAM, and suddenly your Node.js app is eating 2GB of memory for no reason."
        }
        p { "Let's take a look at a Rust example:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">allocate_data</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> data </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">vec![</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">3</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// data is freed automatically when it goes out of scope\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "No leak. No GC. No surprises. You write the code, and Rust ensures memory is released safely, every single time."
        }
        h2 { id: "6-rusts-tooling-works-out-of-the-box-typescript-devs-live-in-dependency-hell",
            a {
                href: "#6-rusts-tooling-works-out-of-the-box-typescript-devs-live-in-dependency-hell",
                class: "header",
                "6. Rust's Tooling Works Out-of-the-Box. TypeScript Devs Live in Dependency Hell."
            }
        }
        p {
            "Let's talk about dev setup. With TypeScript, the typical project setup requires "
            strong { "Node" }
            ", "
            strong { "npm/yarn/pnpm" }
            ", "
            strong { "tsconfig.json" }
            ", "
            strong { "eslint" }
            ", "
            strong { "prettier" }
            ", "
            strong { "Jest" }
            ", "
            strong { "ts-node" }
            ", "
            strong { "webpack or Vite" }
            ", and maybe a dozen other tools just to get a basic dev environment running. That's before you even write a single line of code. Every time you start a new project, you basically have to rebuild the wheel. You Google \"best tsconfig\", copy-paste some Stack Overflow answer, and cross your fingers. And once you get it running, there's still a weekly stream of broken dependencies and config nightmares."
        }
        p {
            "Rust? It gives you "
            em { "everything" }
            " you need with one tool: "
            code { "cargo" }
            ". You install Rust using "
            a { href: "https://rustup.rs/", "rustup" }
            ", and suddenly you have a compiler ("
            code { "rustc" }
            "), a package manager ("
            code { "cargo" }
            "), a formatter ("
            code { "rustfmt" }
            "), a linter ("
            code { "clippy" }
            "), a test runner, a doc generator, and a build tool. All built-in. No plugins, no config chaos. You type "
            code { "cargo new my_proj" }
            ", and you're coding. That's it."
        }
        h3 { id: "want-to-format-your-code-deal",
            a { href: "#want-to-format-your-code-deal", class: "header",
                "Want to format your code? Deal!"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo fmt</span></pre>\n" }
        h3 { id: "want-to-lint-it-gotcha",
            a { href: "#want-to-lint-it-gotcha", class: "header", "Want to lint it? Gotcha!" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo clippy</span></pre>\n" }
        h3 { id: "want-to-build-it-cool",
            a { href: "#want-to-build-it-cool", class: "header", "Want to build it? Cool!" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">release</span></pre>\n" }
        h3 { id: "want-to-run-tests-awesome",
            a { href: "#want-to-run-tests-awesome", class: "header", "Want to run tests? Awesome!" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo test</span></pre>\n" }
        p {
            "All of this just works, everywhere. No hours lost on setting up a CI pipeline. No guessing what version of TypeScript is compatible with what version of Webpack. In Rust, your productivity is "
            strong { "compounding" }
            ", not constantly being reset."
        }
        h2 { id: "7-rusts-error-handling-is-real-typescript-just-throws-and-hopes",
            a {
                href: "#7-rusts-error-handling-is-real-typescript-just-throws-and-hopes",
                class: "header",
                "7. Rust's Error Handling is Real. TypeScript Just Throws and Hopes."
            }
        }
        p {
            "TypeScript error handling is like a seatbelt made of spaghetti. Sure, you can  "
            code { "try/catch" }
            ", but exceptions in JavaScript are a runtime mess. Everything is a possible  "
            code { "undefined" }
            ", so half the code is checking for nulls,  "
            code { "typeof === \"function\"" }
            ", and truthy/falsy values. You throw errors using  "
            code { "throw" }
            ", but catching them is a gamble. If something fails deep in the call stack, and you didn't catch it early, you're cooked."
        }
        p {
            "Rust doesn't do runtime errors like that. It uses  "
            code { "Result<T, E>" }
            ", a built-in type that forces you to handle both success and failure. The compiler enforces it. If you call a function that can fail, you "
            strong { "must" }
            " deal with the error. No unhandled exceptions. No hidden undefineds. Just crystal-clear control flow."
        }
        p { "Let's consider a Rust example:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">divide</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">a</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">b</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; Result&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">, String&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> b </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Division by zero&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">())\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(a </span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\"> b)\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match </span><span style=\"color:#66d9ef;\">divide</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">10</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(result) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Result is </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, result),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(e) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Error: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, e),\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Output\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Error: Division by zero</span></pre>\n",
        }
        p {
            "This is type-safe, explicit, and predictable. You can use combinators like  "
            code { ".map()" }
            ",  "
            code { ".and_then()" }
            ", or even the elegant  "
            code { "?" }
            " operator to propagate errors up the stack. That means you get all the power of exceptions, but with "
            strong { "no runtime chaos" }
            "."
        }
        h2 { id: "8-rusts-enums-are-algebraic-data-types-typescripts-unions-are-just-strings-with-hats",
            a {
                href: "#8-rusts-enums-are-algebraic-data-types-typescripts-unions-are-just-strings-with-hats",
                class: "header",
                "8. Rust's Enums Are Algebraic Data Types. TypeScript's Unions Are Just Strings with Hats."
            }
        }
        p {
            "TypeScript has something called union types, like  "
            code { "type Shape = 'circle' | 'square' | 'triangle'" }
            ". It's better than plain JS, sure. But they're "
            strong { "flat" }
            ". You can't associate meaningful data with each variant cleanly. You have to use additional structures or type guards. And if you forget one case in a "
            code { "switch" }
            ", nothing warns you."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">Shape </span><span style=\"color:#f92672;\">= &#39;</span><span style=\"color:#f8f8f2;\">circle</span><span style=\"color:#f92672;\">&#39; | &#39;</span><span style=\"color:#f8f8f2;\">square</span><span style=\"color:#f92672;\">&#39; | &#39;</span><span style=\"color:#f8f8f2;\">triangle</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">function calculateArea(shape: Shape, size: number): number {{\n</span><span style=\"color:#f8f8f2;\">  switch (shape) {{\n</span><span style=\"color:#f8f8f2;\">    case </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">circle</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">:\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">return</span><span style=\"color:#f8f8f2;\"> Math.</span><span style=\"color:#ff80f4;\">PI </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> size </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> size;\n</span><span style=\"color:#f8f8f2;\">    case </span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">square</span><span style=\"color:#f92672;\">&#39;</span><span style=\"color:#f8f8f2;\">:\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">return</span><span style=\"color:#f8f8f2;\"> size </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> size;\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">return </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Rust's  "
            code { "enum" }
            " is a full-featured "
            strong { "algebraic data type" }
            ". Each variant can carry "
            strong { "its own structure" }
            ", and pattern matching on them is "
            strong { "exhaustive" }
            ". If you forget to handle a variant, the compiler slaps your wrist before you run the code."
        }
        p { "Let's consider a Rust example:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Shape {{\n</span><span style=\"color:#f8f8f2;\">    Circle {{ radius: </span><span style=\"font-style:italic;color:#66d9ef;\">f64 </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">    Square {{ side: </span><span style=\"font-style:italic;color:#66d9ef;\">f64 </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">    Triangle {{ base: </span><span style=\"font-style:italic;color:#66d9ef;\">f64</span><span style=\"color:#f8f8f2;\">, height: </span><span style=\"font-style:italic;color:#66d9ef;\">f64 </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">area</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">shape</span><span style=\"color:#f8f8f2;\">: Shape) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">f64 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> shape {{\n</span><span style=\"color:#f8f8f2;\">        Shape::Circle {{ radius }} </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#ff80f4;\">3.14 </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> radius </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> radius,\n</span><span style=\"color:#f8f8f2;\">        Shape::Square {{ side }} </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> side </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> side,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Output\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">//  --&gt; src/lib.rs:8:11\n</span><span style=\"color:#8c8c8c;\">//   |\n</span><span style=\"color:#8c8c8c;\">// 8 |     match shape {{\n</span><span style=\"color:#8c8c8c;\">//   |           ^^^^^ pattern `Shape::Triangle {{ .. }}` not covered\n</span><span style=\"color:#8c8c8c;\">//   |\n</span><span style=\"color:#8c8c8c;\">// note: `Shape` defined here\n</span><span style=\"color:#8c8c8c;\">//  --&gt; src/lib.rs:1:6\n</span><span style=\"color:#8c8c8c;\">//   |\n</span><span style=\"color:#8c8c8c;\">// 1 | enum Shape {{\n</span><span style=\"color:#8c8c8c;\">//   |      ^^^^^\n</span><span style=\"color:#8c8c8c;\">// ...\n</span><span style=\"color:#8c8c8c;\">// 4 |     Triangle {{ base: f64, height: f64 }},\n</span><span style=\"color:#8c8c8c;\">//   |     -------- not covered\n</span><span style=\"color:#8c8c8c;\">//   = note: the matched value is of type `Shape`\n</span><span style=\"color:#8c8c8c;\">// help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown\n</span><span style=\"color:#8c8c8c;\">//   |\n</span><span style=\"color:#8c8c8c;\">// 10~         Shape::Square {{ side }} =&gt; side * side,\n</span><span style=\"color:#8c8c8c;\">// 11~         Shape::Triangle {{ .. }} =&gt; todo!(),\n</span><span style=\"color:#8c8c8c;\">//   |</span></pre>\n",
        }
        p {
            "This is expressive. It's powerful. It's "
            strong { "compiler-enforced safety" }
            ". TypeScript can't touch this. At best, you get a combination of interfaces, union types, and runtime type guards, and even then, it can still go wrong. In Rust, once it compiles, it's solid."
        }
        h2 { id: "9-rusts-traits-enable-powerful-generic-code-typescript-has-duck-typing-and-hopes",
            a {
                href: "#9-rusts-traits-enable-powerful-generic-code-typescript-has-duck-typing-and-hopes",
                class: "header",
                "9. Rust's Traits Enable Powerful, Generic Code. TypeScript Has Duck Typing and Hopes."
            }
        }
        p {
            "In TypeScript, if two objects look the same, they're considered the same; That's duck typing. It's flexible, but also fragile. You can accidentally \"satisfy\" a type without actually meaning to. There's no real interface enforcement at runtime. And generic constraints? They're shallow."
        }
        p {
            "Rust has "
            strong { "traits" }
            ", which are contracts for behavior. If a type implements a trait, the compiler guarantees that it provides the required methods. You can write generic functions and structs that work across types, as long as those types implement the necessary traits. This gives you real "
            strong { "polymorphism" }
            ", enforced by the compiler, with zero overhead."
        }
        p { "Let's consider a Rust example:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">trait </span><span style=\"color:#f8f8f2;\">Printable {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">print</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">Document;\n</span><span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">Image;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">Printable </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">Document {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">print</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">        println!(</span><span style=\"color:#ffee99;\">&quot;Printing document...&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">Printable </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">Image {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">print</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">        println!(</span><span style=\"color:#ffee99;\">&quot;Printing image...&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">print_all</span><span style=\"color:#f8f8f2;\">&lt;T: Printable&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">items</span><span style=\"color:#f8f8f2;\">: Vec&lt;T&gt;) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> item </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> items {{\n</span><span style=\"color:#f8f8f2;\">        item.</span><span style=\"color:#66d9ef;\">print</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This is generic, reusable, and strictly typed. There's no ambiguity. No unexpected duck behavior. Just clean, reliable abstraction. And it works at "
            strong { "zero cost" }
            ", meaning there's no performance penalty for using these abstractions."
        }
        h2 { id: "10-rust-has-webassembly-muscle-typescript-just-compiles-to-more-js",
            a {
                href: "#10-rust-has-webassembly-muscle-typescript-just-compiles-to-more-js",
                class: "header",
                "10. Rust Has WebAssembly Muscle. TypeScript Just Compiles to More JS."
            }
        }
        p {
            "WebAssembly is the future of high-performance web apps, and Rust is the "
            strong { "king" }
            " of Wasm. You can write entire parts of your frontend in Rust, compile them to Wasm, and get performance on par with native code, running in the browser. Not just that. You also get memory safety, type safety, and way better performance than JS for CPU-heavy tasks."
        }
        p {
            "TypeScript? You compile it to JavaScript. That's all it ever does. It can't go lower. It can't go native. It can't do real-time rendering or heavy lifting without choking."
        }
        p { "Let's consider a Rust example:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">wasm_bindgen::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[wasm_bindgen]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">add</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">a</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">b</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    a </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> b\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Compile with  "
            code { "wasm-pack build" }
            ", and boom. You have a lightning-fast WebAssembly module, callable from JavaScript."
        }
        p {
            "Rust is being used in production WebAssembly apps, graphics engines, real-time simulations, and more. If you're serious about performance in the browser, Rust isn't optional. It's essential."
        }
        h2 { id: "11-rust-has-a-strong-growing-ecosystem-typescript-is-still-tied-to-javascripts-chaos",
            a {
                href: "#11-rust-has-a-strong-growing-ecosystem-typescript-is-still-tied-to-javascripts-chaos",
                class: "header",
                "11. Rust Has a Strong, Growing Ecosystem. TypeScript Is Still Tied to JavaScript's Chaos."
            }
        }
        p {
            "Let's be honest: a huge part of TypeScript's ecosystem is just "
            strong { "wrappers around JavaScript libraries" }
            ". It's bolted-on typing. Most TS libraries are ports or bindings of JS ones, and you still deal with the old JS quirks, "
            code { "any" }
            " types, and version mismatches."
        }
        p {
            "Rust? It's built clean from the ground up. The "
            a { href: "https://crates.io/", "crates.io" }
            " registry is full of "
            strong { "modern, safe, composable libraries" }
            ". You've got "
            a { href: "https://docs.rs/axum", "Axum" }
            ", "
            a { href: "https://docs.rs/rocket", "Rocket" }
            " and "
            a { href: "https://docs.rs/actix", "Actix" }
            " for backends, "
            a { href: "https://leptos.dev", "Leptos" }
            ", "
            a { href: "https://dioxuslabs.com/", "Dioxus" }
            ", and "
            a { href: "https://yew.rs", "Yew" }
            " for frontend, and more. Every library you use follows the same philosophy: safety, performance, and "
            strong { "zero tolerance for ambiguity" }
            "."
        }
        p {
            "You can write a "
            strong { "frontend" }
            ", "
            strong { "backend" }
            ", "
            strong { "CLI" }
            ", and even a "
            strong { "game engine" }
            ", all in one language. You don't need separate runtimes, tools, or platforms. Rust does it all, and it does it better."
        }
        h2 { id: "final-word-for-now",
            a { href: "#final-word-for-now", class: "header", "Final Word, for now..." }
        }
        p {
            "If you've made it this far, you're either a true believer or you're furiously writing a rebuttal in VS Code with three broken npm packages. Either way, the truth is clear:"
        }
        blockquote {
            p {
                "Rust is better than TypeScript. Not just \"better\". It's a different league. A language for real software."
            }
        }
        p {
            "We're not saying TypeScript is bad. We're saying "
            strong { "Rust solves problems TypeScript can't even define" }
            ". If you want to build reliable, performant, error-free systems, Rust is your new home."
        }
        blockquote {
            p {
                strong { "We are Open SASS, babe!" }
                "."
            }
        }
        blockquote {
            p { "We're working tirelessly on making Rust development extremely easy for everyone." }
        }
        blockquote {
            p {
                "If you made it this far, it would be nice if you could "
                a { href: "https://discord.gg/b5JbvHW5nv", "join us on Discord" }
                "."
            }
        }
        p {
            "Together, let's move the web beyond JavaScript, and into something that actually compiles."
        }
        blockquote {
            p {
                "Feel free to share this. Fork it. Turn it into a meme. Tattoo it on your CI pipeline. Tell your manager Rust is your spirit animal now."
            }
        }
    }
}

use crate::components::blog::code::CodeBlock;
use dioxus::prelude::*;
