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
    #[route("/beyond-typescript")]
    BeyondTypescript {},
    #[route("/rust-is-god-101")]
    RustIsGod101 {},
    #[route("/opensass-vs-shadcn")]
    OpensassVsShadcn {},
    #[route("/image-rs-release")]
    ImageRsRelease {},
    #[route("/image-rs-vs-next-js-image")]
    ImageRsVsNextJsImage {},
    #[route("/table-rs-release")]
    TableRsRelease {},
    #[route("/tanstack-table-vs-table-rs")]
    TanstackTableVsTableRs {},
    #[route("/navbar-release")]
    NavbarRelease {},
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
            BookRoute::BeyondTypescript {} => use_mdbook::mdbook_shared::PageId(4usize),
            BookRoute::RustIsGod101 {} => use_mdbook::mdbook_shared::PageId(5usize),
            BookRoute::OpensassVsShadcn {} => use_mdbook::mdbook_shared::PageId(6usize),
            BookRoute::ImageRsRelease {} => use_mdbook::mdbook_shared::PageId(7usize),
            BookRoute::ImageRsVsNextJsImage {} => use_mdbook::mdbook_shared::PageId(8usize),
            BookRoute::TableRsRelease {} => use_mdbook::mdbook_shared::PageId(9usize),
            BookRoute::TanstackTableVsTableRs {} => use_mdbook::mdbook_shared::PageId(10usize),
            BookRoute::NavbarRelease {} => use_mdbook::mdbook_shared::PageId(11usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::NavbarRelease {}
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
                        title: " 5 |---| Beyond TypeScript |---| blog |---| beyond-typescript |---| Apr 20 2025 |---| Hey devs, and anyone still dealing with a 900MB node_modules folder. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
                            .to_string(),
                        url: BookRoute::BeyondTypescript {},
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
            BookRoute::BeyondTypescript {},
            ::use_mdbook::mdbook_shared::PageId(4usize),
        );
        pages
            .push((
                5usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 6 |---| Biblically Accurate Rust 😇 |---| blog |---| rust-is-god-101 |---| Apr 22 2025 |---| Rust is the biblically accurate programming language; Fast, safe, and blessed with memory safety. |---| https://github.com/user-attachments/assets/a9fc3f6f-8ad3-40b9-92c8-233efa64acc0"
                            .to_string(),
                        url: BookRoute::RustIsGod101 {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "TL;DR".to_string(),
                                id: "tl;dr".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Table of Contents".to_string(),
                                id: "table-of-contents".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Who Is This Article For?".to_string(),
                                id: "who-is-this-article-for?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Is Rust?".to_string(),
                                id: "what-is-rust?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why Not Rust?".to_string(),
                                id: "why-not-rust?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Personal Experience".to_string(),
                                id: "personal-experience".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why I Switched to Rust".to_string(),
                                id: "why-i-switched-to-rust".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "How To Get Started".to_string(),
                                id: "how-to-get-started".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Verifying Rust Installation".to_string(),
                                id: "verifying-rust-installation".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Creating a Rust Project".to_string(),
                                id: "creating-a-rust-project".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Linters and Formatters".to_string(),
                                id: "linters-and-formatters".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Basic Rust Concepts".to_string(),
                                id: "basic-rust-concepts".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Variables".to_string(),
                                id: "variables".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Constants".to_string(),
                                id: "constants".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Functions".to_string(),
                                id: "functions".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Assertions and the ".to_string(),
                                id: "assertions-and-the".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The ".to_string(),
                                id: "the".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Functions and Returning Values".to_string(),
                                id: "functions-and-returning-values".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Rust's Type System".to_string(),
                                id: "rust's-type-system".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Scalar Types:".to_string(),
                                id: "scalar-types:".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Compound Types:".to_string(),
                                id: "compound-types:".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: " Statements and Control Flow".to_string(),
                                id: "statements-and-control-flow".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Enums in Rust".to_string(),
                                id: "enums-in-rust".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Rust Generics".to_string(),
                                id: "rust-generics".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Optional Enums (".to_string(),
                                id: "optional-enums-(".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Traits".to_string(),
                                id: "traits".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Ownership".to_string(),
                                id: "ownership".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Borrowing".to_string(),
                                id: "borrowing".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Lifetimes".to_string(),
                                id: "lifetimes".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Wrap Up".to_string(),
                                id: "wrap-up".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(5usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::RustIsGod101 {},
            ::use_mdbook::mdbook_shared::PageId(5usize),
        );
        pages
            .push((
                6usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 7 |---| Open SASS vs ShadCN UI ⚔\u{fe0f} |---| blog |---| opensass-vs-shadcn |---| Apr 23 2025 |---| Open SASS obliterates ShadCN UI with framework-agnostic components, blazing performance, and unmatched versatility. |---| https://github.com/user-attachments/assets/5e2bf427-0401-4cf6-9c72-d9bffb445ee0"
                            .to_string(),
                        url: BookRoute::OpensassVsShadcn {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Framework and Styling Agnosticism".to_string(),
                                id: "framework-and-styling-agnosticism".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Performance".to_string(),
                                id: "performance".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Customization and Extensibility".to_string(),
                                id: "customization-and-extensibility".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Conclusion".to_string(),
                                id: "conclusion".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(6usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::OpensassVsShadcn {},
            ::use_mdbook::mdbook_shared::PageId(6usize),
        );
        pages
            .push((
                7usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 8 |---| Image RS: Next-Gen WASM Image Component 🚀 |---| blog |---| image-rs-release |---| Apr 26 2025 |---| Image RS launches as the ultimate image solution for Yew, Dioxus, and Leptos apps with smart lazy loading, responsive layouts, accessibility, and incredible flexibility. |---| https://raw.githubusercontent.com/opensass/image-rs/refs/heads/main/assets/logo.webp"
                            .to_string(),
                        url: BookRoute::ImageRsRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📸 What Is Image RS?".to_string(),
                                id: "📸-what-is-image-rs?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Why You'll Love Image RS".to_string(),
                                id: "🚀-why-you'll-love-image-rs".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔥 Quick Yew Setup".to_string(),
                                id: "🔥-quick-yew-setup".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧩 Features That Set It Apart".to_string(),
                                id: "🧩-features-that-set-it-apart".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚙\u{fe0f} Full Control with Props".to_string(),
                                id: "⚙\u{fe0f}-full-control-with-props".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎯 Real-World Examples".to_string(),
                                id: "🎯-real-world-examples".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "💡 Final Thoughts".to_string(),
                                id: "💡-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(7usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::ImageRsRelease {},
            ::use_mdbook::mdbook_shared::PageId(7usize),
        );
        pages
            .push((
                8usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 9 |---| Image RS vs Next.js Image 🔥 |---| blog |---| image-rs-vs-next-js-image |---| Apr 27 2025 |---| A deep comparison proving why Yew Image RS outperforms Next.js Image with native WASM speed, fine-grained DOM control, better memory usage, and smoother performance at scale. |---| https://github.com/user-attachments/assets/9fa9ff50-32ea-4369-a263-0bb8c32197c1"
                            .to_string(),
                        url: BookRoute::ImageRsVsNextJsImage {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚔\u{fe0f} Who Are The Competitors?".to_string(),
                                id: "⚔\u{fe0f}-who-are-the-competitors?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🥊 Next.js Image".to_string(),
                                id: "🥊-next.js-image".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🥊 Image RS".to_string(),
                                id: "🥊-image-rs".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧠 Why Image RS Wins".to_string(),
                                id: "🧠-why-image-rs-wins".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Native Rust + Wasm vs JavaScript".to_string(),
                                id: "native-rust-+-wasm-vs-javascript".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Built-in Image Optimization: Both Are Good (Kind Of)"
                                    .to_string(),
                                id: "built-in-image-optimization:-both-are-good-(kind-of)"
                                    .to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Fine-Grained DOM Control: WASM Takes the Crown"
                                    .to_string(),
                                id: "fine-grained-dom-control:-wasm-takes-the-crown"
                                    .to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "JS/Wasm Payload Size: Both Keep It Tight"
                                    .to_string(),
                                id: "js/wasm-payload-size:-both-keep-it-tight".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📈 Real Benchmark Time.".to_string(),
                                id: "📈-real-benchmark-time.".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "When Loading 10 Images (small scale)".to_string(),
                                id: "when-loading-10-images-(small-scale)".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "When Loading 10,000 Images (huge scale)"
                                    .to_string(),
                                id: "when-loading-10,000-images-(huge-scale)".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧪 Why Wasm + Rust Crushes JS + React"
                                    .to_string(),
                                id: "🧪-why-wasm-+-rust-crushes-js-+-react".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎯 The Verdict".to_string(),
                                id: "🎯-the-verdict".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Final Thoughts".to_string(),
                                id: "🚀-final-thoughts".to_string(),
                                level: 1usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(8usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::ImageRsVsNextJsImage {},
            ::use_mdbook::mdbook_shared::PageId(8usize),
        );
        pages
            .push((
                9usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 10 |---| Table RS: Advanced Wasmy Table Component 📊 |---| blog |---| table-rs-release |---| Apr 29 2025 |---| Table RS delivers a fully-featured, accessible, and customizable table component for Wasm apps with built-in search, sorting, pagination, and styling control. |---| https://raw.githubusercontent.com/opensass/table-rs/refs/heads/main/assets/logo.webp"
                            .to_string(),
                        url: BookRoute::TableRsRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📦 What Is Table RS?".to_string(),
                                id: "📦-what-is-table-rs?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Why You'll Love Table RS".to_string(),
                                id: "🚀-why-you'll-love-table-rs".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚙\u{fe0f} Quick Setup for Yew".to_string(),
                                id: "⚙\u{fe0f}-quick-setup-for-yew".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "1\u{fe0f}\u{20e3} Add the Dependency".to_string(),
                                id: "1\u{fe0f}\u{20e3}-add-the-dependency".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "2\u{fe0f}\u{20e3} Import the Component".to_string(),
                                id: "2\u{fe0f}\u{20e3}-import-the-component".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "3\u{fe0f}\u{20e3} Use It in Your Component"
                                    .to_string(),
                                id: "3\u{fe0f}\u{20e3}-use-it-in-your-component"
                                    .to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧩 Table RS Props Overview".to_string(),
                                id: "🧩-table-rs-props-overview".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔑 Core Props".to_string(),
                                id: "🔑-core-props".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧱 Column Definition".to_string(),
                                id: "🧱-column-definition".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎨 Style/Classes Reference".to_string(),
                                id: "🎨-style/classes-reference".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🤝 Built With Open SASS".to_string(),
                                id: "🤝-built-with-open-sass".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎯 Final Thoughts".to_string(),
                                id: "🎯-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(9usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::TableRsRelease {},
            ::use_mdbook::mdbook_shared::PageId(9usize),
        );
        pages
            .push((
                10usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 11 |---| Table RS: Why WASM Outperforms JS at Scale 📈 |---| blog |---| tanstack-table-vs-table-rs |---| Apr 29 2025 |---| A deep-dive benchmark comparing TanStack Table (React) vs Table RS (Yew + WASM). |---| https://github.com/user-attachments/assets/2cd8279a-9d9d-4f75-bf13-b61fbbb130da"
                            .to_string(),
                        url: BookRoute::TanstackTableVsTableRs {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚔\u{fe0f} The Competitors: React vs WASM"
                                    .to_string(),
                                id: "⚔\u{fe0f}-the-competitors:-react-vs-wasm"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🟨 TanStack Table (React)".to_string(),
                                id: "🟨-tanstack-table-(react)".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🟩 Table RS (Yew + WASM)".to_string(),
                                id: "🟩-table-rs-(yew-+-wasm)".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📊 Benchmark: TanStack Table vs Table RS (1 Million Rows)"
                                    .to_string(),
                                id: "📊-benchmark:-tanstack-table-vs-table-rs-(1-million-rows)"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧠 Why TanStack Falls Apart".to_string(),
                                id: "🧠-why-tanstack-falls-apart".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🦾 Why Table RS Rocks So Hard".to_string(),
                                id: "🦾-why-table-rs-rocks-so-hard".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Real-World Experience".to_string(),
                                id: "🚀-real-world-experience".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔥 Why WebAssembly Is the Future".to_string(),
                                id: "🔥-why-webassembly-is-the-future".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📣 Final Verdict".to_string(),
                                id: "📣-final-verdict".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "👋 Wrap-Up: JS Had a Good Run...".to_string(),
                                id: "👋-wrap-up:-js-had-a-good-run...".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(10usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::TanstackTableVsTableRs {},
            ::use_mdbook::mdbook_shared::PageId(10usize),
        );
        pages
            .push((
                11usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 12 |---| 🍔 Navbar: A Deliciously Simple Wasmy Navbar Component |---| blog |---| navbar-release |---| May 01 2025 |---| A hands-on guide to adding a feature-rich, fully customizable Navbar component to your WASM app. |---| https://github.com/user-attachments/assets/1fa1e562-8861-4dd9-99af-060c768a23a7"
                            .to_string(),
                        url: BookRoute::NavbarRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 What Is Navbar?".to_string(),
                                id: "🚀-what-is-navbar?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚡\u{fe0f} Why You'll Love It".to_string(),
                                id: "⚡\u{fe0f}-why-you'll-love-it".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧰 Quick Setup in Yew".to_string(),
                                id: "🧰-quick-setup-in-yew".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧩 Feature Breakdown".to_string(),
                                id: "🧩-feature-breakdown".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧭 Logo & Menu".to_string(),
                                id: "🧭-logo-&-menu".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔍 Search Input".to_string(),
                                id: "🔍-search-input".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🍕 Mega Menu".to_string(),
                                id: "🍕-mega-menu".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🙋 Profile Menu".to_string(),
                                id: "🙋-profile-menu".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📱 Mobile Support".to_string(),
                                id: "📱-mobile-support".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎨 Full Styling Control".to_string(),
                                id: "🎨-full-styling-control".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "💻 Developer Features".to_string(),
                                id: "💻-developer-features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎛\u{fe0f} Full Props Reference".to_string(),
                                id: "🎛\u{fe0f}-full-props-reference".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧪 Real-World Scenarios".to_string(),
                                id: "🧪-real-world-scenarios".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "A clean marketing navbar:".to_string(),
                                id: "a-clean-marketing-navbar:".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "A complex app dashboard navbar:".to_string(),
                                id: "a-complex-app-dashboard-navbar:".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "A styled, custom-themed navbar:".to_string(),
                                id: "a-styled,-custom-themed-navbar:".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "💡 Final Thoughts".to_string(),
                                id: "💡-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(11usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::NavbarRelease {},
            ::use_mdbook::mdbook_shared::PageId(11usize),
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
                        name: " 5 |---| Beyond TypeScript |---| blog |---| beyond-typescript |---| Apr 20 2025 |---| Hey devs, and anyone still dealing with a 900MB node_modules folder. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
                            .to_string(),
                        location: Some(BookRoute::BeyondTypescript {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 6 |---| Biblically Accurate Rust 😇 |---| blog |---| rust-is-god-101 |---| Apr 22 2025 |---| Rust is the biblically accurate programming language; Fast, safe, and blessed with memory safety. |---| https://github.com/user-attachments/assets/a9fc3f6f-8ad3-40b9-92c8-233efa64acc0"
                            .to_string(),
                        location: Some(BookRoute::RustIsGod101 {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![6u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 7 |---| Open SASS vs ShadCN UI ⚔\u{fe0f} |---| blog |---| opensass-vs-shadcn |---| Apr 23 2025 |---| Open SASS obliterates ShadCN UI with framework-agnostic components, blazing performance, and unmatched versatility. |---| https://github.com/user-attachments/assets/5e2bf427-0401-4cf6-9c72-d9bffb445ee0"
                            .to_string(),
                        location: Some(BookRoute::OpensassVsShadcn {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![7u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 8 |---| Image RS: Next-Gen WASM Image Component 🚀 |---| blog |---| image-rs-release |---| Apr 26 2025 |---| Image RS launches as the ultimate image solution for Yew, Dioxus, and Leptos apps with smart lazy loading, responsive layouts, accessibility, and incredible flexibility. |---| https://raw.githubusercontent.com/opensass/image-rs/refs/heads/main/assets/logo.webp"
                            .to_string(),
                        location: Some(BookRoute::ImageRsRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![8u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 9 |---| Image RS vs Next.js Image 🔥 |---| blog |---| image-rs-vs-next-js-image |---| Apr 27 2025 |---| A deep comparison proving why Yew Image RS outperforms Next.js Image with native WASM speed, fine-grained DOM control, better memory usage, and smoother performance at scale. |---| https://github.com/user-attachments/assets/9fa9ff50-32ea-4369-a263-0bb8c32197c1"
                            .to_string(),
                        location: Some(BookRoute::ImageRsVsNextJsImage {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![9u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 10 |---| Table RS: Advanced Wasmy Table Component 📊 |---| blog |---| table-rs-release |---| Apr 29 2025 |---| Table RS delivers a fully-featured, accessible, and customizable table component for Wasm apps with built-in search, sorting, pagination, and styling control. |---| https://raw.githubusercontent.com/opensass/table-rs/refs/heads/main/assets/logo.webp"
                            .to_string(),
                        location: Some(BookRoute::TableRsRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![10u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 11 |---| Table RS: Why WASM Outperforms JS at Scale 📈 |---| blog |---| tanstack-table-vs-table-rs |---| Apr 29 2025 |---| A deep-dive benchmark comparing TanStack Table (React) vs Table RS (Yew + WASM). |---| https://github.com/user-attachments/assets/2cd8279a-9d9d-4f75-bf13-b61fbbb130da"
                            .to_string(),
                        location: Some(BookRoute::TanstackTableVsTableRs {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![11u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 12 |---| 🍔 Navbar: A Deliciously Simple Wasmy Navbar Component |---| blog |---| navbar-release |---| May 01 2025 |---| A hands-on guide to adding a feature-rich, fully customizable Navbar component to your WASM app. |---| https://github.com/user-attachments/assets/1fa1e562-8861-4dd9-99af-060c768a23a7"
                            .to_string(),
                        location: Some(BookRoute::NavbarRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![12u32]),
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
pub fn BeyondTypescript() -> dioxus::prelude::Element {
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
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">User {{\n</span><span style=\"color:#f8f8f2;\">    name: String,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">get_user</span><span style=\"color:#f8f8f2;\">() -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ff80f4;\">123\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> user: User </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">get_user</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Output\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// error[E0308]: mismatched types\n</span><span style=\"color:#8c8c8c;\">//   --&gt; src/main.rs:10:22\n</span><span style=\"color:#8c8c8c;\">//    |\n</span><span style=\"color:#8c8c8c;\">// 10 |     let user: User = get_user(); // error[E0308]: mismatched types\n</span><span style=\"color:#8c8c8c;\">//    |               ----   ^^^^^^^^^^ expected `User`, found `i32`\n</span><span style=\"color:#8c8c8c;\">//    |               |\n</span><span style=\"color:#8c8c8c;\">//    |               expected due to this\n</span><span style=\"color:#8c8c8c;\">//\n</span><span style=\"color:#8c8c8c;\">// For more information about this error, try `rustc --explain E0308`.\n</span><span style=\"color:#8c8c8c;\">// error: could not compile `playground` (bin &quot;playground&quot;) due to 1 previous error</span></pre>\n",
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
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">Shape </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ffee99;\">&quot;circle&quot; </span><span style=\"color:#f92672;\">| </span><span style=\"color:#ffee99;\">&quot;square&quot; </span><span style=\"color:#f92672;\">| </span><span style=\"color:#ffee99;\">&quot;triangle&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">function calculateArea(shape: Shape, size: number): number {{\n</span><span style=\"color:#f8f8f2;\">  switch (shape) {{\n</span><span style=\"color:#f8f8f2;\">    case </span><span style=\"color:#ffee99;\">&quot;circle&quot;</span><span style=\"color:#f8f8f2;\">:\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">return</span><span style=\"color:#f8f8f2;\"> Math.</span><span style=\"color:#ff80f4;\">PI </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> size </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> size;\n</span><span style=\"color:#f8f8f2;\">    case </span><span style=\"color:#ffee99;\">&quot;square&quot;</span><span style=\"color:#f8f8f2;\">:\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">return</span><span style=\"color:#f8f8f2;\"> size </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> size;\n</span><span style=\"color:#f8f8f2;\">  }}\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">return </span><span style=\"color:#ff80f4;\">0</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
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
            p { "We're working tirelessly on making Rust web development extremely easy for everyone." }
        }
        blockquote {
            p {
                "If you made it this far, it would be nice if you could "
                a { href: "https://discord.gg/b5JbvHW5nv", "join us on Discord" }
                "."
            }
        }
        blockquote {
            p {
                "Together, let's move the web beyond JavaScript, and into something that actually compiles."
            }
        }
        blockquote {
            p {
                "Feel free to share this. Fork it. Turn it into a meme. Tattoo it on your CI pipeline. Tell your manager Rust is your spirit animal now."
            }
        }
    }
}
#[component(no_case_check)]
pub fn RustIsGod101() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h2 { id: "tldr",
            a { href: "#tldr", class: "header", "TL;DR" }
        }
        blockquote {
            p { "Welcome 👋!" }
        }
        blockquote {
            p {
                "Rust is a "
                em { "biblically accurate" }
                " programming language 😇!"
            }
        }
        p {
            "Rust is a modern and powerful programming language that prioritizes performance and memory safety without sacrificing control or efficiency. It's an open-source language that's been steadily gaining popularity because of its ability to solve some of the most difficult problems that we as developers face, particularly when it comes to writing code that's both fast and safe. Unlike some older languages, e.g. C, that allow you to manipulate memory directly, which can lead to bugs and vulnerabilities if done incorrectly, Rust enforces strict safety rules at compile time. This means that many of the issues that would typically appear during runtime - such as memory leaks or data races - are caught early in the development process, even before the program starts running. The trade-off is that, while Rust is designed to help you write safer and more reliable code, it also requires you to think more deeply about how data is used and manipulated in your programs. This makes it a little harder to learn initially, especially if you come from a background in languages like Python, Ruby, or JavaScript, but once you grasp its concepts, you'll find that it can actually make your code cleaner and easier to maintain. Rust's combination of speed, safety, and concurrency makes it an ideal choice for a wide variety of applications, from operating systems and web servers to game engines and blockchain technology. Rust is particularly useful when building systems that require high performance without sacrificing safety or concurrency. It achieves this balance by using a set of concepts that, while initially tricky to learn, become second nature over time."
        }
        hr {}
        h2 { id: "table-of-contents",
            a { href: "#table-of-contents", class: "header", "Table of Contents" }
        }
        ul {
            li {
                a { href: "#who-is-this-article-for", "Who Is This Article For?" }
            }
            li {
                a { href: "#what-is-rust", "What Is Rust?" }
            }
            li {
                a { href: "#why-not-rust", "Why Not Rust?" }
            }
            li {
                a { href: "#personal-experience", "Personal Experience" }
            }
            li {
                a { href: "#why-i-switched-to-rust", "Why I Switched to Rust" }
            }
            li {
                a { href: "#how-to-get-started", "How To Get Started" }
            }
            li {
                a { href: "#installing-rust", "Installing Rust" }
            }
            li {
                a { href: "#verifying-rust-installation", "Verifying Rust Installation" }
            }
            li {
                a { href: "#creating-a-rust-project", "Creating a Rust Project" }
            }
            li {
                a { href: "#linters-and-formatters", "Linters and Formatters" }
            }
            li {
                a { href: "#basic-rust-concepts", "Basic Rust Concepts" }
            }
            li {
                a { href: "#variables", "Variables" }
            }
            li {
                a { href: "#constants", "Constants" }
            }
            li {
                a { href: "#functions", "Functions" }
            }
            li {
                a { href: "#assertions-and-the-assert-macro-in-rust",
                    "Assertions and the  "
                    code { "assert!" }
                    " Macro in Rust"
                }
            }
            li {
                a { href: "#the-let-keyword-and-type-inference",
                    "The  "
                    code { "let" }
                    " Keyword and Type Inference"
                }
            }
            li {
                a { href: "#if-statements-and-control-flow",
                    code { "if" }
                    " Statements and Control Flow"
                }
            }
            li {
                a { href: "#enums-in-rust", "Enums in Rust" }
            }
            li {
                a { href: "#rust-generics", "Rust Generics" }
            }
            li {
                a { href: "#optional-enums-optiont",
                    "Optional Enums ( "
                    code { "Option<T>" }
                    ")"
                }
            }
            li {
                a { href: "#ownership", "Ownership" }
            }
            li {
                a { href: "#borrowing", "Borrowing" }
            }
            li {
                a { href: "#lifetimes", "Lifetimes" }
            }
            li {
                a { href: "#wrap-up", "Wrap Up" }
            }
        }
        hr {}
        h2 { id: "who-is-this-article-for",
            a { href: "#who-is-this-article-for", class: "header", "Who Is This Article For?" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "This article is primarily intended for those who have some experience with programming and are looking to learn a new language or dive deeper into systems-level programming. If you have a solid understanding of concepts in languages like C, C++, or Java, but you've never worked with Rust before, then this article will help you understand why Rust is becoming increasingly popular in the software development world. On the other hand, if you are coming from higher-level languages like Python, JavaScript, or Ruby, and you are curious about learning something that gives you more control over how your program interacts with the hardware, then this article is a perfect starting point. Understanding Rust can be incredibly rewarding because it helps you write code that is both fast and safe at the same time. However, Rust has a steep learning curve, especially if you've never dealt with concepts like ownership, borrowing, and lifetimes, which are central to its design. Rust's focus on memory safety and concurrency makes it an excellent choice for building complex applications that require both speed and reliability, such as game engines, web servers, and embedded systems. This article assumes that you have at least some basic familiarity with programming concepts, such as variables, loops, and functions, but it will guide you through everything you need to know to start writing Rust code. Even if you're new to systems programming or low-level languages, the principles and patterns used in Rust are incredibly valuable for all kinds of software development. The goal here is not to overwhelm you with every single detail but to provide you with the foundational knowledge and understanding to begin writing Rust code that is safe, efficient, and maintainable."
        }
        h2 { id: "what-is-rust",
            a { href: "#what-is-rust", class: "header", "What Is Rust?" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            img {
                src: "https://github.com/user-attachments/assets/4b420eb4-8e9d-42ad-ac79-5c8e08c7e0f3",
                alt: "A compiled programming language, waku waku!",
                title: "",
            }
        }
        p {
            "Rust is a systems programming language that focuses on performance, reliability, and safety. It was created by Mozilla Research in 2006 and later developed by the Rust community. Its primary goal is to make it easier to write safe and concurrent programs without sacrificing performance. One of the key differentiators of Rust from other languages is its ownership system, which helps prevent bugs related to memory access, such as null pointer dereferencing or data races, both of which can lead to crashes or vulnerabilities in software. Rust achieves this by introducing a set of rules around "
            strong { "ownership" }
            ", "
            strong { "borrowing" }
            ", and "
            strong { "lifetime" }
            " of data. In Rust, data has an "
            strong { "owner" }
            ", and each piece of data can have only one owner at a time. When the owner of data goes out of scope, the data is automatically cleaned up, preventing memory leaks. This ownership system, while initially challenging to understand, is one of the reasons why Rust can prevent many common programming bugs that plague other languages. Additionally, Rust's borrowing system allows data to be temporarily shared without giving up ownership, which makes it possible to write concurrent programs safely. These concepts help ensure that your programs are free from memory-related errors without the need for a garbage collector, which is common in languages like Java or Python. Rust also excels at "
            strong { "concurrency" }
            ", meaning that it can handle multiple threads of execution simultaneously without the risk of data races, thanks to its strict rules for data access. This makes it ideal for building highly efficient and reliable systems that require heavy parallel processing, such as game engines or data processing pipelines. As a compiled language, Rust offers performance comparable to C and C++ while ensuring memory safety, which is something those older languages do not inherently guarantee."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> s1 </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::from(</span><span style=\"color:#ffee99;\">&quot;hello&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> s2 </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> s1; </span><span style=\"color:#8c8c8c;\">// Ownership of s1 is transferred to s2\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// println!(&quot;{{}}&quot;, s1);  // This would throw an error because s1 is no longer valid\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, s2);  </span><span style=\"color:#8c8c8c;\">// This prints &quot;hello&quot;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In the code example above, you can see Rust's ownership in action. The string  "
            code { "s1" }
            " is created, and then its ownership is transferred to  "
            code { "s2" }
            ". Once ownership has been transferred,  "
            code { "s1" }
            " is no longer valid and cannot be accessed, as shown in the commented-out line. This mechanism eliminates issues like double-free errors, which can occur in other languages when memory is deallocated more than once. Rust ensures that memory is freed at the right time, without relying on runtime checks or garbage collection. This is part of what makes Rust so efficient, as it avoids the overhead that garbage collection introduces in other languages, making it suitable for performance-critical applications."
        }
        h2 { id: "why-not-rust",
            a { href: "#why-not-rust", class: "header", "Why Not Rust?" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            img {
                src: "https://github.com/user-attachments/assets/bebd1300-f666-45ee-aedd-8dc05b501036",
                alt: "Just steap learning curve, don't mind it!",
                title: "",
            }
        }
        p {
            "While Rust offers many benefits, it's not always the best choice for every project. One of the biggest drawbacks of Rust is its steep learning curve. Unlike higher-level languages like Python or JavaScript, Rust requires a deep understanding of memory management and the ownership model to write efficient and error-free code. For those who are just starting out in programming, Rust might feel overwhelming at first. The concepts of ownership, borrowing, and lifetimes are fundamental to writing safe and correct Rust code, but they can be difficult to grasp, especially if you don't have experience with lower-level programming concepts. Additionally, Rust's syntax is not as forgiving as some other languages. Small mistakes, like forgetting to mark a variable as mutable or trying to use a value after it's been moved, can lead to compiler errors that might be frustrating for beginners. This strictness is part of what makes Rust so reliable, but it can also make the development process slower, particularly when you're starting out. Another potential downside of Rust is that, while it's a fast and powerful language, its tooling ecosystem isn't as mature as some of the more established languages like Python or JavaScript. However, we are working on improving this area at Open Sass and making it easier to use Rust on the web. Although Rust's package manager, Cargo, is highly regarded, there may still be situations where finding the right libraries or frameworks for your project is more challenging than in more widely-used languages. Rust also lacks some of the higher-level abstractions that languages like Python provide, which can make it less suitable for quick prototypes or applications that don't require high performance. As a result, Rust is best suited for situations where performance, memory safety, and concurrency are critical, such as systems programming, embedded systems, web or game development. For simpler projects or applications that don't have rigid performance requirements, a language like Python, JavaScript, or Ruby might be a better fit. However, for those willing to invest the time and effort to learn its nuances, Rust can be an incredibly rewarding and powerful tool."
        }
        h2 { id: "personal-experience",
            a { href: "#personal-experience", class: "header", "Personal Experience" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "In my personal journey with Rust, I found that the language, while initially difficult to learn, has grown to be an invaluable tool for tackling complex projects. Like many developers, I started with higher-level languages, like Python, and was initially intimidated by Rust's focus on memory safety and ownership. At first, the compiler errors felt like roadblocks, but over time, I realized that these errors were not obstacles - they were guiding me toward better, safer code. One of the most rewarding moments in learning Rust was when I finally understood the borrowing and ownership system. It clicked when I saw how it could prevent data races and memory corruption, things that had been difficult to manage in other languages without introducing complex synchronization mechanisms. After gaining proficiency in Rust, I felt more confident tackling systems-level programming tasks, like building a web server or working with low-level hardware interactions, tasks that would have been much more error-prone and dangerous in languages like C. Even though Rust is a compiled language, it has a fast feedback loop through its excellent testing and error messages, which makes the development experience feel more like working with an interpreted language. Rust's tooling, including the Cargo package manager and rustfmt for automatic formatting, has made it easy to stay organized and productive. I also found that the Rust community is incredibly supportive, with a large and active group of developers who are always eager to help newcomers. Whether you're working on small open-source projects or large, complex systems, the community provides resources, tutorials, and documentation that can help you overcome challenges quickly. Overall, Rust has made me a better programmer by forcing me to think more carefully about memory management and concurrency, two areas that were previously abstracted away in higher-level languages."
        }
        h2 { id: "why-i-switched-to-rust",
            a { href: "#why-i-switched-to-rust", class: "header", "Why I Switched to Rust" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Switching to Rust was a decision that, at first, seemed daunting but ultimately transformed the way I approached software development. Coming from a background in higher-level languages like Python and JavaScript, I was drawn to a more relaxed approach to memory management and error handling. However, as I began working on performance-critical projects, such as building a web server and optimizing algorithms, I started running into limitations with languages that abstracted away too much of the underlying system. I realized that while these languages are excellent for rapid development and prototyping, they often struggle when it comes to fine-tuning performance or ensuring reliability in large-scale, concurrent applications. This is where Rust came in. What drew me to Rust was its focus on both speed and safety. In particular, the ability to write concurrent programs without worrying about race conditions or memory corruption was a game-changer. The idea of having zero-cost abstractions, where I could write high-performance code without worrying about runtime overhead, was incredibly appealing. In addition, Rust's ownership system ensured that memory management would be handled automatically, without the need for a garbage collector or manual memory allocation. This was a massive improvement over other languages, where manual memory management often leads to bugs and security vulnerabilities. After diving deeper into Rust and understanding its core principles, I realized that it not only solved the performance issues I had been facing but also helped me write more reliable and maintainable code. I no longer had to worry about the kinds of memory errors or data races that could cause unpredictable behavior in my applications. While Rust does have a steep learning curve, the payoff was well worth it, and I quickly became a believer in its advantages. It wasn't just about the performance gains - it was about the clarity and correctness that came with the language's design. In the end, switching to Rust allowed me to approach software development with more confidence, knowing that my programs would be faster, safer, and more scalable."
        }
        h2 { id: "how-to-get-started",
            a { href: "#how-to-get-started", class: "header", "How To Get Started" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Getting started with Rust might seem intimidating, but it is actually quite straightforward once you break it down into manageable steps. The first thing you need to do is install Rust on your system. Fortunately, the Rust installation process is simple and well-documented, and the official website provides detailed instructions for various operating systems, including Windows, macOS, and Linux. The recommended way to install Rust is through the Rust toolchain installer called "
            strong { "rustup" }
            ", which handles everything from downloading the compiler to setting up the necessary components for your development environment. After installing rustup, you can begin writing Rust code with a simple text editor, but using an IDE with Rust support, such as Visual Studio Code with the Rust extension, can make your development experience smoother. This setup will allow you to take advantage of features like autocompletion, inline documentation, and error highlighting, which can help you become more productive. Once your environment is ready, you can create a new Rust project using Cargo, the built-in package manager and build system for Rust. Cargo handles all the dependencies, compilation, and testing of your projects, which simplifies the development process. When you create a new project with Cargo, it will generate the necessary directory structure, a "
            code { "Cargo.toml" }
            " file for managing dependencies, and a basic "
            code { "main.rs" }
            " file to get you started. From here, you can begin writing Rust code and testing it with Cargo's built-in testing framework. The official Rust documentation, known as "
            em { "The Rust Book" }
            ", is an invaluable resource for beginners. It provides a comprehensive guide to the language and walks you through all of the core concepts, such as variables, functions, ownership, and error handling, step by step. Additionally, there are many online tutorials, forums, and video courses that can help you along the way. The Rust community is incredibly welcoming, and there's a strong support network for newcomers, whether you're asking questions on forums or reading blog posts and books from experienced developers. As you get more comfortable with the basics, you can explore more advanced topics like concurrency, memory management, and unsafe code, which are some of Rust's most powerful features. The key to getting started with Rust is persistence and practice. The language has a steep learning curve, but the more you code, the more comfortable you will become with its unique concepts and powerful features."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> Installing Rust on macOS and Linux via rustup\n</span><span style=\"color:#f8f8f2;\">curl </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">proto </span><span style=\"color:#f92672;\">&#39;=</span><span style=\"color:#f8f8f2;\">https</span><span style=\"color:#f92672;\">&#39; --</span><span style=\"color:#f8f8f2;\">tlsv1.</span><span style=\"color:#ff80f4;\">2 </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">sSf https:</span><span style=\"color:#8c8c8c;\">//sh.rustup.rs | sh\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> Windows installation using rustup\n</span><span style=\"color:#f8f8f2;\">https:</span><span style=\"color:#8c8c8c;\">//forge.rust-lang.org/infra/other-installation-methods.html</span></pre>\n" }
        p { "This simple command installs Rust on your system using rustup." }
        h2 { id: "verifying-rust-installation",
            a { href: "#verifying-rust-installation", class: "header",
                "Verifying Rust Installation"
            }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "After installing Rust using rustup, it's important to verify that everything has been installed correctly. The verification process is simple and can be done by checking the version of the Rust compiler ( "
            code { "rustc" }
            ") and the package manager ( "
            code { "cargo" }
            ") that rustup installs. First, open a terminal or command prompt and run the following command to check the version of the Rust compiler:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rustc </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">version</span></pre>\n" }
        p {
            "This command will display the version of  "
            code { "rustc" }
            ", the Rust compiler. If the installation was successful, you should see output similar to this:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">rustc </span><span style=\"color:#ff80f4;\">1.85</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">(4eb161250 </span><span style=\"color:#ff80f4;\">2025</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">03</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">15</span><span style=\"color:#f8f8f2;\">)</span></pre>\n" }
        p {
            "This confirms that the Rust compiler is installed and that it's ready to use. Additionally, you can verify that Cargo, Rust's package manager, is installed by running the following command:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">version\n</span><span style=\"color:#f92672;\">#</span><span style=\"color:#f8f8f2;\"> cargo </span><span style=\"color:#ff80f4;\">1.85</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#ff80f4;\">1 </span><span style=\"color:#f8f8f2;\">(d73d2caf9 </span><span style=\"color:#ff80f4;\">2024</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">12</span><span style=\"color:#f92672;\">-</span><span style=\"color:#ff80f4;\">31</span><span style=\"color:#f8f8f2;\">)</span></pre>\n" }
        p {
            "This will display the version of Cargo, which should be installed automatically when you install rustup. Cargo is an essential tool for managing dependencies, building projects, and running tests, so it's important to make sure that it's working correctly. If you encounter any issues during installation or verification, the Rust website provides troubleshooting steps, and the community is active in helping resolve common issues. Once you've confirmed that both  "
            code { "rustc" }
            " and  "
            code { "cargo" }
            " are installed, you're ready to start building Rust projects!"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">h</span></pre>\n" }
        h2 { id: "creating-a-rust-project",
            a { href: "#creating-a-rust-project", class: "header", "Creating a Rust Project" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Once you have successfully installed Rust and verified that everything is working correctly, the next step is to create your first Rust project. This is done using Cargo, Rust's powerful package manager and build system. Cargo makes it easy to manage your Rust projects, handle dependencies, and compile your code into executable binaries. To create a new Rust project, simply run the following command in your terminal:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo new my_first_project</span></pre>\n" }
        p {
            "This will create a new directory called  "
            code { "my_first_project" }
            " with a basic Rust project structure. The directory will contain a  "
            code { "Cargo.toml" }
            " file, which is used to manage your project's dependencies, and a  "
            code { "src" }
            " folder with a  "
            code { "main.rs" }
            " file. The  "
            code { "main.rs" }
            " file contains a simple \"Hello, World!\" program to get you started. Inside  "
            code { "main.rs" }
            ", you will find the following code:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;Hello, world!&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "This is a very basic program that prints \"Hello, world!\" to the console when executed. To run your new project, navigate to the project directory and execute the following command:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo run</span></pre>\n" }
        p {
            "This will compile your program and execute it, displaying the output in your terminal. Cargo automatically handles compiling and running your program for you, making it much easier to get started with Rust development. If you want to build the project without running it, you can use the  "
            code { "cargo build" }
            " command, which will generate an executable file in the  "
            code { "target/debug" }
            " directory. As you progress with your Rust development, you can modify the  "
            code { "main.rs" }
            " file to add more functionality, or you can create additional files and modules to structure your code."
        }
        h2 { id: "linters-and-formatters",
            a { href: "#linters-and-formatters", class: "header", "Linters and Formatters" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Linters and formatters are essential tools in Rust for ensuring that your code adheres to best practices and remains consistent throughout the development process. A linter is a tool that analyzes your code for potential errors, stylistic issues, or patterns that may lead to bugs. Rust's ecosystem provides several linters, with the most commonly used one being  "
            code { "clippy" }
            ".  "
            code { "Clippy" }
            " is a powerful linter that checks for a wide variety of potential mistakes and bad practices, helping you write cleaner, more efficient, and more idiomatic Rust code. When you run  "
            code { "clippy" }
            ", it performs a detailed static analysis of your Rust code, checking for common mistakes, like unnecessary allocations, non-idiomatic patterns, or code that may not be as efficient as it could be. It can also suggest more idiomatic solutions to problems, which can help you follow the best practices for writing Rust code. Here's a simple example of running  "
            code { "clippy" }
            " on your project:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo clippy</span></pre>\n" }
        p {
            "This command runs  "
            code { "clippy" }
            " on the current project, analyzing your code for potential issues. The tool will output warnings and suggestions, helping you identify where improvements can be made. For instance,  "
            code { "clippy" }
            " might suggest replacing a  "
            code { "for" }
            " loop with an iterator method to make the code more idiomatic and performant. This process helps us avoid many common pitfalls and ensures our code is optimized for both performance and clarity. In addition to  "
            code { "clippy" }
            ", the Rust ecosystem provides a code formatter called  "
            code { "rustfmt" }
            ".  "
            code { "Rustfmt" }
            " automatically formats your code to ensure consistent indentation, spacing, and line length according to Rust's style guidelines. Formatting your code is an important aspect of teamwork, as it ensures that everyone on a project follows the same conventions, making it easier to read, maintain, and contribute to the codebase. When you run  "
            code { "rustfmt" }
            ", it automatically formats your Rust files, ensuring that they conform to the official style guide. To format your code with  "
            code { "rustfmt" }
            ", you simply run:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo fmt</span></pre>\n" }
        p {
            "This command formats all the Rust files in the current project.  "
            code { "rustfmt" }
            " is highly customizable, allowing you to configure various options, such as line length and indentation style, to match the specific needs of your project or team. By using both  "
            code { "clippy" }
            " and  "
            code { "rustfmt" }
            ", we can significantly improve the quality of our code, ensuring that it is both correct and maintainable. This process not only helps avoid common mistakes but also promotes consistency and readability across the entire codebase. The use of linters and formatters is a crucial part of Rust's emphasis on safety and quality, helping us write code that is not only functional but also clean and easy to work with. In large codebases or team environments, using these tools ensures that everyone is on the same page, reducing friction and enhancing collaboration. It is also worth noting that both  "
            code { "clippy" }
            " and  "
            code { "rustfmt" }
            " are often integrated into development workflows and CI/CD pipelines, so code quality is automatically maintained throughout the development process."
        }
        h2 { id: "basic-rust-concepts",
            a { href: "#basic-rust-concepts", class: "header", "Basic Rust Concepts" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Rust is a systems programming language that emphasizes memory safety, concurrency, and performance. Its core concepts revolve around ownership, borrowing, and lifetimes, which are designed to eliminate common bugs that other programming languages struggle with, such as data races, null pointer dereferencing, and memory leaks. Understanding these concepts is critical to becoming proficient in Rust. The concept of ownership in Rust means that each value in the program has a single owner, and the ownership can be transferred (moved) or borrowed. This ownership model ensures that there is exactly one owner of each value at any given time, making it impossible for multiple parts of the program to accidentally modify or access the same data concurrently. The rules of ownership are enforced at compile-time, which means that many common errors are caught before the program ever runs. Rust's ownership system is tightly integrated with its borrowing and lifetime mechanisms, which together form the foundation of memory safety in the language."
        }
        p {
            "Borrowing allows one part of a program to temporarily access data without taking ownership of it. Rust distinguishes between two types of borrowing: immutable borrowing, where the data cannot be modified, and mutable borrowing, where the data can be modified. The borrow checker ensures that there is either only one mutable reference or multiple immutable references to a piece of data at any given time. This prevents data races and ensures that data is accessed safely across different threads or parts of the program. The concept of lifetimes is closely tied to borrowing, as it describes how long references to data are valid. Lifetimes are a way of ensuring that references don't outlive the data they point to, which prevents use-after-free errors and dangling pointers, common issues in other languages. Together, ownership, borrowing, and lifetimes enable Rust to offer memory safety guarantees without the need for a garbage collector, making it both efficient and reliable for building high-performance systems."
        }
        p {
            "Rust also introduces the concept of pattern matching, which allows you to work with data in a concise and expressive way. Pattern matching in Rust is an incredibly powerful tool, enabling is to destructure data types, check conditions, and perform actions based on the structure and values of data. The  "
            code { "match" }
            " statement in Rust is a form of pattern matching that is used extensively throughout the language. It allows you to match against various possible values of an enum, struct, or other types, providing a clean and expressive way to handle different cases. Here's an example of using  "
            code { "match" }
            " with an enum:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Direction {{\n</span><span style=\"color:#f8f8f2;\">    Up,\n</span><span style=\"color:#f8f8f2;\">    Down,\n</span><span style=\"color:#f8f8f2;\">    Left,\n</span><span style=\"color:#f8f8f2;\">    Right,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">move_player</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">direction</span><span style=\"color:#f8f8f2;\">: Direction) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> direction {{\n</span><span style=\"color:#f8f8f2;\">        Direction::Up </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Moving up&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">        Direction::Down </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Moving down&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">        Direction::Left </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Moving left&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">        Direction::Right </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Moving right&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In this example, the  "
            code { "match" }
            " statement checks the value of the  "
            code { "direction" }
            " variable and executes the corresponding branch. Pattern matching helps reduce boilerplate code, making your programs more concise and easier to maintain. Along with  "
            code { "match" }
            ", Rust's  "
            code { "if let" }
            " and  "
            code { "while let" }
            " constructs provide further ways to destructure and handle data in a more flexible and readable manner. The language's emphasis on pattern matching makes it an excellent choice for writing concise, expressive, and error-free code."
        }
        h2 { id: "variables",
            a { href: "#variables", class: "header", "Variables" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "In Rust, variables are immutable by default, which means that once a value is assigned to a variable, it cannot be changed. This immutability is a key feature of Rust's design, as it promotes safety and predictability in code. When you declare a variable, Rust ensures that the value cannot be modified unless you explicitly declare it as mutable. This default immutability makes it easier to reason about how data is used in a program, as you don't have to worry about unexpected modifications happening elsewhere in the code. To declare a mutable variable, you use the  "
            code { "mut" }
            " keyword:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">5</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">x </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">6</span><span style=\"color:#f8f8f2;\">;</span></pre>\n" }
        p {
            "In this case,  "
            code { "x" }
            " is mutable, meaning that its value can be changed. The  "
            code { "let mut" }
            " syntax indicates that  "
            code { "x" }
            " can be reassigned, and this is the only way to change a variable's value in Rust. If you try to modify an immutable variable, the Rust compiler will produce an error, ensuring that you don't unintentionally modify data. The immutability of variables in Rust encourages developers to use data in a more controlled and predictable way. It reduces the risk of bugs related to data being changed unexpectedly in other parts of the program, making the code more maintainable and reliable. However, if you need to modify a value, Rust provides the flexibility of using mutable variables, which are necessary for many situations, such as when managing state in a program."
        }
        p {
            "Rust also offers the concept of shadowing, where you can declare a new variable with the same name as an existing one, effectively replacing it. This can be useful when you want to change the value of a variable but also keep the original type. Shadowing allows for reassignment of the variable without the need for  "
            code { "mut" }
            ", and it's important to note that shadowing does not mutate the original variable; instead, it creates a new binding. Here's an example of shadowing:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">5</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;</span></pre>\n" }
        p {
            "In this case, the second  "
            code { "x" }
            " shadows the first  "
            code { "x" }
            ", and the value of  "
            code { "x" }
            " is incremented by one. The original  "
            code { "x" }
            " is no longer accessible, and the new variable takes its place. Shadowing can be useful when transforming data through different stages or scopes, as it allows you to reuse variable names without worrying about modifying the original value."
        }
        h2 { id: "constants",
            a { href: "#constants", class: "header", "Constants" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Constants in Rust are similar to variables, but with a key difference: they are always immutable and their values are fixed at compile time. Constants are declared using the  "
            code { "const" }
            " keyword, and they must always have an explicit type annotation. Unlike regular variables, constants can be used in contexts where a variable would not be allowed, such as array sizes or static memory locations. Constants are evaluated at compile time, which makes them more efficient than regular variables in certain cases. For example, a constant can be used to define a value that is needed across the entire program, like a fixed configuration value or a mathematical constant, and it will not take up memory at runtime."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">const </span><span style=\"color:#ff80f4;\">MAX_POINTS</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">u32 </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">1_000_000</span><span style=\"color:#f8f8f2;\">;</span></pre>\n" }
        p {
            "Here,  "
            code { "MAX_POINTS" }
            " is a constant that cannot be changed during the program's execution, and its value is available for use anywhere in the program. Constants are particularly useful for defining values that will remain the same throughout the program's lifetime, such as mathematical constants like PI, conversion factors, or predefined configuration values. Since constants are evaluated at compile time, they provide performance benefits by reducing runtime overhead and ensuring that the values they represent are fixed before the program starts running. Another key advantage of constants is that they are available in both the global and local scopes, making them versatile and easy to work with."
        }
        p {
            "Constants can be used in expressions, just like regular variables, and are available for use in all places where a constant value is needed, such as array sizes or memory allocation. The fact that they are immutable and evaluated at compile time ensures that they are always safe to use and will never result in unexpected behavior. Constants in Rust provide a level of predictability and stability to your program, making them an excellent choice for handling fixed values that do not change throughout the execution of the program. They also contribute to Rust's strong type system, ensuring that values are explicitly typed and used consistently across the codebase."
        }
        h2 { id: "functions",
            a { href: "#functions", class: "header", "Functions" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Functions in Rust are used to encapsulate logic that can be reused throughout a program. Functions allow you to break down a complex problem into smaller, manageable pieces, promoting modularity and reusability in your code. Rust functions are declared using the  "
            code { "fn" }
            " keyword, followed by the function name, parameters, and return type. Functions can take any number of parameters and can return a value, though returning a value is optional. In Rust, every function has a clear and explicit return type, and the compiler enforces type safety by ensuring that the return type matches the expected type. Here's an example of a simple function:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">add</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">x</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">y</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    x </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> y\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "This function takes two parameters,  "
            code { "x" }
            " and  "
            code { "y" }
            ", both of type  "
            code { "i32" }
            ", and returns their sum. The  "
            code { "-> i32" }
            " part specifies that the function will return a value of type  "
            code { "i32" }
            ". Functions in Rust are very flexible, and you can define functions that don't return anything, in which case the return type is  "
            code { "()" }
            ", which is similar to  "
            code { "void" }
            " in other languages. Functions can also accept default values for their parameters, making them more flexible and reducing the need for multiple function overloads. This is particularly useful when you want to provide reasonable defaults while still allowing for customization when necessary."
        }
        p {
            "Rust also supports closures, which are anonymous functions that can capture variables from their surrounding environment. Closures are particularly useful for passing short-lived functions as arguments to higher-order functions, and they can capture values either by reference or by value. Here's an example of a closure that captures a variable from its environment:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#a6e22e;\">add_one </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">|</span><span style=\"font-style:italic;color:#fd971f;\">x</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">| x </span><span style=\"color:#f92672;\">+ </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#66d9ef;\">add_one</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">5</span><span style=\"color:#f8f8f2;\">));  </span><span style=\"color:#8c8c8c;\">// Prints 6</span></pre>\n",
        }
        p {
            "Closures in Rust are versatile and can be used in many situations, such as when implementing callbacks or when performing short tasks that don't warrant a separate function declaration. Rust's functional programming capabilities, combined with its ownership and borrowing model, allow closures to be used safely and efficiently."
        }
        h2 { id: "assertions-and-the",
            a { href: "#assertions-and-the", class: "header", "Assertions and the " }
            code { "assert!" }
            " Macro in Rust"
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "In Rust, performing checks on the correctness of our code is often essential, particularly when we want to validate certain assumptions or ensure that specific conditions hold true during execution. One of the key tools provided by the Rust language for such checks is the  "
            code { "assert!" }
            " macro. The  "
            code { "assert!" }
            " macro allows us to assert that a given condition evaluates to  "
            code { "true" }
            ". If the condition is false, the program will panic and terminate immediately, thereby signaling an issue that needs to be addressed. This mechanism is especially useful during development and debugging phases, where catching errors early on can prevent more serious issues later."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">divide</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">numerator</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">f64</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">denominator</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">f64</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">f64 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    assert!(denominator </span><span style=\"color:#f92672;\">!= </span><span style=\"color:#ff80f4;\">0.0</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Denominator must not be zero!&quot;</span><span style=\"color:#f8f8f2;\">); </span><span style=\"color:#8c8c8c;\">// Assertion\n</span><span style=\"color:#f8f8f2;\">    numerator </span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\"> denominator\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In the code snippet above, we are using the  "
            code { "assert!" }
            " macro to ensure that the denominator is not zero before performing a division. If the  "
            code { "denominator" }
            " is zero, the program will panic with the provided error message, which serves as a clear indicator of the problem. This feature is particularly valuable in preventing division by zero errors, which, without proper checks, could lead to undefined behavior or runtime exceptions that are difficult to debug."
        }
        p {
            "Rust's assertion mechanism is advantageous because it enables us to catch errors early in the development cycle. Assertions act as a safety net to verify that the code behaves as expected, especially before the program reaches production, where errors can lead to significant performance or reliability issues. By using  "
            code { "assert!" }
            ", we can ensure that assumptions about their program are validated during runtime, reducing the chances of introducing bugs into the final codebase."
        }
        h2 { id: "the",
            a { href: "#the", class: "header", "The " }
            code { "let" }
            " Keyword and Type Inference"
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Rust is a systems programming language designed with a strong emphasis on safety, performance, and concurrency. One of its goals is to make the development experience as productive as possible while maintaining a high level of performance. One of the ways Rust achieves this is through type inference, which allows us to write cleaner, more concise code without sacrificing safety or clarity."
        }
        p {
            "When you declare a variable in Rust using the  "
            code { "let" }
            " keyword, the compiler automatically infers the type of the variable based on the value assigned to it. This reduces the need for explicitly specifying the type in many cases, allowing for more flexible and readable code. However, we still have the option to specify the type explicitly when necessary, especially in cases where ambiguity might arise or for more complex data structures."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> fraction </span><span style=\"color:#f92672;\">= </span><span style=\"color:#ff80f4;\">3.0</span><span style=\"color:#f8f8f2;\">; </span><span style=\"color:#8c8c8c;\">// Rust infers that `fraction` is of type f64\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;Fraction value: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, fraction);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "In this example, the variable  "
            code { "fraction" }
            " is assigned the value  "
            code { "3.0" }
            ", which is a floating-point number. Because of Rust's type inference system, the type of  "
            code { "fraction" }
            " is automatically inferred to be  "
            code { "f64" }
            " (a 64-bit floating-point number). The compiler determines this based on the fact that  "
            code { "3.0" }
            " is a floating-point literal. This eliminates the need for us to manually specify  "
            code { "f64" }
            ", making the code more concise and easier to maintain."
        }
        p {
            "While Rust's type inference system is highly effective and handles most cases, there are scenarios where we might want to explicitly specify the type, such as when dealing with complex data structures, generics, or ambiguous situations. In these cases, the explicit type declaration helps ensure clarity and prevents unintended behavior due to type mismatches."
        }
        h3 { id: "functions-and-returning-values",
            a { href: "#functions-and-returning-values", class: "header",
                "Functions and Returning Values"
            }
        }
        p {
            "In Rust, functions are an essential part of structuring code and enabling code reuse. Functions are defined using the  "
            code { "fn" }
            " keyword, followed by the function name and its parameters enclosed in parentheses. The return type of a function is specified using the  "
            code { "->" }
            " syntax, but it is not always necessary to specify it explicitly. Rust has a unique approach to function returns: if a function returns a value, the value is typically the result of the last expression in the function body. Rust does not require an explicit  "
            code { "return" }
            " keyword to return values, though it can still be used for clarity or to break early from a function."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">square</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">x</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">i32 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    x </span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#8c8c8c;\">// The last expression is implicitly returned\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "In this example, the function  "
            code { "square" }
            " takes an integer  "
            code { "x" }
            " and returns its square by multiplying  "
            code { "x" }
            " by itself. Since  "
            code { "x * x" }
            " is the last expression in the function body, Rust automatically returns this value, making the code clean and concise. The  "
            code { "return" }
            " keyword is not needed because Rust implicitly returns the result of the last expression in the function body. This approach is a key feature of Rust's emphasis on simplicity and efficiency."
        }
        p {
            "By eliminating the need for an explicit  "
            code { "return" }
            " keyword, Rust helps reduce boilerplate code while still maintaining clarity. This results in more compact and readable functions, while also enforcing Rust's focus on performance by reducing unnecessary overhead in the code."
        }
        h3 { id: "rusts-type-system",
            a { href: "#rusts-type-system", class: "header", "Rust's Type System" }
        }
        p {
            "Rust's type system is one of its defining features, ensuring that errors related to data types are caught at compile time rather than runtime. This strong and static type system makes Rust a safer and more predictable language for building systems-level software. The system ensures that data is handled appropriately by enforcing strict typing rules, preventing issues such as null pointer dereferencing or type mismatches, which are common in languages with weaker typing systems."
        }
        p { "Rust's type system is composed of two major categories: scalar types and compound types." }
        h3 { id: "scalar-types",
            a { href: "#scalar-types", class: "header", "Scalar Types:" }
        }
        ul {
            li {
                strong { "Integers" }
                ": Represent whole numbers, such as "
                code { "i32" }
                " (signed 32-bit integer) or "
                code { "u64" }
                " (unsigned 64-bit integer)."
            }
            li {
                strong { "Floating-Point Numbers" }
                ": Represent real numbers with decimal points, using types like "
                code { "f32" }
                " (32-bit floating-point number) and "
                code { "f64" }
                " (64-bit floating-point number)."
            }
            li {
                strong { "Booleans" }
                ": Represent truth values, either "
                code { "true" }
                " or "
                code { "false" }
                ", using the "
                code { "bool" }
                " type."
            }
        }
        h3 { id: "compound-types",
            a { href: "#compound-types", class: "header", "Compound Types:" }
        }
        ul {
            li {
                strong { "Tuples" }
                ": A collection of values that can be of different types. Tuples allow for grouping related data together in a single unit."
            }
            li {
                strong { "Arrays and Vectors" }
                ": Collections of values of the same type. Arrays have a fixed size, while vectors can grow or shrink dynamically."
            }
        }
        p {
            "Rust's type system prevents a wide variety of errors by ensuring that every value has a well-defined type and that operations on these values are type-safe. The compiler checks these types at compile time, meaning that we can catch and fix potential issues before the program is run. This is a significant advantage over dynamically typed languages, where many errors are only detected during runtime, making debugging more challenging."
        }
        h2 { id: "",
            a { href: "#", class: "header", "" }
            code { "if" }
            " Statements and Control Flow"
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Control flow is fundamental in programming, and Rust provides familiar constructs for making decisions based on conditions. One of the most commonly used control flow structures is the  "
            code { "if" }
            " statement, which allows us to execute different blocks of code depending on whether a condition is true or false."
        }
        p {
            "Rust's  "
            code { "if" }
            " statement follows a straightforward syntax:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> condition {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Code to execute if condition is true\n</span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// Code to execute if condition is false\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "The  "
            code { "if" }
            " statement evaluates the condition first. If the condition evaluates to  "
            code { "true" }
            ", the code inside the block executes. If the condition is  "
            code { "false" }
            ", the code in the  "
            code { "else" }
            " block runs. Rust also supports the use of  "
            code { "else if" }
            " to handle multiple conditions."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">classify_number</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">num</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> num </span><span style=\"color:#f92672;\">&gt; </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        println!(</span><span style=\"color:#ffee99;\">&quot;Positive&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">else if</span><span style=\"color:#f8f8f2;\"> num </span><span style=\"color:#f92672;\">&lt; </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        println!(</span><span style=\"color:#ffee99;\">&quot;Negative&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        println!(</span><span style=\"color:#ffee99;\">&quot;Zero&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In this example, the function  "
            code { "classify_number" }
            " checks whether the input number  "
            code { "num" }
            " is positive, negative, or zero, and prints the corresponding result. The use of  "
            code { "if" }
            ",  "
            code { "else if" }
            ", and  "
            code { "else" }
            " allows for clear and structured decision-making. This provides an efficient way to implement branching logic in your programs, ensuring that code execution follows the correct path based on specific conditions."
        }
        h2 { id: "enums-in-rust",
            a { href: "#enums-in-rust", class: "header", "Enums in Rust" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Rust's  "
            code { "enum" }
            " types are a powerful feature that allows us to define types that can represent multiple different variants, each potentially holding different types of data. Unlike enums in languages like C or C++, where each variant is a simple label, Rust's enums can carry associated data with each variant, making them much more flexible and expressive."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Hobbies {{\n</span><span style=\"color:#f8f8f2;\">    Coding,\n</span><span style=\"color:#f8f8f2;\">    Reading,\n</span><span style=\"color:#f8f8f2;\">    Hiking,\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "Here, the  "
            code { "Hobbies" }
            " enum defines three variants:  "
            code { "Coding" }
            ",  "
            code { "Reading" }
            ", and  "
            code { "Hiking" }
            ". These variants don't contain any associated data, but Rust allows each variant to hold data if necessary."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">ArithmeticOperation {{\n</span><span style=\"color:#f8f8f2;\">    Add(</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    Subtract(</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    Division(</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In this case, the  "
            code { "ArithmeticOperation" }
            " enum defines three arithmetic operations:  "
            code { "Add" }
            ",  "
            code { "Subtract" }
            ", and  "
            code { "Division" }
            ", each of which holds two  "
            code { "i64" }
            " values. This allows for a more complex and flexible representation of arithmetic operations, enabling us to handle different operations in a single, unified type."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">evaluate_operation</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">operation</span><span style=\"color:#f8f8f2;\">: ArithmeticOperation) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">i64 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> operation {{\n</span><span style=\"color:#f8f8f2;\">        ArithmeticOperation::Add(x, y) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#f92672;\">+</span><span style=\"color:#f8f8f2;\"> y,\n</span><span style=\"color:#f8f8f2;\">        ArithmeticOperation::Subtract(x, y) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\"> y,\n</span><span style=\"color:#f8f8f2;\">        ArithmeticOperation::Division(x, y) </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> x </span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\"> y,\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> add_operation </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">ArithmeticOperation::Add(</span><span style=\"color:#ff80f4;\">5</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">3</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> result </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">evaluate_operation</span><span style=\"color:#f8f8f2;\">(add_operation);\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;Result: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, result); </span><span style=\"color:#8c8c8c;\">// Output: Result: 8\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This example demonstrates how pattern matching works with enums. The  "
            code { "match" }
            " expression allows us to destructure the  "
            code { "ArithmeticOperation" }
            " enum and perform the appropriate action based on the variant. Pattern matching is a powerful tool in Rust and makes working with enums straightforward and intuitive."
        }
        h2 { id: "rust-generics",
            a { href: "#rust-generics", class: "header", "Rust Generics" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Generics are a core feature of Rust, allowing developers to write flexible and reusable code while maintaining type safety. With generics, we can create functions, structs, and enums that work with multiple data types without duplicating code or losing the benefits of Rust's strong typing system."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">max</span><span style=\"color:#f8f8f2;\">&lt;T: PartialOrd&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">a</span><span style=\"color:#f8f8f2;\">: T, </span><span style=\"font-style:italic;color:#fd971f;\">b</span><span style=\"color:#f8f8f2;\">: T) -&gt; T {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> a </span><span style=\"color:#f92672;\">&gt;=</span><span style=\"color:#f8f8f2;\"> b {{\n</span><span style=\"color:#f8f8f2;\">        a\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        b\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "In this example, the  "
            code { "max" }
            " function is generic and can accept any type  "
            code { "T" }
            " that implements the  "
            code { "PartialOrd" }
            " trait, which is required for types that can be compared and partially ordered (e.g., NaN cannot be ordered). This allows the function to compare two values of any type that supports ordering, providing a reusable solution that works for a wide variety of data types."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> int_result </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">max</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">5</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">10</span><span style=\"color:#f8f8f2;\">); </span><span style=\"color:#8c8c8c;\">// T is i32\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> float_result </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">max</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">3.4</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">2.1</span><span style=\"color:#f8f8f2;\">); </span><span style=\"color:#8c8c8c;\">// T is f64\n</span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Max integer: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, int_result);\n</span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Max float: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, float_result);</span></pre>\n",
        }
        p {
            "Here, we demonstrate how the  "
            code { "max" }
            " function can work with different types of data. By leveraging generics, we avoid the need to write separate  "
            code { "max" }
            " functions for  "
            code { "i32" }
            ",  "
            code { "f64" }
            ", or any other type, making the code more maintainable and reusable. Rust's generics system provides a powerful and flexible way to write type-safe code that can work with any data type."
        }
        h2 { id: "optional-enums-",
            a { href: "#optional-enums-", class: "header", "Optional Enums (" }
            code { "Option<T>" }
            ")"
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Rust's  "
            code { "Option<T>" }
            " enum is one of its most powerful and widely-used features. It represents an optional value - either a value of type  "
            code { "T" }
            " or nothing at all ( "
            code { "None" }
            "). This is particularly useful for situations where a value may be missing, such as when querying a database or performing a search."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Option&lt;T&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(T),\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "The  "
            code { "Option<T>" }
            " enum has two variants:  "
            code { "Some" }
            ", which holds a value of type  "
            code { "T" }
            ", and  "
            code { "None" }
            ", which represents the absence of a value. This enum is commonly used for error handling and dealing with optional data."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">divide</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">numerator</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">, </span><span style=\"font-style:italic;color:#fd971f;\">denominator</span><span style=\"color:#f8f8f2;\">: </span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">) -&gt; Option&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">i64</span><span style=\"color:#f8f8f2;\">&gt; {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">if</span><span style=\"color:#f8f8f2;\"> denominator </span><span style=\"color:#f92672;\">== </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#8c8c8c;\">// Return None if the denominator is zero\n</span><span style=\"color:#f8f8f2;\">    }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(numerator </span><span style=\"color:#f92672;\">/</span><span style=\"color:#f8f8f2;\"> denominator) </span><span style=\"color:#8c8c8c;\">// Return Some with the result\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> result </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">divide</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ff80f4;\">10</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> result {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(value) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Quotient: </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, value),\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">println!(</span><span style=\"color:#ffee99;\">&quot;Cannot divide by zero&quot;</span><span style=\"color:#f8f8f2;\">),\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In this example, the  "
            code { "divide" }
            " function returns an  "
            code { "Option<i64>" }
            ". If the denominator is zero, the function returns  "
            code { "None" }
            ", signaling an error. Otherwise, it returns the quotient wrapped in  "
            code { "Some" }
            ". The  "
            code { "match" }
            " expression is used to handle both cases of the  "
            code { "Option" }
            ", ensuring that the program handles both valid and invalid scenarios in a type-safe and predictable manner. This approach is a great example of how Rust uses enums to handle errors in a way that avoids the pitfalls of null pointers or exceptions common in other languages."
        }
        h3 { id: "traits",
            a { href: "#traits", class: "header", "Traits" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Rust's type system is robust, allowing you to set constraints on the data types used in your programs. This is achieved through "
            strong { "traits" }
            ", which act as interfaces or agreements that define the behavior of a type. A trait specifies which functionalities a type must implement to conform to it."
        }
        p {
            "A commonly used trait in Rust is the  "
            code { "PartialOrd" }
            " trait. As we saw in the previous example with the  "
            code { "max" }
            " function, the  "
            code { "PartialOrd" }
            " trait defines an ordering for a type, making it possible to compare values of that type. This allows us to write functions like  "
            code { "max" }
            " and  "
            code { "min" }
            ", which work with values that can be ordered."
        }
        p {
            "In the generic  "
            code { "max" }
            " function example, we add a constraint that  "
            code { "T" }
            " must implement the  "
            code { "PartialOrd" }
            " trait. This ensures that we can compare the values correctly, as they are ordered relative to each other."
        }
        p {
            "The following is an example of a  "
            code { "while" }
            " loop that works with the  "
            code { "Iterator" }
            " type, which implements the  "
            code { "IntoIterator" }
            " trait in Rust's standard library. The  "
            code { "IntoIterator" }
            " trait converts collections like arrays into iterators, enabling the use of for-loops:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> seasons: [</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">; </span><span style=\"color:#ff80f4;\">4</span><span style=\"color:#f8f8f2;\">] </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;Winter&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Spring&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Summer&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Fall&quot;</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> iter </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> seasons.</span><span style=\"color:#66d9ef;\">into_iter</span><span style=\"color:#f8f8f2;\">();  </span><span style=\"color:#8c8c8c;\">// Convert `seasons` array into an `Iterator`\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">while </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(season) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> iter.</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">() {{  </span><span style=\"color:#8c8c8c;\">// Loop until the iterator is empty\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, season);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Alternatively, using a  "
            code { "for" }
            " loop simplifies the syntax:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> seasons: [</span><span style=\"color:#f92672;\">&amp;</span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">; </span><span style=\"color:#ff80f4;\">4</span><span style=\"color:#f8f8f2;\">] </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;Winter&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Spring&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Summer&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;Fall&quot;</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"color:#f92672;\">for</span><span style=\"color:#f8f8f2;\"> season </span><span style=\"color:#f92672;\">in</span><span style=\"color:#f8f8f2;\"> seasons.</span><span style=\"color:#66d9ef;\">into_iter</span><span style=\"color:#f8f8f2;\">() {{  </span><span style=\"color:#8c8c8c;\">// Loop through the iterator\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, season);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This demonstrates how the  "
            code { "IntoIterator" }
            " trait converts a collection into an iterator and allows us to iterate over it cleanly."
        }
        p {
            "The  "
            code { "Iterator" }
            " trait itself defines two important components:"
        }
        ol {
            li {
                strong { "Item type" }
                ": The type of value produced during iteration. For example, if iterating over a vector of integers, the "
                code { "Item" }
                " type would be "
                code { "i32" }
                "."
            }
            li {
                strong { "next method" }
                ": This method returns an "
                code { "Option<Self::Item>" }
                ", yielding either "
                code { "Some(v)" }
                " for the next value or "
                code { "None" }
                " to indicate the end of the iteration."
            }
        }
        p { "Here's how we can define a custom iterator for a vector of integers:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">struct </span><span style=\"color:#f8f8f2;\">IntegerVector {{\n</span><span style=\"color:#f8f8f2;\">    vector: Vec&lt;</span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">&gt;,\n</span><span style=\"color:#f8f8f2;\">    index: </span><span style=\"font-style:italic;color:#66d9ef;\">usize</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">impl </span><span style=\"color:#f8f8f2;\">Iterator </span><span style=\"color:#f92672;\">for </span><span style=\"color:#f8f8f2;\">IntegerVector {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">type </span><span style=\"color:#f8f8f2;\">Item </span><span style=\"color:#f92672;\">= &amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">i32</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">next</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut </span><span style=\"font-style:italic;color:#fd971f;\">self</span><span style=\"color:#f8f8f2;\">) -&gt; Option&lt;</span><span style=\"font-style:italic;color:#66d9ef;\">Self::</span><span style=\"color:#f8f8f2;\">Item&gt; {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">if </span><span style=\"color:#f8f8f2;\">self.index </span><span style=\"color:#f92672;\">&gt;= </span><span style=\"color:#f8f8f2;\">self.vector.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">None\n</span><span style=\"color:#f8f8f2;\">        }} </span><span style=\"color:#f92672;\">else </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> item </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">self.vector[self.index];\n</span><span style=\"color:#f8f8f2;\">            self.index </span><span style=\"color:#f92672;\">+= </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(item)\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "You can then use this iterator like this:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> vector </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">vec![</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#ff80f4;\">3</span><span style=\"color:#f8f8f2;\">];\n</span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> iter </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> IntegerVector {{ vector, index: </span><span style=\"color:#ff80f4;\">0 </span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">while </span><span style=\"font-style:italic;color:#66d9ef;\">let Some</span><span style=\"color:#f8f8f2;\">(val) </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> iter.</span><span style=\"color:#66d9ef;\">next</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, val);\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "This example shows how to implement and use a custom iterator, leveraging the flexibility of Rust's iterator traits."
        }
        h2 { id: "ownership",
            a { href: "#ownership", class: "header", "Ownership" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "In Rust, "
            strong { "ownership" }
            " is a central concept that ensures memory safety without the need for a garbage collector. The core rule is that each value can have only one owner at a time, which ensures that the value is freed properly when no longer needed."
        }
        p { "Consider this simple example:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> s </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::from(</span><span style=\"color:#ffee99;\">&quot;hello&quot;</span><span style=\"color:#f8f8f2;\">);  </span><span style=\"color:#8c8c8c;\">// s owns the string &quot;hello&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> t </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> s;  </span><span style=\"color:#8c8c8c;\">// t now owns the string &quot;hello&quot;\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, s);  </span><span style=\"color:#8c8c8c;\">// This won&#39;t compile because s no longer owns the string\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, t);  </span><span style=\"color:#8c8c8c;\">// This will print &quot;hello&quot;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In this example, ownership of the string is transferred from  "
            code { "s" }
            " to  "
            code { "t" }
            ". After the transfer,  "
            code { "s" }
            " is no longer valid, and any attempt to use it will result in a compile-time error. Rust's ownership model prevents issues like memory leaks and dangling pointers, making memory management more efficient."
        }
        h2 { id: "borrowing",
            a { href: "#borrowing", class: "header", "Borrowing" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            strong { "Borrowing" }
            " in Rust allows functions to access data without taking ownership, which prevents unnecessary data copies and enables safer memory access. Borrowing can be either mutable or immutable, depending on whether the data can be modified."
        }
        p {
            "Here's an example of "
            strong { "immutable borrowing" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">calculate_length</span><span style=\"color:#f8f8f2;\">(</span><span style=\"font-style:italic;color:#fd971f;\">s</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">String) -&gt; </span><span style=\"font-style:italic;color:#66d9ef;\">usize </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    s.</span><span style=\"color:#66d9ef;\">len</span><span style=\"color:#f8f8f2;\">()  </span><span style=\"color:#8c8c8c;\">// We only borrow the reference to the string\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> s </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::from(</span><span style=\"color:#ffee99;\">&quot;hello&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> len </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">calculate_length</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">s);  </span><span style=\"color:#8c8c8c;\">// Pass a reference to s\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;The length of &#39;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&#39; is </span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">.&quot;</span><span style=\"color:#f8f8f2;\">, s, len);  </span><span style=\"color:#8c8c8c;\">// s is still usable\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In this case, the  "
            code { "calculate_length" }
            " function borrows the string  "
            code { "s" }
            " without taking ownership, meaning  "
            code { "s" }
            " can still be used after the function call."
        }
        h2 { id: "lifetimes",
            a { href: "#lifetimes", class: "header", "Lifetimes" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            strong { "Lifetimes" }
            " are a way for Rust to track how long references to data are valid. They help prevent dangling references, ensuring that data is not accessed after it is no longer valid."
        }
        p { "The following is an example of a function with a lifetime annotation:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">print_string</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">s</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;&#39;a </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, s);  </span><span style=\"color:#8c8c8c;\">// Print the string\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> s </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::from(</span><span style=\"color:#ffee99;\">&quot;Hello, world!&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">print_string</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">s);  </span><span style=\"color:#8c8c8c;\">// Pass a reference to the string\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In this case, the  "
            code { "'a" }
            " lifetime annotation tells Rust that the reference  "
            code { "s" }
            " is valid for at least as long as the  "
            code { "print_string" }
            " function is using it."
        }
        p { "Lifetimes also apply to mutable references:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">mutate_string</span><span style=\"color:#f8f8f2;\">&lt;</span><span style=\"color:#f92672;\">&#39;a</span><span style=\"color:#f8f8f2;\">&gt;(</span><span style=\"font-style:italic;color:#fd971f;\">s</span><span style=\"color:#f8f8f2;\">: </span><span style=\"color:#f92672;\">&amp;&#39;a mut</span><span style=\"color:#f8f8f2;\"> String) {{\n</span><span style=\"color:#f8f8f2;\">    s.</span><span style=\"color:#66d9ef;\">push_str</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;, world!&quot;</span><span style=\"color:#f8f8f2;\">);  </span><span style=\"color:#8c8c8c;\">// Modify the string\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> s </span><span style=\"color:#f92672;\">= </span><span style=\"font-style:italic;color:#66d9ef;\">String</span><span style=\"color:#f8f8f2;\">::from(</span><span style=\"color:#ffee99;\">&quot;Hello&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#66d9ef;\">mutate_string</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">&amp;mut</span><span style=\"color:#f8f8f2;\"> s);  </span><span style=\"color:#8c8c8c;\">// Pass a mutable reference\n</span><span style=\"color:#f8f8f2;\">    println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, s);  </span><span style=\"color:#8c8c8c;\">// Prints &quot;Hello, world!&quot;\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "In this example, the mutable reference  "
            code { "s" }
            " is valid for as long as the  "
            code { "mutate_string" }
            " function uses it, ensuring the memory is safely managed."
        }
        h2 { id: "wrap-up",
            a { href: "#wrap-up", class: "header", "Wrap Up" }
        }
        p {
            strong {
                "Go to "
                a { href: "#table-of-contents", "TOC" }
            }
        }
        p {
            "Congratulations, comrade 🫡! You've successfully learned some of the core concepts in Rust, including traits, ownership, borrowing, and lifetimes. With these foundational tools in your toolkit, you're now ready to take on more advanced projects and explore more complex domains like web development, game design, or even data science. Rust's efficiency and safety features will help you write faster, more reliable code while avoiding common pitfalls like memory leaks or race conditions. The possibilities are endless, and the cosmos is your canvas!"
        }
        p {
            "So, now that you're equipped with these skills, what's next? Keep building, keep experimenting, and don't forget to make use of Rust's powerful documentation. It's there to guide you as you grow your skills."
        }
        p {
            "But hey, remember. Rust isn't just about writing code. It's about writing efficient, safe, and blazingly fast code that's built to last. As Rustaceans (that's you now!) we must carry the science of memory safety forward. So, let's keep pushing boundaries and make the web, data science, and everything in between that much better with the power of Rust!"
        }
        blockquote {
            p {
                "We are "
                em { "open sass" }
                ", babe! We are committed to driving the Rust science forward. Stay tuned for future blogs on all things Rust, WASM, and beyond. The journey doesn't end here - let's keep pushing the envelope together."
            }
        }
        blockquote {
            p {
                "Oh, and feel free to share this article with your friends, turn it into a meme, tattoo it on your CI/CD pipeline - whatever you want! Spread the Rust love far and wide. Because why not?"
            }
        }
        blockquote {
            p { "Happy coding, and remember: In Rust we trust! 🦀" }
        }
    }
}
#[component(no_case_check)]
pub fn OpensassVsShadcn() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Welcome 👋!" }
        }
        p {
            "When evaluating modern UI libraries, the market is flooded with impressive choices; But not all are built equal. "
            strong { "Shadcn UI" }
            " has earned a following among developers working primarily within the React + Tailwind + Radix ecosystem. However, its design philosophy is inherently narrow. It forces teams into a predefined stack, leaving little room for freedom, modularity, or framework independence. That's where "
            strong { "Open SASS" }
            " comes in, not just as an alternative, but as a next-generation solution built for a truly agnostic, performant, and future-ready development experience. Let's explore why "
            strong { "Open SASS" }
            " doesn't just compete; It "
            strong { "obliterates" }
            " Shadcn UI on every front."
        }
        h2 { id: "framework-and-styling-agnosticism",
            a { href: "#framework-and-styling-agnosticism", class: "header",
                "Framework and Styling Agnosticism"
            }
        }
        p {
            "One of the most obvious limitations of "
            strong { "Shadcn UI" }
            " is that it is "
            em { "hardwired" }
            " into the React ecosystem. You can't use it without installing and depending on "
            strong { "React" }
            ", "
            strong { "Radix UI" }
            ", "
            strong { "Lucide React" }
            ", and "
            strong { "Tailwind CSS" }
            ". This is fine if you're working on a Next.js project and never plan to leave that bubble. But what happens when you're building with "
            strong { "Vue" }
            ", "
            strong { "Svelte" }
            " or even a native desktop interface? You're out of luck. Although there are unofficial ShadCN components available for each of these frameworks, they are maintained by the community rather than the core team. Shadcn simply does not operate outside of React."
        }
        p {
            "To see how tightly Shadcn is bound to its stack, try installing a simple Accordion component with the following command:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">npx shadcn</span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">latest add accordion</span></pre>\n" }
        p { "The generated component looks like this:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#ffee99;\">&quot;use client&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">import </span><span style=\"color:#f92672;\">* as</span><span style=\"color:#f8f8f2;\"> React from </span><span style=\"color:#ffee99;\">&quot;react&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">import </span><span style=\"color:#f92672;\">* as</span><span style=\"color:#f8f8f2;\"> AccordionPrimitive from </span><span style=\"color:#ffee99;\">&quot;@radix-ui/react-accordion&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">import {{ ChevronDownIcon }} from </span><span style=\"color:#ffee99;\">&quot;lucide-react&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">import {{ cn }} from </span><span style=\"color:#ffee99;\">&quot;@/lib/utils&quot;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">function Accordion({{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">...</span><span style=\"color:#f8f8f2;\">props\n</span><span style=\"color:#f8f8f2;\">}}: React.ComponentProps&lt;typeof AccordionPrimitive</span><span style=\"background-color:#f92672;color:#f8f8f0;\">.</span><span style=\"color:#f8f8f2;\">Root</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">&lt;AccordionPrimitive</span><span style=\"background-color:#f92672;color:#f8f8f0;\">.</span><span style=\"color:#f8f8f2;\">Root data</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">slot</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;accordion&quot; </span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#f92672;\">...</span><span style=\"color:#f8f8f2;\">props}} </span><span style=\"color:#f92672;\">/&gt;</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">function AccordionItem({{\n</span><span style=\"color:#f8f8f2;\">  className,\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">...</span><span style=\"color:#f8f8f2;\">props\n</span><span style=\"color:#f8f8f2;\">}}: React.ComponentProps&lt;typeof AccordionPrimitive</span><span style=\"background-color:#f92672;color:#f8f8f0;\">.</span><span style=\"color:#f8f8f2;\">Item</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">AccordionPrimitive.Item\n</span><span style=\"color:#f8f8f2;\">      data</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">slot</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;accordion-item&quot;\n</span><span style=\"color:#f8f8f2;\">      className</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#66d9ef;\">cn</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;border-b last:border-b-0&quot;</span><span style=\"color:#f8f8f2;\">, className)}}\n</span><span style=\"color:#f8f8f2;\">      {{</span><span style=\"color:#f92672;\">...</span><span style=\"color:#f8f8f2;\">props}}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">  );\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">function AccordionTrigger({{\n</span><span style=\"color:#f8f8f2;\">  className,\n</span><span style=\"color:#f8f8f2;\">  children,\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">...</span><span style=\"color:#f8f8f2;\">props\n</span><span style=\"color:#f8f8f2;\">}}: React.ComponentProps&lt;typeof AccordionPrimitive</span><span style=\"background-color:#f92672;color:#f8f8f0;\">.</span><span style=\"color:#f8f8f2;\">Trigger</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">AccordionPrimitive.Header className</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;flex&quot;</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">AccordionPrimitive.Trigger\n</span><span style=\"color:#f8f8f2;\">        data</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">slot</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;accordion-trigger&quot;\n</span><span style=\"color:#f8f8f2;\">        className</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#66d9ef;\">cn</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">          </span><span style=\"color:#ffee99;\">&quot;focus-visible:border-ring focus-visible:ring-ring/50 flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&amp;[data-state=open]&gt;svg]:rotate-180&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">          className\n</span><span style=\"color:#f8f8f2;\">        )}}\n</span><span style=\"color:#f8f8f2;\">        {{</span><span style=\"color:#f92672;\">...</span><span style=\"color:#f8f8f2;\">props}}\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">        {{children}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">ChevronDownIcon className</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;text-muted-foreground pointer-events-none size-4 shrink-0 translate-y-0.5 transition-transform duration-200&quot; </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">AccordionPrimitive.Trigger</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">AccordionPrimitive.Header</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">  );\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">function AccordionContent({{\n</span><span style=\"color:#f8f8f2;\">  className,\n</span><span style=\"color:#f8f8f2;\">  children,\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">...</span><span style=\"color:#f8f8f2;\">props\n</span><span style=\"color:#f8f8f2;\">}}: React.ComponentProps&lt;typeof AccordionPrimitive</span><span style=\"background-color:#f92672;color:#f8f8f0;\">.</span><span style=\"color:#f8f8f2;\">Content</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">) {{\n</span><span style=\"color:#f8f8f2;\">  </span><span style=\"color:#f92672;\">return </span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">AccordionPrimitive.Content\n</span><span style=\"color:#f8f8f2;\">      data</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">slot</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;accordion-content&quot;\n</span><span style=\"color:#f8f8f2;\">      className</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down overflow-hidden text-sm&quot;\n</span><span style=\"color:#f8f8f2;\">      {{</span><span style=\"color:#f92672;\">...</span><span style=\"color:#f8f8f2;\">props}}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">      </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div className</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#66d9ef;\">cn</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;pt-0 pb-4&quot;</span><span style=\"color:#f8f8f2;\">, className)}}</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">{{children}}</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">AccordionPrimitive.Content</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">  );\n</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">export {{ Accordion, AccordionItem, AccordionTrigger, AccordionContent }};</span></pre>\n",
        }
        p {
            "This code is riddled with dependencies:  "
            code { "@radix-ui/react-accordion" }
            ",  "
            code { "lucide-react" }
            ", Tailwind utilities, and React's own component system. Want to use it in a Vue app? Too bad. Trying to reuse it in a Yew project written in Rust? Forget about it. You're locked in, hard."
        }
        p {
            "In contrast, "
            strong { "Open SASS" }
            " components are the very definition of "
            em { "framework-agnostic" }
            ". Each component in Open SASS is shipped in multiple flavors, tailored to the platform you're using. Want to use the Accordion in "
            strong { "Yew" }
            "? Run, after installing the "
            a { href: "https://crates.io/crates/opensass", "Open SASS CLI" }
            ":"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">os add accordion</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs yew</span></pre>\n" }
        p {
            "This fetches only the Rust/Yew-compatible source code with no extra fluff, no unnecessary dependencies, and no styling assumptions. Here's an excerpt of what you get:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n</pre>\n" }
        p {
            "No framework or CSS library is forced on you. Use "
            strong { "Tailwind" }
            ", "
            strong { "Bootstrap" }
            ", "
            strong { "Vanilla CSS" }
            ", or "
            strong { "inline styles" }
            "; Open SASS doesn't care. The component is just logic and structure, not opinionated about how you present it. Want to switch from Yew to Dioxus? Easy:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">os add accordion</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs dio</span></pre>\n" }
        p {
            "This modularity empowers teams to build components once and reuse them across frontend frameworks, native apps, and even WebAssembly runtimes. This kind of adaptability is simply "
            strong { "unachievable" }
            " with Shadcn UI."
        }
        h2 { id: "performance",
            a { href: "#performance", class: "header", "Performance" }
        }
        p {
            "In software, speed isn't everything; But when tooling begins to slow down your workflow, it becomes a bottleneck. Shadcn UI relies on a Node.js-based CLI to fetch and generate components. That means it needs to spawn a Node process, resolve dependencies, and scaffold code in a relatively bloated JavaScript environment."
        }
        p { "Here's what happens when you run Shadcn's CLI:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">❯ time npx shadcn</span><span style=\"color:#f92672;\">@</span><span style=\"color:#f8f8f2;\">latest add accordion\n</span><span style=\"color:#ff80f4;\">5.93</span><span style=\"color:#f8f8f2;\">s user </span><span style=\"color:#ff80f4;\">2.22</span><span style=\"color:#f8f8f2;\">s system </span><span style=\"color:#ff80f4;\">93</span><span style=\"color:#f92672;\">%</span><span style=\"color:#f8f8f2;\"> cpu</span></pre>\n" }
        p {
            "Almost "
            strong { "6 seconds" }
            " to add a single component. That may not sound like much on its own, but in large projects or CI/CD pipelines, those seconds "
            strong { "add up" }
            "."
        }
        p {
            "Now compare that to "
            strong { "Open SASS" }
            ":"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">❯ time os add accordion</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs yew\n</span><span style=\"color:#ff80f4;\">0.17</span><span style=\"color:#f8f8f2;\">s user </span><span style=\"color:#ff80f4;\">0.02</span><span style=\"color:#f8f8f2;\">s system </span><span style=\"color:#ff80f4;\">6</span><span style=\"color:#f92672;\">%</span><span style=\"color:#f8f8f2;\"> cpu</span></pre>\n" }
        p {
            strong { "Under 0.2 seconds" }
            " of actual user time. That's roughly "
            strong { "60x faster" }
            ". The reason? Open SASS is built using "
            strong { "native Rust" }
            ", compiled ahead-of-time to a single binary, and it makes blazing fast HTTP requests directly to "
            code { "crates.io" }
            ". There's no JavaScript runtime bloat, no npm resolution latency, and no unnecessary complexity."
        }
        p {
            "This level of performance doesn't just make your dev life smoother; It scales beautifully in automated systems, remote dev containers, CI/CD pipelines, and embedded developer tooling. It's the kind of raw speed you feel instantly, and once you've experienced it, you'll never want to go back to a sluggish Node-based CLI."
        }
        h2 { id: "customization-and-extensibility",
            a { href: "#customization-and-extensibility", class: "header",
                "Customization and Extensibility"
            }
        }
        p {
            "Shadcn UI gives you a component that looks nice; "
            strong { "If" }
            " you stay inside its carefully crafted box. It assumes you want Tailwind. It assumes you want React. It assumes you want Radix. If you try to diverge, maybe to add some accessibility behavior that Radix doesn't support out-of-the-box; You'll find yourself fighting the abstraction more than working with it."
        }
        p {
            "Let's say you want to change the accordion animation or use a different icon system in Shadcn. You're diving into a web of dependencies like  "
            code { "@radix-ui/react-accordion" }
            ", needing to override Radix's animations, swap Lucide icons, and massage Tailwind classes around React's className hell."
        }
        p {
            "Open SASS, on the other hand, is "
            strong { "fully declarative and extensible" }
            ". You can:"
        }
        ul {
            li {
                "Customize animation speed with a "
                code { "duration" }
                " prop"
            }
            li {
                "Inject callbacks ("
                code { "will_open" }
                ", "
                code { "did_open" }
                ", "
                code { "will_close" }
                ", "
                code { "did_close" }
                ")"
            }
            li { "Apply your own ARIA logic" }
            li { "Use class-based or style-based theming" }
            li {
                "Extend components like "
                code { "Item" }
                ", "
                code { "Button" }
                ", or "
                code { "List" }
                " with your own elements"
            }
        }
        p {
            "No wrappers. No monkey patching. No awkward re-exports. Just clean, readable components, documented in detail, with no styling or framework assumptions. You're the boss."
        }
        p {
            "Want a bootstrap-themed accordion with your own icons and custom transition logic? You don't have to hack around someone else's component; You build directly with Open SASS:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n</pre>\n" }
        p { "No layers of indirection. No theme providers. Just clean code." }
        h2 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "In every measurable way, "
            strong { "portability" }
            ", "
            strong { "performance" }
            ", "
            strong { "customization" }
            ", "
            strong { "extensibility" }
            ", and "
            strong { "cross-platform support" }
            ", "
            strong { "Open SASS" }
            " obliterates "
            strong { "Shadcn UI" }
            ". Shadcn is a slick choice if you're building "
            em { "yet another" }
            " Tailwind/React site. But the moment you need to step outside that bubble, whether it's for performance, flexibility, native development, or Rust integration, Shadcn breaks. Open SASS thrives."
        }
        p {
            "Open SASS isn't just an alternative, it's the "
            strong { "future" }
            " of component libraries. Cross-framework. Cross-language (TODO). Ultra-fast. Infinitely extensible."
        }
        p { "If you want freedom, speed, and true composability; Open SASS isn't just better." }
        p { "It's on a whole other level." }
        blockquote {
            p {
                strong { "We are Open SASS, babe!" }
                "."
            }
        }
        blockquote {
            p { "We're working tirelessly on making Rust web development extremely easy for everyone." }
        }
        blockquote {
            p {
                "If you made it this far, it would be nice if you could "
                a { href: "https://discord.gg/b5JbvHW5nv", "join us on Discord" }
                "."
            }
        }
        blockquote {
            p {
                "Together, let's move the web beyond JavaScript, and into something that actually compiles."
            }
        }
        blockquote {
            p {
                "Feel free to share this. Fork it. Turn it into a meme. Tattoo it on your CI pipeline. Tell your manager Rust is your spirit animal now."
            }
        }
    }
}
#[component(no_case_check)]
pub fn ImageRsRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Welcome 👋!" }
        }
        p {
            "When it comes to building lightning-fast, highly interactive WASM apps, the devil's in the details, and images are a "
            strong { "huge" }
            " part of that. Traditionally, working with images in Yew, Dioxus, or Leptos has been... well, clunky. Limited flexibility, poor accessibility, bad loading UX, you name it."
        }
        p {
            "That's why we're excited to announce "
            a { href: "https://github.com/opensass/image-rs",
                strong { "Image RS" }
            }
            ", a "
            strong { "next-gen" }
            " image component designed from the ground up for "
            strong { "WASM-based" }
            " frameworks. Not just another helper, but a fully-loaded, highly-optimized solution that "
            strong { "removes" }
            " the bottlenecks we as developers usually run into when dealing with images."
        }
        p {
            "Let's explore why "
            strong { "Image RS" }
            " doesn't just meet your needs, it "
            strong { "exceeds" }
            " them 🚀."
        }
        h2 { id: "-what-is-image-rs",
            a { href: "#-what-is-image-rs", class: "header", "📸 What Is Image RS?" }
        }
        p {
            strong { "Image RS" }
            " is a "
            strong { "highly optimized" }
            ", "
            strong { "feature-rich" }
            " image component crafted specifically for the new wave of "
            strong { "Rust-powered frontend frameworks" }
            " like "
            strong { "Yew" }
            ", "
            strong { "Dioxus" }
            ", and "
            strong { "Leptos" }
            "."
        }
        p {
            "It's "
            strong { "fast" }
            " (thanks to smart lazy loading and "
            code { "IntersectionObserver" }
            "), "
            strong { "flexible" }
            " (supporting a range of layouts and styles), and "
            strong { "accessible" }
            " (full ARIA support baked in). Basically, it's everything you ever wanted from an image component, but for the WASM era."
        }
        h2 { id: "-why-youll-love-image-rs",
            a { href: "#-why-youll-love-image-rs", class: "header", "🚀 Why You'll Love Image RS" }
        }
        p {
            "Other image components in the wild are either too basic, too bloated, or too locked into specific frameworks. "
            strong { "Image RS" }
            " gives you "
            strong { "the best of all worlds" }
            ", right out of the box:"
        }
        ul {
            li {
                strong { "Performance Obsessed" }
                ": Lazy loading, blur placeholders, async decoding, all built-in and tuned for maximum speed."
            }
            li {
                strong { "Layout Freedom" }
                ": Fixed, Intrinsic, Responsive, Fill, Stretch, ScaleDown, pick whatever fits your design best."
            }
            li {
                strong { "Fully Accessible" }
                ": ARIA labels, roles, dynamic states, it's all handled properly."
            }
            li {
                strong { "Interactive Events" }
                ": Get callbacks for success or failure, build smarter, more resilient apps."
            }
            li {
                strong { "Visual Candy" }
                ": Blur-up placeholders, fallback images, quality settings, gorgeous UX without extra work."
            }
        }
        h2 { id: "-quick-yew-setup",
            a { href: "#-quick-yew-setup", class: "header", "🔥 Quick Yew Setup" }
        }
        p { "Getting started? It's a breeze." }
        p {
            "Add  "
            code { "image-rs" }
            " to your project:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add image</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        p { "Import it in your app:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">yew::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">image_rs::yew::Image;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">image_rs::Layout;</span></pre>\n" }
        p { "Use it:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[function_component(App)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Html {{\n</span><span style=\"color:#f8f8f2;\">    html! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Image\n</span><span style=\"color:#f8f8f2;\">            src</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/images/photo.jpg&quot;\n</span><span style=\"color:#f8f8f2;\">            alt</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;A beautiful view&quot;\n</span><span style=\"color:#f8f8f2;\">            width</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;800&quot;\n</span><span style=\"color:#f8f8f2;\">            height</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;600&quot;\n</span><span style=\"color:#f8f8f2;\">            layout</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Layout::Responsive}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            strong { "That's it." }
            " Blazing fast images, minimal setup."
        }
        h2 { id: "-features-that-set-it-apart",
            a { href: "#-features-that-set-it-apart", class: "header",
                "🧩 Features That Set It Apart"
            }
        }
        table {
            thead {
                th { "Feature" }
                th { "What It Means for You" }
            }
            tr {
                th { "Lazy Loading" }
                th { "Only loads images when needed, faster pages" }
            }
            tr {
                th { "Blur Placeholder" }
                th { "Smooth loading experience with visual cues" }
            }
            tr {
                th { "ARIA Accessibility" }
                th { "Inclusive, accessible apps without extra code" }
            }
            tr {
                th { "Layouts" }
                th { "Responsive, Fixed, Stretch, and more" }
            }
            tr {
                th { "Event Hooks" }
                th { "Handle load success, failure, and retries" }
            }
            tr {
                th { "Fallback Images" }
                th { "No more broken image icons" }
            }
            tr {
                th { "Custom Styling" }
                th { "Tailor it exactly to your needs" }
            }
        }
        h2 { id: "-full-control-with-props",
            a { href: "#-full-control-with-props", class: "header",
                "⚙\u{fe0f} Full Control with Props"
            }
        }
        p {
            "Need more fine-grained control?"
            strong { "Image RS" }
            " exposes a full suite of props, including:"
        }
        ul {
            li {
                code { "src" }
                ", "
                code { "alt" }
                ", "
                code { "width" }
                ", "
                code { "height" }
            }
            li {
                code { "fallback_src" }
                ", "
                code { "placeholder" }
                ", "
                code { "priority" }
            }
            li {
                code { "layout" }
                ", "
                code { "object_fit" }
                ", "
                code { "object_position" }
            }
            li {
                code { "on_loading_complete" }
                ", "
                code { "on_error" }
            }
            li {
                "Accessibility props like "
                code { "aria_labelledby" }
                ", "
                code { "aria_hidden" }
                ", "
                code { "aria_current" }
                ", etc."
            }
        }
        p {
            strong { "Customization?" }
            " ✅"
        }
        p {
            strong { "Flexibility?" }
            " ✅"
        }
        p {
            strong { "Developer happiness?" }
            " ✅"
        }
        h2 { id: "-real-world-examples",
            a { href: "#-real-world-examples", class: "header", "🎯 Real-World Examples" }
        }
        p {
            "Want to see it in action? Here are some real snippets you can find on "
            a { href: "https://image-rs.netlify.app", "the live demo" }
            ":"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Simple image\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Image src</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;https://placehold.co/300x200&quot;</span><span style=\"color:#f8f8f2;\"> alt</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Basic Image&quot; </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Responsive layout\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Image src</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;https://placehold.co/600x400&quot;</span><span style=\"color:#f8f8f2;\"> alt</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Responsive Layout&quot;</span><span style=\"color:#f8f8f2;\"> layout</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Layout::Responsive}} width</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;600&quot;</span><span style=\"color:#f8f8f2;\"> height</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;400&quot; </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Blur-up placeholder\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Image src</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;https://placehold.co/600x400&quot;</span><span style=\"color:#f8f8f2;\"> alt</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Blurred Image&quot;</span><span style=\"color:#f8f8f2;\"> layout</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Layout::Responsive}} width</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;600&quot;</span><span style=\"color:#f8f8f2;\"> height</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;400&quot;</span><span style=\"color:#f8f8f2;\"> placeholder</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;blur&quot;</span><span style=\"color:#f8f8f2;\"> blur_data_url</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;https://placehold.co/10x10&quot; </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Priority loading\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Image src</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;https://placehold.co/400x300&quot;</span><span style=\"color:#f8f8f2;\"> alt</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Priority Image&quot;</span><span style=\"color:#f8f8f2;\"> priority</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ff80f4;\">true </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#8c8c8c;\">// Custom object fit and position\n</span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Image\n</span><span style=\"color:#f8f8f2;\">    src</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;https://placehold.co/600x400&quot;\n</span><span style=\"color:#f8f8f2;\">    alt</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Cover Fit&quot;\n</span><span style=\"color:#f8f8f2;\">    layout</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Layout::Responsive}}\n</span><span style=\"color:#f8f8f2;\">    width</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;600&quot;\n</span><span style=\"color:#f8f8f2;\">    height</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;400&quot;\n</span><span style=\"color:#f8f8f2;\">    object_fit</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{ObjectFit::Cover}}\n</span><span style=\"color:#f8f8f2;\">    object_position</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Position::TopRight}}\n</span><span style=\"color:#f92672;\">/&gt;</span></pre>\n",
        }
        p {
            "Yes, you can do "
            strong { "a lot" }
            " with it, and still keep your codebase super clean."
        }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "💡 Final Thoughts" }
        }
        p {
            "When you're building modern, performant WASM apps, you need your images to keep up, not slow you down."
        }
        p {
            strong { "Image RS" }
            " is lightweight, battle-tested, accessibility-first, and packed with features that make real-world web development easier, smoother, and more scalable."
        }
        p {
            "If you're ready to stop fighting your image components and start shipping polished, professional apps, "
            strong { "Image RS" }
            " is here 🖼\u{fe0f}."
        }
        blockquote {
            p {
                strong { "We are Open SASS, babe!" }
                "."
            }
        }
        blockquote {
            p { "We're working tirelessly on making Rust web development extremely easy for everyone." }
        }
        blockquote {
            p {
                "If you made it this far, it would be nice if you could "
                a { href: "https://discord.gg/b5JbvHW5nv", "join us on Discord" }
                "."
            }
        }
        blockquote {
            p { "Till next time 👋!" }
        }
    }
}
#[component(no_case_check)]
pub fn ImageRsVsNextJsImage() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Hello, amazing developer! 👋" }
        }
        p {
            "Today we are going to talk about something that may seem small at first glance, but is actually "
            strong { "super duper important" }
            " for building fast, reliable, and professional websites: "
            strong { "image components" }
            "."
        }
        p {
            "And not just any image components, we're putting "
            strong { "Next.js's Native Image" }
            " head-to-head against the newer, shinier, Rust-powered "
            strong { "Image RS" }
            "."
        }
        p {
            "You might think, \"How different could two image components really be?\". Well, sit tight, because the difference is not just big, it's "
            strong { "game-changing" }
            " when you actually dig under the surface."
        }
        p {
            "This post will explain, in simple words, "
            strong { "why Image RS completely wins" }
            ", with "
            strong { "real facts" }
            ", "
            strong { "benchmarks" }
            ", and a few "
            strong { "good laughs" }
            " along the way."
        }
        p { "Let's get started!" }
        h2 { id: "-who-are-the-competitors",
            a { href: "#-who-are-the-competitors", class: "header",
                "⚔\u{fe0f} Who Are The Competitors?"
            }
        }
        h3 { id: "-nextjs-image",
            a { href: "#-nextjs-image", class: "header", "🥊 Next.js Image" }
        }
        p {
            a { href: "https://nextjs.org/docs/pages/api-reference/components/image",
                "Next.js Image"
            }
            " is part of the popular React-based Next.js framework. It's been around for a while and is trusted by thousands of developers across the world. It does a good job of "
            strong { "optimizing images automatically" }
            ", helping with "
            strong { "SEO" }
            ", and "
            strong { "improving page loading speed" }
            " by lazy-loading images. It's very good when you are already working in a JavaScript and React environment, and it makes handling images much easier than doing it manually. It's reliable, it's solid, butt, at the end of the day, it is still "
            strong { "limited by JavaScript" }
            " and its natural problems (we'll talk about that later on)."
        }
        h3 { id: "-image-rs",
            a { href: "#-image-rs", class: "header", "🥊 Image RS" }
        }
        p {
            a { href: "https://github.com/opensass/image-rs", "Image RS" }
            " is something "
            strong { "special" }
            ". It's a brand-new image component built specifically for "
            strong { "Rust-based frontend frameworks" }
            " like Yew, Dioxus, and Leptos. It runs inside the browser "
            strong { "through WebAssembly (Wasm)" }
            ", meaning it's written in "
            strong { "Rust" }
            ", then compiled into "
            strong { "blazing fast binary code" }
            " that the browser can run directly."
        }
        p {
            "That means you are not using slow JavaScript, you are using the "
            strong { "godly power of Rust" }
            ", the language famous for being blazingly fast, reliable, and efficient. With Image RS, you don't just get a component that looks good, you get "
            strong { "raw, unmatched performance" }
            ", "
            strong { "fine-grained control" }
            " over how the image behaves, and "
            strong { "less memory usage" }
            "."
        }
        p { "It's like upgrading from a scooter (Next.js Image) to a rocket ship (Image RS)." }
        h2 { id: "-why-image-rs-wins",
            a { href: "#-why-image-rs-wins", class: "header", "🧠 Why Image RS Wins" }
        }
        p {
            "Let's break it down point by point. You'll see "
            strong { "exactly why" }
            " Rust + Wasm is a total game-changer."
        }
        h3 { id: "native-rust--wasm-vs-javascript",
            a { href: "#native-rust--wasm-vs-javascript", class: "header",
                "Native Rust + Wasm vs JavaScript"
            }
        }
        p { "First, let's talk about how the two components are built." }
        ul {
            li {
                strong { "Next.js Image" }
                " runs inside the browser using "
                strong { "JavaScript" }
                ", the traditional way websites have worked for years."
            }
            li {
                strong { "Image RS" }
                " runs using "
                strong { "WebAssembly (Wasm)" }
                " compiled from "
                strong { "Rust" }
                ", meaning it acts much closer to how native applications behave."
            }
        }
        p { "This matters a lot." }
        p {
            "JavaScript is a "
            strong { "dynamic" }
            ", "
            strong { "garbage-collected" }
            " language, full of garbage. It's flexible, but it has a lot of overhead. Every time you load new images, update the DOM, or interact with elements, JavaScript engines need to "
            strong { "manage memory dynamically" }
            ", which sometimes leads to "
            strong { "random performance issues" }
            ", also known as "
            a { href: "https://developer.mozilla.org/en-US/docs/Glossary/Jank",
                strong { "jank" }
            }
            "."
        }
        p {
            "Meanwhile, Wasm is "
            strong { "strict" }
            " and "
            strong { "predictable" }
            ". Rust code running inside Wasm "
            strong { "manages memory very efficiently" }
            " and doesn't need a garbage collector. This means "
            strong { "no random pauses" }
            ", "
            strong { "no unexpected memory bloats" }
            ", and "
            strong { "super stable performance" }
            ", even under heavy loads."
        }
        p {
            "When your app grows, and you start loading hundreds or thousands of images, this difference "
            strong { "becomes massive" }
            "."
        }
        table {
            thead {
                th { "Feature" }
                th { "Image RS" }
                th { "Next.js Image" }
            }
            tr {
                th { "Native Rust+Wasm" }
                th { "✅" }
                th { "❌" }
            }
        }
        p {
            "✅ "
            strong { "Clear Advantage: Image RS" }
        }
        h3 { id: "built-in-image-optimization-both-are-good-kind-of",
            a {
                href: "#built-in-image-optimization-both-are-good-kind-of",
                class: "header",
                "Built-in Image Optimization: Both Are Good (Kind Of)"
            }
        }
        p {
            "Here, both "
            strong { "Next.js Image" }
            " and "
            strong { "Image RS" }
            " have strong features. They both offer:"
        }
        ul {
            li {
                strong { "Automatic resizing" }
                " of images depending on the screen size."
            }
            li {
                strong { "Lazy loading" }
                " images so they only appear when the user scrolls near them."
            }
            li {
                strong { "Smart decoding" }
                " of images in a way that doesn't block page rendering."
            }
            li {
                strong { "Placeholder blurring" }
                ", so users see a nice blur before the full image loads."
            }
        }
        p {
            "These features are "
            strong { "must-haves" }
            " today because they dramatically improve page speed and user experience. In this case, both components are "
            strong { "equally great" }
            " when it comes to basic optimizations."
        }
        table {
            thead {
                th { "Feature" }
                th { "Image RS" }
                th { "Next.js Image" }
            }
            tr {
                th { "Built-in Image Optimization" }
                th { "✅" }
                th { "✅" }
            }
        }
        p {
            "🤝 "
            strong { "Result: Tie" }
        }
        h3 { id: "fine-grained-dom-control-wasm-takes-the-crown",
            a {
                href: "#fine-grained-dom-control-wasm-takes-the-crown",
                class: "header",
                "Fine-Grained DOM Control: WASM Takes the Crown"
            }
        }
        p {
            "This is where "
            strong { "Image RS" }
            " starts pulling ahead strongly."
        }
        p {
            "With Yew and Wasm, you have "
            strong { "fine-grained control over every single DOM node" }
            ". You can directly manipulate how each image behaves, how it loads, how it resizes, or how it triggers callbacks, all with the speed and safety of Rust."
        }
        p {
            "Next.js Image, however, is tied deeply to "
            strong { "React's virtual DOM" }
            ". The virtual DOM is smart, but it's also a layer of abstraction that "
            strong { "gets in the way" }
            " when you need "
            strong { "precise, large-scale updates" }
            "."
        }
        p {
            "If you need to load, say, "
            strong { "10,000 images" }
            ", Image RS can handle it "
            strong { "smoothly and efficiently" }
            " by updating the real DOM "
            strong { "surgically" }
            "."
            strong { "very expensive" }
            " and slow at this scale."
        }
        table {
            thead {
                th { "Feature" }
                th { "Yew Image RS" }
                th { "Next.js Image" }
            }
            tr {
                th { "Fine-grained DOM Control" }
                th { "✅" }
                th { "❌" }
            }
        }
        p {
            "🏆 "
            strong { "Clear Winner: Image RS" }
        }
        h3 { id: "jswasm-payload-size-both-keep-it-tight",
            a {
                href: "#jswasm-payload-size-both-keep-it-tight",
                class: "header",
                "JS/Wasm Payload Size: Both Keep It Tight"
            }
        }
        p {
            "One big concern in web development today is "
            strong { "bundle size" }
            ", you don't want users downloading megabytes of JavaScript just to open a simple page."
        }
        p {
            "Luckily, both Next.js Image and Image RS are "
            strong { "optimized for small payloads" }
            ":"
        }
        ul {
            li {
                "Next.js uses "
                a { href: "https://en.wikipedia.org/wiki/Tree_shaking", "tree-shaking" }
                " and "
                a { href: "https://developer.mozilla.org/en-US/docs/Glossary/Code_splitting",
                    "code-splitting"
                }
                " to send only the code you need."
            }
            li {
                "Yew compiles Rust code into "
                strong { "tiny Wasm binaries" }
                " that are often "
                strong { "smaller than equivalent JavaScript" }
                "."
            }
        }
        p { "Either way, you're not going to overload your user's internet connection." }
        table {
            thead {
                th { "Feature" }
                th { "Yew Image RS" }
                th { "Next.js Image" }
            }
            tr {
                th { "Smaller JS/Wasm Payload" }
                th { "✅" }
                th { "✅" }
            }
        }
        p {
            "🤝 "
            strong { "Result: Tie" }
        }
        h2 { id: "-real-benchmark-time",
            a { href: "#-real-benchmark-time", class: "header", "📈 Real Benchmark Time." }
        }
        p {
            "We actually "
            strong { "measured" }
            " everything using "
            strong { "Lighthouse" }
            ", and the results speak for themselves."
        }
        h3 { id: "when-loading-10-images-small-scale",
            a { href: "#when-loading-10-images-small-scale", class: "header",
                "When Loading 10 Images (small scale)"
            }
        }
        table {
            thead {
                th { "Metric" }
                th { "Yew (Wasm)" }
                th { "Next.js" }
            }
            tr {
                th { "Performance Score" }
                th { "100" }
                th { "100" }
            }
            tr {
                th { "Memory Usage" }
                th { "8 MB" }
                th { "8 MB" }
            }
        }
        p {
            img {
                src: "https://github.com/user-attachments/assets/703d6623-c2b9-46a7-81fb-01c56dd13466",
                alt: "Yew Image RS",
                title: "",
            }
        }
        p {
            "At small sizes, both are lightning-fast. You won't notice much difference if you are only loading a few images. Both are "
            strong { "perfectly smooth and reliable" }
            " here."
        }
        h3 { id: "when-loading-10000-images-huge-scale",
            a {
                href: "#when-loading-10000-images-huge-scale",
                class: "header",
                "When Loading 10,000 Images (huge scale)"
            }
        }
        table {
            thead {
                th { "Metric" }
                th { "Yew (Wasm)" }
                th { "Next.js" }
            }
            tr {
                th { "Performance Score" }
                th { "64" }
                th { "❌ (FAILED)" }
            }
            tr {
                th { "Memory Usage" }
                th { "78 MB" }
                th { "83 MB" }
            }
            tr {
                th { "Scrolling Smoothness" }
                th { "Very Smooth" }
                th { "Laggy Disaster" }
            }
        }
        p {
            img {
                src: "https://github.com/user-attachments/assets/acb106b0-8c1c-47c7-ae75-605f2f6a8cfb",
                alt: "Yew Image RS",
                title: "",
            }
            img {
                src: "https://github.com/user-attachments/assets/38b053ed-22af-48fc-a52c-617860fe33d3",
                alt: "Image RS Memory Usage",
                title: "",
            }
            img {
                src: "https://github.com/user-attachments/assets/88580a9a-0036-4782-8d40-a98faa63e789",
                alt: "Next JS Image Memory Usage",
                title: "",
            }
            img {
                src: "https://github.com/user-attachments/assets/beecbee3-f021-4572-bd1d-6515de0e8f15",
                alt: "Next JS Image Lighthouse score failed",
                title: "",
            }
        }
        p { "Once we crank up the scale, the story changes:" }
        ul {
            li {
                strong { "Image RS" }
                " keeps its cool. Scrolling stays buttery smooth, memory usage is controlled, and the page is still responsive."
            }
            li {
                strong { "Next.js Image" }
                " completely falls apart. Lighthouse "
                strong { "couldn't even finish auditing" }
                " because the page became too slow and unresponsive."
            }
        }
        p {
            "✅ Image RS uses "
            strong { "less memory" }
            ", scrolls faster, and behaves like a professional athlete."
        }
        p {
            "❌ Next.js Image "
            strong { "lags" }
            ", "
            strong { "stutters" }
            ", and "
            strong { "breaks" }
            " under the pressure."
        }
        h2 { id: "-why-wasm--rust-crushes-js--react",
            a { href: "#-why-wasm--rust-crushes-js--react", class: "header",
                "🧪 Why Wasm + Rust Crushes JS + React"
            }
        }
        p {
            strong { "Technical reason in simple words:" }
        }
        ul {
            li {
                "Rust gives "
                strong { "tight, efficient control over memory" }
                "."
            }
            li {
                "Wasm allows "
                strong { "direct execution" }
                " without needing garbage collection."
            }
            li { "No surprise pauses, no random memory leaks, no hidden costs." }
        }
        p { "Meanwhile, JavaScript has to:" }
        ul {
            li { "Constantly check and clean up memory." }
            li { "Handle garbage collection pauses unpredictably." }
        }
        p {
            "That's why Rust+Wasm simply "
            strong { "outperforms" }
            " JavaScript at any serious scale."
        }
        h2 { id: "-the-verdict",
            a { href: "#-the-verdict", class: "header", "🎯 The Verdict" }
        }
        table {
            thead {
                th { "Category" }
                th { "Winner" }
            }
            tr {
                th { "Native Performance" }
                th { "Image RS" }
            }
            tr {
                th { "Fine-grained Control" }
                th { "Image RS" }
            }
            tr {
                th { "Massive Scaling" }
                th { "Image RS" }
            }
            tr {
                th { "Developer Smugness" }
                th { "Image RS" }
            }
        }
        p {
            "✅ If you care about "
            strong { "speed" }
            ", "
            strong { "reliability" }
            ", "
            strong { "future-proof apps" }
            ", and "
            strong { "developer happiness" }
            ", then "
            strong { "Image RS" }
            " is the clear winner."
        }
        p {
            "✅ If you want your app to handle not just today's needs but tomorrow's massive data scales, "
            strong { "Rust + Image RS" }
            " is the way to go."
        }
        p {
            "✅ If you want "
            strong { "less pain, better performance, and cleaner code" }
            ", the answer is simple."
        }
        h1 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "🚀 Final Thoughts" }
        }
        p {
            "Next.js Image is good, for small apps where JavaScript's overhead doesn't show. But if you're dreaming big? If you're building "
            strong { "modern apps that scale" }
            "? If you want "
            strong { "buttery-smooth UX" }
            " no matter what? Then "
            strong { "Image RS" }
            " isn't just an option. It's the obvious next step."
        }
        p {
            "So don't let your users suffer slow load times and laggy scrolling. "
            strong { "Give them the Wasm experience they deserve." }
            " 🚀"
        }
        blockquote {
            p {
                strong { "We are Open SASS, babe!" }
                "."
            }
        }
        blockquote {
            p { "We're working tirelessly on making Rust web development extremely easy for everyone." }
        }
        blockquote {
            p {
                "If you made it this far, it would be nice if you could "
                a { href: "https://discord.gg/b5JbvHW5nv", "join us on Discord" }
                "."
            }
        }
        blockquote {
            p {
                "Together, let's move the web beyond JavaScript, and into something that actually compiles."
            }
        }
        blockquote {
            p {
                "Feel free to share this. Fork it. Turn it into a meme. Tattoo it on your CI pipeline. Tell your manager Rust is your spirit animal now."
            }
        }
    }
}
#[component(no_case_check)]
pub fn TableRsRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "👋 Welcome" }
        }
        p {
            "When you're building "
            strong { "lightning-fast" }
            ", "
            strong { "interactive" }
            ", and "
            strong { "accessible" }
            " WASM frontends with "
            strong { "Yew" }
            ", the little things can make a big difference, and "
            strong { "tables" }
            " are no exception."
        }
        p {
            "Until now, building a rich data table in Rust meant reinventing the wheel, stitching together sorting logic, pagination UI, and URL state sync, all while wrestling with accessibility concerns and styling constraints."
        }
        p {
            "That's why we're excited to introduce "
            a { href: "https://github.com/opensass/table-rs",
                strong { "Table RS" }
            }
            " 📊, a "
            strong { "powerful" }
            ", "
            strong { "lightweight" }
            ", and "
            strong { "accessibility-first" }
            " table component purpose-built for "
            strong { "Yew" }
            ", "
            strong { "Dioxus" }
            ", and "
            strong { "Leptos" }
            "."
        }
        p {
            "Let's explore why "
            strong { "Table RS" }
            " isn't just another table, it's the one your Rust frontend deserves."
        }
        h2 { id: "-what-is-table-rs",
            a { href: "#-what-is-table-rs", class: "header", "📦 What Is Table RS?" }
        }
        p {
            strong { "Table RS" }
            " is a modular, fully-featured data table component that brings together "
            strong { "real-time search" }
            ", "
            strong { "column sorting" }
            ", "
            strong { "pagination" }
            ", and "
            strong { "style customization" }
            ", all with built-in support for "
            strong { "semantic markup" }
            ", "
            strong { "ARIA attributes" }
            ", and "
            strong { "URL synchronization" }
            "."
        }
        p {
            "Built specifically for "
            strong { "WASM-based Rust frameworks" }
            ", Table RS was designed with three key priorities:"
        }
        ul {
            li {
                strong { "Performance" }
                " (don't block your app or over-render)."
            }
            li {
                strong { "Accessibility" }
                " (keyboard users and screen readers welcome)."
            }
            li {
                strong { "Developer Ergonomics" }
                " (composable, prop-driven API)."
            }
        }
        p { "It's time to stop wrestling with HTML tables and start focusing on your app logic." }
        h2 { id: "-why-youll-love-table-rs",
            a { href: "#-why-youll-love-table-rs", class: "header", "🚀 Why You'll Love Table RS" }
        }
        p { "Table RS isn't just a helper, it's a solution. Here's what sets it apart:" }
        ul {
            li {
                strong { "🔍 Built-in Search" }
                ": Add a global search bar in one prop, no extra logic required. It even syncs with the URL ("
                code { "?search=query" }
                ") so users can share filtered views."
            }
            li {
                strong { "⬆\u{fe0f} Sorting Support" }
                ": Enable column-based sorting with accessible "
                code { "aria-sort" }
                " indicators for screen readers."
            }
            li {
                strong { "📄 Pagination" }
                ": Easily split data into pages, customize page size, and add intuitive nav controls."
            }
            li {
                strong { "🧹 Debounced Inputs" }
                ": Reduce unnecessary re-renders for better user experience and performance."
            }
            li {
                strong { "🎨 Full Customization" }
                ": Override class names and inline styles with ease."
            }
            li {
                strong { "♿ Accessibility First" }
                ": Proper roles, ARIA attributes, and semantic tags built-in by default."
            }
            li {
                strong { "🛠 Zero Boilerplate" }
                ": Focus on your data, not on wiring up handlers or rebuilding UI from scratch."
            }
        }
        h2 { id: "-quick-setup-for-yew",
            a { href: "#-quick-setup-for-yew", class: "header", "⚙\u{fe0f} Quick Setup for Yew" }
        }
        p {
            "Setting up "
            strong { "Table RS" }
            " in your Yew app is easy and intuitive. Let's walk through it."
        }
        h3 { id: "1-add-the-dependency",
            a { href: "#1-add-the-dependency", class: "header",
                "1\u{fe0f}\u{20e3} Add the Dependency"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add table</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        h3 { id: "2-import-the-component",
            a { href: "#2-import-the-component", class: "header",
                "2\u{fe0f}\u{20e3} Import the Component"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">yew::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">table_rs::yew::table::Table;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">table_rs::yew::types::Column;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">maplit::hashmap;</span></pre>\n" }
        h3 { id: "3-use-it-in-your-component",
            a { href: "#3-use-it-in-your-component", class: "header",
                "3\u{fe0f}\u{20e3} Use It in Your Component"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[function_component(App)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Html {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> data </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">vec![\n</span><span style=\"color:#f8f8f2;\">        hashmap! {{ </span><span style=\"color:#ffee99;\">&quot;name&quot; </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#ffee99;\">&quot;Ferris&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ffee99;\">&quot;email&quot; </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#ffee99;\">&quot;ferris@opensass.org&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">() }},\n</span><span style=\"color:#f8f8f2;\">        hashmap! {{ </span><span style=\"color:#ffee99;\">&quot;name&quot; </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#ffee99;\">&quot;Ferros&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ffee99;\">&quot;email&quot; </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#ffee99;\">&quot;ferros@opensass.org&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">() }},\n</span><span style=\"color:#f8f8f2;\">        hashmap! {{ </span><span style=\"color:#ffee99;\">&quot;name&quot; </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#ffee99;\">&quot;Crab&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(), </span><span style=\"color:#ffee99;\">&quot;email&quot; </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#ffee99;\">&quot;crab@opensass.org&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">() }},\n</span><span style=\"color:#f8f8f2;\">    ];\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> columns </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">vec![\n</span><span style=\"color:#f8f8f2;\">        Column {{\n</span><span style=\"color:#f8f8f2;\">            id: </span><span style=\"color:#ffee99;\">&quot;name&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            header: </span><span style=\"color:#ffee99;\">&quot;Name&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            sortable: </span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">..</span><span style=\"font-style:italic;color:#66d9ef;\">Default</span><span style=\"color:#f8f8f2;\">::default()\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">        Column {{\n</span><span style=\"color:#f8f8f2;\">            id: </span><span style=\"color:#ffee99;\">&quot;email&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            header: </span><span style=\"color:#ffee99;\">&quot;Email&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            sortable: </span><span style=\"color:#ff80f4;\">false</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">..</span><span style=\"font-style:italic;color:#66d9ef;\">Default</span><span style=\"color:#f8f8f2;\">::default()\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    ];\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    html! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Table data</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{data}} columns</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{columns}} </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "✅ That's it, your table is searchable, paginated, sortable, and accessible out of the box."
        }
        h2 { id: "-table-rs-props-overview",
            a { href: "#-table-rs-props-overview", class: "header", "🧩 Table RS Props Overview" }
        }
        h3 { id: "-core-props",
            a { href: "#-core-props", class: "header", "🔑 Core Props" }
        }
        table {
            thead {
                th { "Prop" }
                th { "Type" }
                th { "Description" }
                th { "Default" }
            }
            tr {
                th {
                    code { "data" }
                    ""
                }
                th {
                    code { "Vec<HashMap<&'static str, String>>" }
                    ""
                }
                th { "Row data, each row is a hashmap of column IDs." }
                th {
                    code { "[]" }
                    ""
                }
            }
            tr {
                th {
                    code { "columns" }
                    ""
                }
                th {
                    code { "Vec<Column>" }
                    ""
                }
                th { "Describes headers and behavior for each column." }
                th {
                    code { "[]" }
                    ""
                }
            }
            tr {
                th {
                    code { "paginate" }
                    ""
                }
                th {
                    code { "bool" }
                    ""
                }
                th { "Enables pagination controls." }
                th {
                    code { "false" }
                    ""
                }
            }
            tr {
                th {
                    code { "page_size" }
                    ""
                }
                th {
                    code { "usize" }
                    ""
                }
                th { "How many rows per page." }
                th {
                    code { "10" }
                    ""
                }
            }
            tr {
                th {
                    code { "search" }
                    ""
                }
                th {
                    code { "bool" }
                    ""
                }
                th { "Enables global search input." }
                th {
                    code { "false" }
                    ""
                }
            }
            tr {
                th {
                    code { "loading" }
                    ""
                }
                th {
                    code { "bool" }
                    ""
                }
                th { "Show a loading indicator." }
                th {
                    code { "false" }
                    ""
                }
            }
            tr {
                th {
                    code { "classes" }
                    ""
                }
                th {
                    code { "TableClasses" }
                    ""
                }
                th { "Customize class names." }
                th { "Defaults" }
            }
            tr {
                th {
                    code { "styles" }
                    ""
                }
                th {
                    code { "HashMap<&'static str, &'static str>" }
                    ""
                }
                th { "Add inline styles to table parts." }
                th {
                    code { "{{}}" }
                    ""
                }
            }
        }
        h3 { id: "-column-definition",
            a { href: "#-column-definition", class: "header", "🧱 Column Definition" }
        }
        table {
            thead {
                th { "Field" }
                th { "Type" }
                th { "Description" }
                th { "Default" }
            }
            tr {
                th {
                    code { "id" }
                    ""
                }
                th {
                    code { "&'static str" }
                    ""
                }
                th { "Matches the key in the row data." }
                th {
                    code { "\"\"" }
                    ""
                }
            }
            tr {
                th {
                    code { "header" }
                    ""
                }
                th {
                    code { "&'static str" }
                    ""
                }
                th { "Display name in the header." }
                th {
                    code { "\"\"" }
                    ""
                }
            }
            tr {
                th {
                    code { "sortable" }
                    ""
                }
                th {
                    code { "bool" }
                    ""
                }
                th { "Enables sorting for this column." }
                th {
                    code { "false" }
                    ""
                }
            }
            tr {
                th {
                    code { "class" }
                    ""
                }
                th {
                    code { "Option<&'static str>" }
                    ""
                }
                th { "Optional class name." }
                th {
                    code { "None" }
                    ""
                }
            }
            tr {
                th {
                    code { "style" }
                    ""
                }
                th {
                    code { "Option<&'static str>" }
                    ""
                }
                th { "Optional inline styles." }
                th {
                    code { "None" }
                    ""
                }
            }
        }
        h3 { id: "-styleclasses-reference",
            a { href: "#-styleclasses-reference", class: "header", "🎨 Style/Classes Reference" }
        }
        table {
            thead {
                th { "Section" }
                th { "CSS Class (default)" }
                th { "Description" }
            }
            tr {
                th {
                    code { "container" }
                    ""
                }
                th {
                    code { "\"table-container\"" }
                    ""
                }
                th { "Outer wrapper" }
            }
            tr {
                th {
                    code { "search_input" }
                    ""
                }
                th {
                    code { "\"search-input\"" }
                    ""
                }
                th { "The input box for search" }
            }
            tr {
                th {
                    code { "table" }
                    ""
                }
                th {
                    code { "\"table\"" }
                    ""
                }
                th {
                    "The "
                    code { "<table>" }
                    " element"
                }
            }
            tr {
                th {
                    code { "thead" }
                    ""
                }
                th {
                    code { "\"thead\"" }
                    ""
                }
                th { "Header row section" }
            }
            tr {
                th {
                    code { "tbody" }
                    ""
                }
                th {
                    code { "\"tbody\"" }
                    ""
                }
                th { "Body rows section" }
            }
            tr {
                th {
                    code { "pagination" }
                    ""
                }
                th {
                    code { "\"pagination-controls\"" }
                    ""
                }
                th { "Pagination UI wrapper" }
            }
        }
        h2 { id: "-built-with-open-sass",
            a { href: "#-built-with-open-sass", class: "header", "🤝 Built With Open SASS" }
        }
        p {
            "Table RS is proudly part of the "
            a { href: "https://github.com/opensass/cli", "Open SASS" }
            " ecosystem, where we build battle-tested UI primitives for Rust-powered frontends."
        }
        p {
            "This project is "
            strong { "open source" }
            ", "
            strong { "community-driven" }
            ", and ready for contributions. Whether it's bug reports, feature ideas, or PRs, we'd love to hear from you."
        }
        p {
            "👉 "
            a { href: "https://github.com/opensass/table-rs", "GitHub Repo" }
        }
        p {
            "👉 "
            a { href: "https://table-rs.netlify.app", "Live Demo" }
        }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "🎯 Final Thoughts" }
        }
        p {
            "If you've struggled with hand-rolling tables in Yew or wanted something more flexible than HTML templates, "
            strong { "Table RS" }
            " is built for you."
        }
        p {
            "It's fast, lightweight, thoughtfully designed, and extensible, ready for production use and just as happy in your side projects."
        }
        blockquote {
            p {
                strong { "We are Open SASS, babe!" }
                "."
            }
        }
        blockquote {
            p { "We're working tirelessly on making Rust web development extremely easy for everyone." }
        }
        blockquote {
            p {
                "If you made it this far, it would be nice if you could "
                a { href: "https://discord.gg/b5JbvHW5nv", "join us on Discord" }
                "."
            }
        }
        blockquote {
            p {
                "Together, let's move the web beyond JavaScript, and into something that actually compiles."
            }
        }
        blockquote {
            p {
                "Feel free to share this. Fork it. Turn it into a meme. Tattoo it on your CI pipeline. Tell your manager Rust is your spirit animal now."
            }
        }
        blockquote {
            p { "Till next time 👋" }
        }
    }
}
#[component(no_case_check)]
pub fn TanstackTableVsTableRs() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Hello, humble bumble dev 👋!" }
        }
        p {
            "Today we're going to explore something "
            strong { "massive" }
            ", both literally and figuratively: "
            strong { "data tables with 1 million rows" }
            "."
        }
        p {
            "Yep, that's right. We're not talking about a handful of to-do items or a polite little contacts list. We're talking "
            strong { "MILLIONS of rows" }
            ", real-world, backend-fed, browser-rendered "
            strong { "big boy data" }
            "."
        }
        p { "And today, we're putting two contenders to the test:" }
        ul {
            li {
                "🥊 "
                strong { "TanStack Table" }
                ", the battle-tested React data table library."
            }
            li {
                "🥊 "
                strong { "Table RS" }
                ", the new WASM godly-powered table built with Rust + Yew."
            }
        }
        p {
            "You might think, \"Wait... a WASM table? How different could it really be?\". Oh, just hold my  "
            code { "cargo build" }
            ". Because what you're about to see is "
            strong { "not just a difference, it's a generational leap" }
            "."
        }
        p {
            "Let's break it down, with "
            strong { "real numbers" }
            ", a little bit of "
            strong { "sass" }
            ", and a whole lot of "
            strong { "truth" }
            "."
        }
        h2 { id: "-the-competitors-react-vs-wasm",
            a { href: "#-the-competitors-react-vs-wasm", class: "header",
                "⚔\u{fe0f} The Competitors: React vs WASM"
            }
        }
        h3 { id: "-tanstack-table-react",
            a { href: "#-tanstack-table-react", class: "header", "🟨 TanStack Table (React)" }
        }
        p {
            a { href: "https://tanstack.com/table/latest", "TanStack Table" }
            " is a beloved data-table library in the React ecosystem. Works great in many apps. Strong community. Solid virtualized rendering support. But... it still lives and breathes JavaScript. And JavaScript, dear friends, "
            strong { "struggles" }
            " when things get "
            strong { "huge" }
            "."
        }
        h3 { id: "-table-rs-yew--wasm",
            a { href: "#-table-rs-yew--wasm", class: "header", "🟩 Table RS (Yew + WASM)" }
        }
        p {
            "A newcomer, built in Rust, compiled to WASM, running in the browser with "
            strong { "near-native performance" }
            ". No virtual DOM. No garbage collection. Just pure speed, stability, and zero apologies."
        }
        h2 { id: "-benchmark-tanstack-table-vs-table-rs-1-million-rows",
            a {
                href: "#-benchmark-tanstack-table-vs-table-rs-1-million-rows",
                class: "header",
                "📊 Benchmark: TanStack Table vs Table RS (1 Million Rows)"
            }
        }
        table {
            thead {
                th { "Metric" }
                th {
                    strong { "TanStack Table (React)" }
                    ""
                }
                th {
                    strong { "Table RS (Yew + WASM)" }
                    ""
                }
            }
            tr {
                th {
                    strong { "Page Load Time (1M rows)" }
                    ""
                }
                th { "~" }
                th { "~" }
            }
            tr {
                th {
                    strong { "Memory Heap Usage" }
                    ""
                }
                th { ">3 GB (heap overflow!)" }
                th { "~" }
            }
            tr {
                th {
                    strong { "Initial Rendering" }
                    ""
                }
                th { "Heavy blocking, slow DOM paint" }
                th { "Efficient, lightweight rendering" }
            }
            tr {
                th {
                    strong { "Browser Responsiveness" }
                    ""
                }
                th { "Laggy and delayed" }
                th { "Smooth like butter" }
            }
            tr {
                th {
                    strong { "Sorting Performance" }
                    ""
                }
                th { "2-4s for large columns" }
                th { "Sub-1s 💨" }
            }
            tr {
                th {
                    strong { "Search Performance" }
                    ""
                }
                th { "Acceptable, but sluggish" }
                th { "Instant. Blink-and-it's-done" }
            }
            tr {
                th {
                    strong { "Lighthouse Performance Score" }
                    ""
                }
                th { "49/100" }
                th { "60/100 (with 1M rows!)" }
            }
            tr {
                th {
                    strong { "Scalability" }
                    ""
                }
                th { "Hits JS memory limits fast" }
                th { "Laughs in WebAssembly" }
            }
        }
        p {
            img {
                src: "https://github.com/user-attachments/assets/5cdf63a2-09de-403d-89d5-bad6cef53a29",
                alt: "Screenshot",
                title: "",
            }
            img {
                src: "https://github.com/user-attachments/assets/7e4f2d08-d5d1-49b8-ba9e-e4e6174313a3",
                alt: "Screenshot",
                title: "",
            }
            img {
                src: "https://github.com/user-attachments/assets/d116fe18-9a49-4520-a24b-8d1b1b37258c",
                alt: "Screenshot",
                title: "",
            }
            img {
                src: "https://github.com/user-attachments/assets/afb55bbf-6a98-4794-a0a9-f7fefaa3707d",
                alt: "Screenshot",
                title: "",
            }
        }
        p {
            "Let's be real: that's "
            strong { "not even close" }
            ". And if you're thinking \"wait, maybe TanStack just needs some optimization\", let me gently stop you right there..."
        }
        h2 { id: "-why-tanstack-falls-apart",
            a { href: "#-why-tanstack-falls-apart", class: "header",
                "🧠 Why TanStack Falls Apart"
            }
        }
        p {
            "TanStack Table is amazing for "
            strong { "normal" }
            " data sets. But 1 million rows is where "
            strong { "JavaScript's weaknesses explode" }
            ":"
        }
        ul {
            li {
                "Memory consumption spikes to "
                strong { "3GB+" }
                " just to load data."
            }
            li {
                "Sorting or filtering starts to "
                strong { "choke the main thread" }
                "."
            }
            li {
                "React's reconciliation starts "
                strong { "gasping for air" }
                "."
            }
            li {
                "Eventually: 💥 "
                strong { "heap overflow errors" }
                "."
            }
        }
        p {
            "You might even get "
            strong { "browser crashes" }
            " depending on the system. It's not TanStack's fault. It's the platform."
        }
        h2 { id: "-why-table-rs-rocks-so-hard",
            a { href: "#-why-table-rs-rocks-so-hard", class: "header",
                "🦾 Why Table RS Rocks So Hard"
            }
        }
        p { "Table RS doesn't play around. It:" }
        ul {
            li {
                "Loads 1 million rows with "
                strong { "~" }
                " memory."
            }
            li {
                "Sorts and filters "
                strong { "in milliseconds" }
                "."
            }
            li { "Doesn't need hydration." }
            li {
                "Doesn't virtualize, it just renders what's needed, "
                strong { "fast" }
                "."
            }
            li {
                "Compiles from Rust to Wasm = "
                strong { "native god-like binary performance in your browser" }
                "."
            }
        }
        p {
            "No garbage collection. No memory leaks. No jank. It's like putting a Formula 1 engine into your HTML table."
        }
        h2 { id: "-real-world-experience",
            a { href: "#-real-world-experience", class: "header", "🚀 Real-World Experience" }
        }
        p { "When testing both, here's what we saw:" }
        ul {
            li {
                strong { "TanStack Table" }
                ": 10+ second initial load. Browser lags. Dev tools freeze. Heap overflow during build when enabling aggressive queries."
            }
            li {
                strong { "Table RS" }
                ": Loads in "
                strong { "2 seconds flat" }
                ". Page remains responsive. Sorts in less than 1s. Feels native."
            }
        }
        p {
            "Even "
            strong { "Lighthouse" }
            " agrees:"
        }
        table {
            thead {
                th { "Metric" }
                th { "TanStack (React)" }
                th { "Table RS (WASM)" }
            }
            tr {
                th { "Performance Score" }
                th { "49/100" }
                th { "60/100" }
            }
            tr {
                th { "Memory Stability" }
                th { "❌" }
                th { "✅" }
            }
            tr {
                th { "First Contentful Paint" }
                th { "4-5s" }
                th { "~" }
            }
            tr {
                th { "CPU Blocking Time" }
                th { "🧱 Huge" }
                th { "😌 Minimal" }
            }
        }
        p {
            "Remember, that's with "
            strong { "1 million rows" }
            ". Not 10. Not 100. "
            strong { "A literal million" }
            "."
        }
        h2 { id: "-why-webassembly-is-the-future",
            a { href: "#-why-webassembly-is-the-future", class: "header",
                "🔥 Why WebAssembly Is the Future"
            }
        }
        p { "Let's put this simply:" }
        ul {
            li {
                strong { "JavaScript" }
                " is fine for interactive toys and CRUD apps."
            }
            li {
                "But when you need to go "
                strong { "beyond the limits of the VDOM" }
                ", you need "
                strong { "WASM" }
                "."
            }
            li {
                "Rust + Wasm "
                strong { "compiles to tiny, fast, deterministic code" }
                "."
            }
            li {
                "It's not just fast, it's "
                strong { "reliable" }
                ", "
                strong { "predictable" }
                ", and "
                strong { "insanely scalable" }
                "."
            }
        }
        p { "WASM doesn't guess. It doesn't garbage collect. It just runs." }
        p {
            "React and JS frameworks are incredible for most apps, but they hit the ceiling "
            strong { "fast" }
            " when you're dealing with real data at scale."
        }
        h2 { id: "-final-verdict",
            a { href: "#-final-verdict", class: "header", "📣 Final Verdict" }
        }
        table {
            thead {
                th { "Category" }
                th { "Winner" }
            }
            tr {
                th { "Memory Efficiency" }
                th { "Table RS" }
            }
            tr {
                th { "Rendering Speed" }
                th { "Table RS" }
            }
            tr {
                th { "Browser Stability" }
                th { "Table RS" }
            }
            tr {
                th { "Sorting/Filtering" }
                th { "Table RS" }
            }
            tr {
                th { "Lighthouse Score" }
                th { "Table RS" }
            }
            tr {
                th { "Developer Vibes 😎" }
                th { "Table RS" }
            }
        }
        p {
            "✅ If you're building a "
            strong { "real-world, data-heavy frontend" }
            ", and you care about:"
        }
        ul {
            li { "Speed." }
            li { "Memory." }
            li { "Reliability." }
            li { "And not making your users cry." }
        }
        p {
            "Then "
            strong { "Table RS with Yew + WASM" }
            " isn't just a cool idea. It's "
            strong { "the clear future" }
            " of frontend performance."
        }
        h2 { id: "-wrap-up-js-had-a-good-run",
            a { href: "#-wrap-up-js-had-a-good-run", class: "header",
                "👋 Wrap-Up: JS Had a Good Run..."
            }
        }
        p { "But it's time." }
        p {
            "The web is moving toward "
            strong { "compiled, typed, high-performance apps" }
            ". And WASM is the rocket fuel that's going to power it. Rust + WASM let you build "
            strong { "massive-scale web apps" }
            " with "
            strong { "tiny, reliable performance footprints" }
            ". That's not just good engineering. That's "
            strong { "ethical development" }
            ". You're saving your users from slow pages and broken tabs."
        }
        blockquote {
            p {
                strong { "We are Open SASS, babe!" }
                "."
            }
        }
        blockquote {
            p { "We're working tirelessly on making Rust web development extremely easy for everyone." }
        }
        blockquote {
            p {
                "If you made it this far, it would be nice if you could "
                a { href: "https://discord.gg/b5JbvHW5nv", "join us on Discord" }
                "."
            }
        }
        blockquote {
            p {
                "Together, let's move the web beyond JavaScript, and into something that actually compiles."
            }
        }
        blockquote {
            p {
                "Feel free to share this. Fork it. Turn it into a meme. Tattoo it on your CI pipeline. Tell your manager Rust is your spirit animal now."
            }
        }
        blockquote {
            p { "Till next time 👋" }
        }
    }
}
#[component(no_case_check)]
pub fn NavbarRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p {
                strong { "Welcome 👋!" }
            }
        }
        p {
            "If you've ever built a frontend app in "
            strong { "Rust" }
            ", you've probably realized one thing real quick: "
            strong { "making a responsive, polished, and customizable navbar is surprisingly annoying" }
            ". Between layout quirks, dropdown handling, mega menu hacks, and mobile toggles... things get out of hand fast."
        }
        p {
            "That's why we're excited to introduce "
            a { href: "https://crates.io/crates/navbar",
                strong { "Navbar" }
            }
            ", a "
            strong { "fully-featured" }
            ", totally plug-and-play navigation component for "
            strong { "WASM" }
            " frameworks like Yew, Dioxus, and Leptos."
        }
        p {
            "It's sleek, smart, responsive, and "
            strong { "just works" }
            ", whether you're building a startup dashboard or the next social media empire."
        }
        p {
            "Let's take a bite into everything "
            strong { "Navbar" }
            " has to offer 🍟."
        }
        h2 { id: "-what-is-navbar",
            a { href: "#-what-is-navbar", class: "header", "🚀 What Is Navbar?" }
        }
        p {
            strong { "Navbar" }
            " is a fully-configurable component built for the "
            strong { "Yew" }
            " framework that gives you:"
        }
        ul {
            li { "A responsive layout with a mobile hamburger menu 🍔." }
            li {
                "Support for "
                strong { "mega menus" }
                ", "
                strong { "dropdowns" }
                ", and "
                strong { "call-to-action buttons" }
                "."
            }
            li {
                "Full "
                strong { "accessibility" }
                ", "
                strong { "custom styling" }
                ", and "
                strong { "state management" }
                " baked in."
            }
            li {
                "Easy integration with "
                code { "state" }
                ", "
                code { "Callback" }
                ", and other hooks."
            }
        }
        p { "No JavaScript duct tape. No style hacks. Just clean, idiomatic Rust and WebAssembly." }
        h2 { id: "-why-youll-love-it",
            a { href: "#-why-youll-love-it", class: "header", "⚡\u{fe0f} Why You'll Love It" }
        }
        p {
            "Most navbars feel like a chore. You paste in 200 lines of spaghetti HTML, try to wire up  "
            code { "onclick" }
            " logic, and still end up with a weird dropdown. "
            strong { "Navbar" }
            " takes care of "
            strong { "all that" }
            ". Out of the box, you get:"
        }
        ul {
            li { "✅ Mobile-first responsiveness." }
            li { "✅ Easy-to-add search bar." }
            li { "✅ Mega menu support for large content." }
            li { "✅ Profile dropdown menus." }
            li {
                "✅ Fully styleable with "
                code { "class" }
                " and "
                code { "style" }
                " props."
            }
            li { "✅ Event handling for resize, clicks, and toggles." }
            li { "✅ 100% declarative API." }
        }
        h2 { id: "-quick-setup-in-yew",
            a { href: "#-quick-setup-in-yew", class: "header", "🧰 Quick Setup in Yew" }
        }
        p { "Getting started is as easy as eating fries:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add navbar </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        p { "Then import and use it in your app:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">yew::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">navbar::yew::{{Navbar, Menu}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[function_component(App)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Html {{\n</span><span style=\"color:#f8f8f2;\">    html! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Navbar\n</span><span style=\"color:#f8f8f2;\">            logo_src</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/assets/logo.svg&quot;\n</span><span style=\"color:#f8f8f2;\">            logo_alt</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;My App&quot;\n</span><span style=\"color:#f8f8f2;\">            menus</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{vec![\n</span><span style=\"color:#f8f8f2;\">                Menu {{ id: </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, link: </span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">, name: </span><span style=\"color:#ffee99;\">&quot;Dashboard&quot;</span><span style=\"color:#f8f8f2;\">, icon_start: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">, icon_end: </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">                Menu {{ id: </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">, link: </span><span style=\"color:#ffee99;\">&quot;/reports&quot;</span><span style=\"color:#f8f8f2;\">, name: </span><span style=\"color:#ffee99;\">&quot;Reports&quot;</span><span style=\"color:#f8f8f2;\">, icon_start: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">, icon_end: </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            ]}}\n</span><span style=\"color:#f8f8f2;\">            button_text</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Sign Up&quot;\n</span><span style=\"color:#f8f8f2;\">            button_href</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/signup&quot;\n</span><span style=\"color:#f8f8f2;\">            show_search</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">            show_mega_menu</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">            show_profile_menu</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Boom 💥. You've got a fully interactive, responsive navbar." }
        h2 { id: "-feature-breakdown",
            a { href: "#-feature-breakdown", class: "header", "🧩 Feature Breakdown" }
        }
        p { "Let's talk toppings. Here's what comes with the Navbar combo meal:" }
        h3 { id: "-logo--menu",
            a { href: "#-logo--menu", class: "header", "🧭 Logo & Menu" }
        }
        p {
            "Display your logo with alt text and optional link, and set up top-level navigation with  "
            code { "Menu" }
            " items."
        }
        h3 { id: "-search-input",
            a { href: "#-search-input", class: "header", "🔍 Search Input" }
        }
        p {
            "Need a search bar? Just flip  "
            code { "show_search = true" }
            " and you're done. You can even manage the input state with  "
            code { "use_state" }
            "."
        }
        h3 { id: "-mega-menu",
            a { href: "#-mega-menu", class: "header", "🍕 Mega Menu" }
        }
        p {
            "Have a ton of links? Turn on  "
            code { "show_mega_menu" }
            " and pass in  "
            code { "mega_menu_items" }
            ". It works with hover and auto-closes cleanly."
        }
        h3 { id: "-profile-menu",
            a { href: "#-profile-menu", class: "header", "🙋 Profile Menu" }
        }
        p {
            "Want a profile dropdown? Set  "
            code { "show_profile_menu = true" }
            " and provide  "
            code { "dropdown_items" }
            " + a profile image URL."
        }
        h3 { id: "-mobile-support",
            a { href: "#-mobile-support", class: "header", "📱 Mobile Support" }
        }
        p {
            "Below 768px, the navbar switches to mobile mode with a hamburger toggle. Menu opens as a vertical drawer, no extra config needed."
        }
        h3 { id: "-full-styling-control",
            a { href: "#-full-styling-control", class: "header", "🎨 Full Styling Control" }
        }
        p {
            "Every single part of the navbar, logo, menu, button, dropdown, etc., can be styled via  "
            code { "class" }
            " and  "
            code { "style" }
            " props. Go wild."
        }
        h2 { id: "-developer-features",
            a { href: "#-developer-features", class: "header", "💻 Developer Features" }
        }
        p { "Here's where it gets extra juicy:" }
        table {
            thead {
                th { "Feature" }
                th { "What It Means for You" }
            }
            tr {
                th {
                    code { "use_state" }
                    " & "
                    code { "Callback" }
                    ""
                }
                th { "Fully interactive toggles without boilerplate" }
            }
            tr {
                th { "Auto-close dropdowns" }
                th { "Click outside? Menus close automatically 🙌" }
            }
            tr {
                th { "Resize listener" }
                th { "Reacts to screen size changes, real-time responsiveness" }
            }
            tr {
                th { "Conditional rendering" }
                th { "Don't need a feature? Just don't enable the prop!" }
            }
            tr {
                th { "Accessibility" }
                th { "ARIA-compliant, screen-reader friendly" }
            }
            tr {
                th { "Custom events" }
                th {
                    "Hook into "
                    code { "oninput" }
                    ", "
                    code { "onclick" }
                    ", and more"
                }
            }
        }
        p { "All of this wrapped in a clean component API that's easy to use." }
        h2 { id: "-full-props-reference",
            a { href: "#-full-props-reference", class: "header",
                "🎛\u{fe0f} Full Props Reference"
            }
        }
        p { "Too many props to list here, but here's a taste:" }
        ul {
            li {
                code { "logo_src" }
                ", "
                code { "logo_alt" }
                ", "
                code { "logo_link" }
            }
            li {
                code { "menus" }
                ", "
                code { "show_search" }
                ", "
                code { "search_placeholder" }
            }
            li {
                code { "button_text" }
                ", "
                code { "button_href" }
                ", "
                code { "button_target" }
            }
            li {
                code { "show_mega_menu" }
                ", "
                code { "mega_menu_items" }
            }
            li {
                code { "show_profile_menu" }
                ", "
                code { "dropdown_items" }
                ", "
                code { "profile_image_url" }
            }
            li {
                code { "navbar_style" }
                ", "
                code { "navbar_class" }
                ", "
                code { "container_style" }
                ", "
                code { "menu_item_class" }
                ", etc."
            }
        }
        p {
            "Check the "
            a { href: "https://docs.rs/navbar", "docs" }
            " or "
            a { href: "https://github.com/opensass/navbar/blob/ea46fa78290a311b16a91525a9fad2f88fc05e5f/src/yew.rs#L66",
                "source"
            }
            " for all props and default values."
        }
        h2 { id: "-real-world-scenarios",
            a { href: "#-real-world-scenarios", class: "header", "🧪 Real-World Scenarios" }
        }
        p { "Let's say you want:" }
        h3 { id: "a-clean-marketing-navbar",
            a { href: "#a-clean-marketing-navbar", class: "header", "A clean marketing navbar:" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Navbar\n</span><span style=\"color:#f8f8f2;\">    logo_src</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/logo.svg&quot;\n</span><span style=\"color:#f8f8f2;\">    logo_alt</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;My Brand&quot;\n</span><span style=\"color:#f8f8f2;\">    menus</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{vec![\n</span><span style=\"color:#f8f8f2;\">        Menu {{ id: </span><span style=\"color:#ff80f4;\">1</span><span style=\"color:#f8f8f2;\">, link: </span><span style=\"color:#ffee99;\">&quot;/&quot;</span><span style=\"color:#f8f8f2;\">, name: </span><span style=\"color:#ffee99;\">&quot;Home&quot;</span><span style=\"color:#f8f8f2;\">, icon_start: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">, icon_end: </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">        Menu {{ id: </span><span style=\"color:#ff80f4;\">2</span><span style=\"color:#f8f8f2;\">, link: </span><span style=\"color:#ffee99;\">&quot;/features&quot;</span><span style=\"color:#f8f8f2;\">, name: </span><span style=\"color:#ffee99;\">&quot;Features&quot;</span><span style=\"color:#f8f8f2;\">, icon_start: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">, icon_end: </span><span style=\"font-style:italic;color:#66d9ef;\">None </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">    ]}}\n</span><span style=\"color:#f8f8f2;\">    button_text</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Start Free Trial&quot;\n</span><span style=\"color:#f8f8f2;\">    button_href</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/signup&quot;\n</span><span style=\"color:#f8f8f2;\">    show_search</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ff80f4;\">false\n</span><span style=\"color:#f92672;\">/&gt;</span></pre>\n",
        }
        h3 { id: "a-complex-app-dashboard-navbar",
            a { href: "#a-complex-app-dashboard-navbar", class: "header",
                "A complex app dashboard navbar:"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Navbar\n</span><span style=\"color:#f8f8f2;\">    logo_src</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/logo.svg&quot;\n</span><span style=\"color:#f8f8f2;\">    logo_alt</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Dashboard&quot;\n</span><span style=\"color:#f8f8f2;\">    menus</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#f92672;\">...</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    show_search</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">    show_profile_menu</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">    dropdown_items</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{vec![\n</span><span style=\"color:#f8f8f2;\">        DropdownItem {{ text: </span><span style=\"color:#ffee99;\">&quot;Settings&quot;</span><span style=\"color:#f8f8f2;\">, link: </span><span style=\"color:#ffee99;\">&quot;/settings&quot; </span><span style=\"color:#f8f8f2;\">}},\n</span><span style=\"color:#f8f8f2;\">        DropdownItem {{ text: </span><span style=\"color:#ffee99;\">&quot;Logout&quot;</span><span style=\"color:#f8f8f2;\">, link: </span><span style=\"color:#ffee99;\">&quot;/logout&quot; </span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    ]}}\n</span><span style=\"color:#f8f8f2;\">    profile_image_url</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;/user.png&quot;\n</span><span style=\"color:#f92672;\">/&gt;</span></pre>\n",
        }
        h3 { id: "a-styled-custom-themed-navbar",
            a { href: "#a-styled-custom-themed-navbar", class: "header",
                "A styled, custom-themed navbar:"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Navbar\n</span><span style=\"color:#f8f8f2;\">    navbar_style</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;background: #0d1117; color: white;&quot;\n</span><span style=\"color:#f8f8f2;\">    button_style</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;background: #238636; border-radius: 4px;&quot;\n</span><span style=\"color:#f8f8f2;\">    search_input_style</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;border-radius: 4px; padding: 8px;&quot;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">...\n</span><span style=\"color:#f92672;\">/&gt;</span></pre>\n" }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "💡 Final Thoughts" }
        }
        p {
            "Your navbar is your app's "
            strong { "first impression" }
            ", don't settle for a janky, fragile mess of divs. With "
            strong { "Navbar" }
            ", you can build a beautiful, responsive, fully accessible header in "
            strong { "minutes" }
            ", not hours. Whether it's a static marketing site or a complex SPA, it scales with you. Fast, customizable, idiomatic, and built "
            strong { "for the modern WASM dev" }
            "."
        }
        blockquote {
            p {
                strong { "We are Open SASS, babe!" }
                "."
            }
        }
        blockquote {
            p { "We're working tirelessly on making Rust web development extremely easy for everyone." }
        }
        blockquote {
            p {
                "If you made it this far, it would be nice if you could "
                a { href: "https://discord.gg/b5JbvHW5nv", "join us on Discord" }
                "."
            }
        }
        blockquote {
            p { "Till next time 👋!" }
        }
    }
}

use dioxus::prelude::*;
use crate::components::blog::code::CodeBlock;
