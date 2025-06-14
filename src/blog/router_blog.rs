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
    #[route("/sidebar-release")]
    SidebarRelease {},
    #[route("/keep-using-aws")]
    KeepUsingAws {},
    #[route("/skeleton-rs-release")]
    SkeletonRsRelease {},
    #[route("/theme-release")]
    ThemeRelease {},
    #[route("/slider-rs-release")]
    SliderRsRelease {},
    #[route("/opensass-kit")]
    OpensassKit {},
    #[route("/hacking-dioxus")]
    HackingDioxus {},
    #[route("/browser-rs-release")]
    BrowserRsRelease {},
    #[route("/hero-release")]
    HeroRelease {},
    #[route("/pride-rs-release")]
    PrideRsRelease {},
    #[route("/pride-hero-release")]
    PrideHeroRelease {},
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
            BookRoute::SidebarRelease {} => use_mdbook::mdbook_shared::PageId(12usize),
            BookRoute::KeepUsingAws {} => use_mdbook::mdbook_shared::PageId(13usize),
            BookRoute::SkeletonRsRelease {} => use_mdbook::mdbook_shared::PageId(14usize),
            BookRoute::ThemeRelease {} => use_mdbook::mdbook_shared::PageId(15usize),
            BookRoute::SliderRsRelease {} => use_mdbook::mdbook_shared::PageId(16usize),
            BookRoute::OpensassKit {} => use_mdbook::mdbook_shared::PageId(17usize),
            BookRoute::HackingDioxus {} => use_mdbook::mdbook_shared::PageId(18usize),
            BookRoute::BrowserRsRelease {} => use_mdbook::mdbook_shared::PageId(19usize),
            BookRoute::HeroRelease {} => use_mdbook::mdbook_shared::PageId(20usize),
            BookRoute::PrideRsRelease {} => use_mdbook::mdbook_shared::PageId(21usize),
            BookRoute::PrideHeroRelease {} => use_mdbook::mdbook_shared::PageId(22usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::OpensassKit {}
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
                        title: " 1 |---| Announcing Open SASS 🚀 |---| announcement |---| announcing-opensass |---| Nov 10 2024 |---| Welcome to Open SASS. Your open-source platform for building the future of SaaS with Rust and Wasm. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/announcing-opensass |---| https://x.com/intent/post?url=https://opensass.org/blogs/announcing-opensass |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/announcing-opensass"
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
                        title: " 2 |---| ✨ Introducing X-AI |---| announcement |---| introducing-x-ai |---| Nov 18 2024 |---| Today, we are excited to announce the release of 𝕏-AI, your gateway to the X-AI API in Rust. |---| https://github.com/user-attachments/assets/e18b9fc2-7b7d-4125-86fe-c1b91fdb0f93 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/introducing-x-ai |---| https://x.com/intent/post?url=https://opensass.org/blogs/introducing-x-ai |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/introducing-x-ai"
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
                        title: " 3 |---| 💡 AIBook v0.0.2 Release |---| announcement |---| aibook-v002-release |---| Nov 22 2024 |---| Welcome back to our blog! We are excited to announce the release of aibook. |---| https://github.com/user-attachments/assets/ec4e080f-37af-4e62-af40-f0bb92d28bff |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/aibook-v002-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/aibook-v002-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/aibook-v002-release"
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
                        title: " 4 |---| Introducing Tripper ✈\u{fe0f} |---| announcement |---| tripper-v001-release |---| Nov 28 2024 |---| Welcome back to our blog! We are excited to announce the release of aibook. |---| https://github.com/user-attachments/assets/d18cb450-f4c7-4455-a9c2-b0f165889487 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/tripper-v001-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/tripper-v001-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/tripper-v001-release"
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
                        title: " 5 |---| Beyond TypeScript |---| blog |---| beyond-typescript |---| Apr 20 2025 |---| Hey devs, and anyone still dealing with a 900MB node_modules folder. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/beyond-typescript |---| https://x.com/intent/post?url=https://opensass.org/blogs/beyond-typescript |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/beyond-typescript"
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
                        title: " 6 |---| Biblically Accurate Rust 😇 |---| blog |---| rust-is-god-101 |---| Apr 22 2025 |---| Rust is the biblically accurate programming language; Fast, safe, and blessed with memory safety. |---| https://github.com/user-attachments/assets/a9fc3f6f-8ad3-40b9-92c8-233efa64acc0 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/rust-is-god-101 |---| https://x.com/intent/post?url=https://opensass.org/blogs/rust-is-god-101 |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/rust-is-god-101"
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
                        title: " 7 |---| Open SASS vs ShadCN UI ⚔\u{fe0f} |---| blog |---| opensass-vs-shadcn |---| Apr 23 2025 |---| Open SASS obliterates ShadCN UI with framework-agnostic components, blazing performance, and unmatched versatility. |---| https://github.com/user-attachments/assets/5e2bf427-0401-4cf6-9c72-d9bffb445ee0 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/opensass-vs-shadcn |---| https://x.com/intent/post?url=https://opensass.org/blogs/opensass-vs-shadcn |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/opensass-vs-shadcn"
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
                        title: " 8 |---| Image RS: Next-Gen WASM Image Component 🚀 |---| announcement |---| image-rs-release |---| Apr 26 2025 |---| Image RS launches as the ultimate image solution for Yew, Dioxus, and Leptos apps with smart lazy loading, responsive layouts, accessibility, and incredible flexibility. |---| https://raw.githubusercontent.com/opensass/image-rs/refs/heads/main/assets/logo.webp |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/image-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/image-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/image-rs-release"
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
                        title: " 9 |---| Image RS vs Next.js Image 🔥 |---| blog |---| image-rs-vs-next-js-image |---| Apr 27 2025 |---| A deep comparison proving why Yew Image RS outperforms Next.js Image with native WASM speed, fine-grained DOM control, better memory usage, and smoother performance at scale. |---| https://github.com/user-attachments/assets/9fa9ff50-32ea-4369-a263-0bb8c32197c1 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/image-rs-vs-next-js-image |---| https://x.com/intent/post?url=https://opensass.org/blogs/image-rs-vs-next-js-image |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/image-rs-vs-next-js-image"
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
                        title: " 10 |---| Table RS: Advanced Wasmy Table Component 📊 |---| announcement |---| table-rs-release |---| Apr 29 2025 |---| Table RS delivers a fully-featured, accessible, and customizable table component for Wasm apps with built-in search, sorting, pagination, and styling control. |---| https://raw.githubusercontent.com/opensass/table-rs/refs/heads/main/assets/logo.webp |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/table-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/table-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/table-rs-release"
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
                        title: " 11 |---| Table RS: Why WASM Outperforms JS at Scale 📈 |---| blog |---| tanstack-table-vs-table-rs |---| Apr 29 2025 |---| A deep-dive benchmark comparing TanStack Table (React) vs Table RS (Yew + WASM). |---| https://github.com/user-attachments/assets/2cd8279a-9d9d-4f75-bf13-b61fbbb130da |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/tanstack-table-vs-table-rs |---| https://x.com/intent/post?url=https://opensass.org/blogs/tanstack-table-vs-table-rs |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/tanstack-table-vs-table-rs"
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
                        title: " 12 |---| 🍔 Navbar: A Deliciously Simple Wasmy Navbar Component |---| announcement |---| navbar-release |---| May 01 2025 |---| A hands-on guide to adding a feature-rich, fully customizable Navbar component to your WASM app. |---| https://github.com/user-attachments/assets/1fa1e562-8861-4dd9-99af-060c768a23a7 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/navbar-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/navbar-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/navbar-release"
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
        pages
            .push((
                12usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 13 |---| 🗃\u{fe0f} Sidebar: The Composable Wasmy Sidebar |---| announcement |---| sidebar-release |---| May 02 2025 |---| A deep dive into Sidebar: a modular, fully styleable sidebar component for Yew, Dioxus, and Leptos. |---| https://github.com/user-attachments/assets/60adb866-9821-4efc-a274-46eecfd48f48 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/sidebar-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/sidebar-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/sidebar-release"
                            .to_string(),
                        url: BookRoute::SidebarRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🗃\u{fe0f} What Is Sidebar?".to_string(),
                                id: "🗃\u{fe0f}-what-is-sidebar?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚡\u{fe0f} Why You'll Love It".to_string(),
                                id: "⚡\u{fe0f}-why-you'll-love-it".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Quick Start with Yew".to_string(),
                                id: "🚀-quick-start-with-yew".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔍 What You Can Customize (Spoiler: Everything)"
                                    .to_string(),
                                id: "🔍-what-you-can-customize-(spoiler:-everything)"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧱 ".to_string(),
                                id: "🧱".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📋 ".to_string(),
                                id: "📋".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔽 ".to_string(),
                                id: "🔽".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🙋 ".to_string(),
                                id: "🙋".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "💡 Final Thoughts".to_string(),
                                id: "💡-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(12usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::SidebarRelease {},
            ::use_mdbook::mdbook_shared::PageId(12usize),
        );
        pages
            .push((
                13usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 14 |---| ☁\u{fe0f} Keep Using AWS as Usual |---| blog |---| keep-using-aws |---| May 04 2025 |---| A no-BS, defense of AWS and why Rust should be your default stack language. |---| https://github.com/user-attachments/assets/e9b7f007-4337-4881-9d5f-ea42c98a567a |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/keep-using-aws |---| https://x.com/intent/post?url=https://opensass.org/blogs/keep-using-aws |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/keep-using-aws"
                            .to_string(),
                        url: BookRoute::KeepUsingAws {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🛑 Stop Using AWS".to_string(),
                                id: "🛑-stop-using-aws".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚂 Copying Netflix Architecture Is Silly"
                                    .to_string(),
                                id: "🚂-copying-netflix-architecture-is-silly"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Most Projects Die From Lack of Users, Not Overengineering"
                                    .to_string(),
                                id: "🚀-most-projects-die-from-lack-of-users,-not-overengineering"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔧 Just Use a VPS + Docker Compose".to_string(),
                                id: "🔧-just-use-a-vps-+-docker-compose".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧠 AWS Is Too Complicated".to_string(),
                                id: "🧠-aws-is-too-complicated".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "✅ When AWS Makes Sense: Always".to_string(),
                                id: "✅-when-aws-makes-sense:-always".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🦀 Why You Should Use Rust for Everything, Everywhere, Always"
                                    .to_string(),
                                id: "🦀-why-you-should-use-rust-for-everything,-everywhere,-always"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔚 Final Thoughts: Your Stack Shouldn't Be a Toy"
                                    .to_string(),
                                id: "🔚-final-thoughts:-your-stack-shouldn't-be-a-toy"
                                    .to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(13usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::KeepUsingAws {},
            ::use_mdbook::mdbook_shared::PageId(13usize),
        );
        pages
            .push((
                14usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 15 |---| 🦴 Seamless Loading with Skeleton RS |---| announcement |---| skeleton-rs-release |---| May 06 2025 |---| A next-gen, zero-clutter skeleton loader for Rust WASM apps. |---| https://github.com/user-attachments/assets/eea87d4d-58a9-4a95-b8f3-57a600c1840b |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/skeleton-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/skeleton-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/skeleton-rs-release"
                            .to_string(),
                        url: BookRoute::SkeletonRsRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🦴 What Is Skeleton RS?".to_string(),
                                id: "🦴-what-is-skeleton-rs?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Built-in UX Goodness".to_string(),
                                id: "built-in-ux-goodness".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Why You'll Love Skeleton RS".to_string(),
                                id: "🚀-why-you'll-love-skeleton-rs".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚡ Quick Yew Setup".to_string(),
                                id: "⚡-quick-yew-setup".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "1. Add it to your dependencies".to_string(),
                                id: "1.-add-it-to-your-dependencies".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "2. Import the component".to_string(),
                                id: "2.-import-the-component".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "3. Drop it into your app".to_string(),
                                id: "3.-drop-it-into-your-app".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧩 Features".to_string(),
                                id: "🧩-features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚙\u{fe0f} Full Control with Props".to_string(),
                                id: "⚙\u{fe0f}-full-control-with-props".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧾 Text Loading Placeholder".to_string(),
                                id: "🧾-text-loading-placeholder".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🟦 Avatar Circle".to_string(),
                                id: "🟦-avatar-circle".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📦 Card Block".to_string(),
                                id: "📦-card-block".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔁 With Child Content (Infer Size)".to_string(),
                                id: "🔁-with-child-content-(infer-size)".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🕵\u{fe0f} Animate Only When Visible".to_string(),
                                id: "🕵\u{fe0f}-animate-only-when-visible".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "💡 Final Thoughts".to_string(),
                                id: "💡-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(14usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::SkeletonRsRelease {},
            ::use_mdbook::mdbook_shared::PageId(14usize),
        );
        pages
            .push((
                15usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 16 |---| 🎨 Infinite Theming with Theme |---| announcement |---| theme-release |---| May 09 2025 |---| A powerful theme manager for Rust + WASM apps with full system support and custom themes. |---| https://github.com/user-attachments/assets/bafca6ab-045b-48cf-bba5-f0f385b3bc3c |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/theme-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/theme-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/theme-release"
                            .to_string(),
                        url: BookRoute::ThemeRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🌈 What Is ".to_string(),
                                id: "🌈-what-is".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚡ Quick Setup".to_string(),
                                id: "⚡-quick-setup".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "1. Add it to your ".to_string(),
                                id: "1.-add-it-to-your".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "2. Import the magic".to_string(),
                                id: "2.-import-the-magic".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "3. Wrap your app".to_string(),
                                id: "3.-wrap-your-app".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎨 Add Your Own Themes".to_string(),
                                id: "🎨-add-your-own-themes".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧠 Theme Context Hook? Yes, please!".to_string(),
                                id: "🧠-theme-context-hook?-yes,-please!".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧰 Tailwind, Meet Theme".to_string(),
                                id: "🧰-tailwind,-meet-theme".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧪 Full Control with Props".to_string(),
                                id: "🧪-full-control-with-props".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧠 Bonus Brainy Features".to_string(),
                                id: "🧠-bonus-brainy-features".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Final Thoughts".to_string(),
                                id: "🚀-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(15usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::ThemeRelease {},
            ::use_mdbook::mdbook_shared::PageId(15usize),
        );
        pages
            .push((
                16usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 17 |---| 🎚\u{fe0f} Precision Sliders with Slider RS |---| announcement |---| slider-rs-release |---| May 13 2025 |---| A highly customizable, accessible slider component for Rust + WASM apps with single & range support. |---| https://github.com/user-attachments/assets/77d554ca-09cd-4b36-99bf-e08f9154dab4 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/slider-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/slider-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/slider-rs-release"
                            .to_string(),
                        url: BookRoute::SliderRsRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎉 Why Slider RS Exists".to_string(),
                                id: "🎉-why-slider-rs-exists".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧰 What Can Slider RS Do?".to_string(),
                                id: "🧰-what-can-slider-rs-do?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Getting Started".to_string(),
                                id: "🚀-getting-started".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "1. Add Slider RS to Your Project".to_string(),
                                id: "1.-add-slider-rs-to-your-project".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "2. Import the Slider Component".to_string(),
                                id: "2.-import-the-slider-component".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "3. Use It In Your App".to_string(),
                                id: "3.-use-it-in-your-app".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎨 But Wait, There's More".to_string(),
                                id: "🎨-but-wait,-there's-more".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🦀 Ferris Approved, Open SASS Blessed"
                                    .to_string(),
                                id: "🦀-ferris-approved,-open-sass-blessed".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "💬 Join the Party".to_string(),
                                id: "💬-join-the-party".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Final Thoughts".to_string(),
                                id: "🚀-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(16usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::SliderRsRelease {},
            ::use_mdbook::mdbook_shared::PageId(16usize),
        );
        pages
            .push((
                17usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 18 |---| 🧰 Open SASS Kit: The Universal UI Toolkit |---| announcement |---| opensass-kit |---| May 22 2025 |---| A centralized, CSS-framework-agnostic component hub for building fast, modular Rust + WASM web apps with CLI-powered scaffolding. |---| https://github.com/user-attachments/assets/11fd714b-498b-4673-b659-ce19ab095492 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/opensass-kit |---| https://x.com/intent/post?url=https://opensass.org/blogs/opensass-kit |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/opensass-kit"
                            .to_string(),
                        url: BookRoute::OpensassKit {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧰 What Is It?".to_string(),
                                id: "🧰-what-is-it?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎨 But, Why?".to_string(),
                                id: "🎨-but,-why?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚡ How It Works?".to_string(),
                                id: "⚡-how-it-works?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📚 A Living Library of Components".to_string(),
                                id: "📚-a-living-library-of-components".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧪 Built for WASM Frameworks".to_string(),
                                id: "🧪-built-for-wasm-frameworks".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🤝 Join the Party".to_string(),
                                id: "🤝-join-the-party".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "📈 This Kit Is Just Getting Started".to_string(),
                                id: "📈-this-kit-is-just-getting-started".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎤 Final Thoughts".to_string(),
                                id: "🎤-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(17usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::OpensassKit {},
            ::use_mdbook::mdbook_shared::PageId(17usize),
        );
        pages
            .push((
                18usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 19 |---| 👨🏻\u{200d}💻 Hacking Dioxus: How Vibe Coding Is Destroying Software Engineering |---| blog |---| hacking-dioxus |---| May 26 2025 |---| A critical reflection on security flaws found in the Dioxus UI framework. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/jw39u3jm6nq5mu2qlre2.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/hacking-dioxus |---| https://x.com/intent/post?url=https://opensass.org/blogs/hacking-dioxus |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/hacking-dioxus"
                            .to_string(),
                        url: BookRoute::HackingDioxus {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "TL;DR".to_string(),
                                id: "tl;dr".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Introduction".to_string(),
                                id: "introduction".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What is a Software Engineer / Developer?"
                                    .to_string(),
                                id: "what-is-a-software-engineer-/-developer?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What is Vibe Coding?".to_string(),
                                id: "what-is-vibe-coding?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Hacking Dioxus".to_string(),
                                id: "hacking-dioxus".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Open Redirect Vulnerability".to_string(),
                                id: "open-redirect-vulnerability".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "CSRF in Server Functions".to_string(),
                                id: "csrf-in-server-functions".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "DoS Caused by Arbitrary Function Pointer Transmute"
                                    .to_string(),
                                id: "dos-caused-by-arbitrary-function-pointer-transmute"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "SSRF in CLI SSG Loop".to_string(),
                                id: "ssrf-in-cli-ssg-loop".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Reflection".to_string(),
                                id: "reflection".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Final Thoughts".to_string(),
                                id: "final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(18usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::HackingDioxus {},
            ::use_mdbook::mdbook_shared::PageId(18usize),
        );
        pages
            .push((
                19usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 20 |---| 🧭 Browser RS: A Wasmy Browser Frame Component |---| announcement |---| browser-rs-release |---| June 1 2025 |---| A highly customizable browser frame component. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/7ooyaxds6u8srabzky3y.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/browser-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/browser-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/browser-rs-release"
                            .to_string(),
                        url: BookRoute::BrowserRsRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🌐 What Is Browser RS?".to_string(),
                                id: "🌐-what-is-browser-rs?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🚀 Why You'll Love Browser RS".to_string(),
                                id: "🚀-why-you'll-love-browser-rs".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🔥 Getting Started with Yew".to_string(),
                                id: "🔥-getting-started-with-yew".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Add the Crate".to_string(),
                                id: "add-the-crate".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Import the Component".to_string(),
                                id: "import-the-component".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Wrap Your Content".to_string(),
                                id: "wrap-your-content".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧩 Features That Matter".to_string(),
                                id: "🧩-features-that-matter".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎨 Styled to Match".to_string(),
                                id: "🎨-styled-to-match".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🎛\u{fe0f} Full Control via Props".to_string(),
                                id: "🎛\u{fe0f}-full-control-via-props".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🛠\u{fe0f} Real-World Use Cases".to_string(),
                                id: "🛠\u{fe0f}-real-world-use-cases".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "💬 Final Thoughts".to_string(),
                                id: "💬-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(19usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::BrowserRsRelease {},
            ::use_mdbook::mdbook_shared::PageId(19usize),
        );
        pages
            .push((
                20usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 21 |---| 🦸 Hero: A WASM-Ready Hero Section Component |---| announcement |---| hero-release |---| June 6 2025 |---| A fully customizable hero section component for Yew, Leptos, and Dioxus. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/im1r51obcsp1lc1xvwsv.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/hero-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/hero-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/hero-release"
                            .to_string(),
                        url: BookRoute::HeroRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Is Hero?".to_string(),
                                id: "what-is-hero?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why Hero?".to_string(),
                                id: "why-hero?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Getting Started with Yew".to_string(),
                                id: "getting-started-with-yew".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Add the Crate".to_string(),
                                id: "add-the-crate".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Import the Hero You Need".to_string(),
                                id: "import-the-hero-you-need".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Basic Example".to_string(),
                                id: "basic-example".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Tailwind Friendly".to_string(),
                                id: "tailwind-friendly".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Full Control via Props".to_string(),
                                id: "full-control-via-props".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Feature Recap".to_string(),
                                id: "feature-recap".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Final Thoughts".to_string(),
                                id: "final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(20usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::HeroRelease {},
            ::use_mdbook::mdbook_shared::PageId(20usize),
        );
        pages
            .push((
                21usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 22 |---| 🏳\u{fe0f}\u{200d}🌈 Pride RS: LGBTQ+ Flag Component for Rust Frontends |---| announcement |---| pride-rs-release |---| June 9 2025 |---| A customizable and accessible pride flag component for rusty apps, with built-in LGBTQ+ flag definitions and flexible layouts. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/ykytq28vzedk9t5bvm76.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/pride-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/pride-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/pride-rs-release"
                            .to_string(),
                        url: BookRoute::PrideRsRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🏳\u{fe0f}\u{200d}🌈 What Is Pride RS?"
                                    .to_string(),
                                id: "🏳\u{fe0f}\u{200d}🌈-what-is-pride-rs?"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "⚙\u{fe0f} Under the Hood".to_string(),
                                id: "⚙\u{fe0f}-under-the-hood".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Flag Sizes".to_string(),
                                id: "flag-sizes".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Direction (Layout)".to_string(),
                                id: "direction-(layout)".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Style Everything".to_string(),
                                id: "style-everything".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧰 Getting Started with Yew".to_string(),
                                id: "🧰-getting-started-with-yew".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Step 1: Add the Crate".to_string(),
                                id: "step-1:-add-the-crate".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Step 2: Use the Component".to_string(),
                                id: "step-2:-use-the-component".to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧑\u{200d}🎨 Real Use Cases".to_string(),
                                id: "🧑\u{200d}🎨-real-use-cases".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🛠\u{fe0f} What's Next?".to_string(),
                                id: "🛠\u{fe0f}-what's-next?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "🧑\u{200d}⚖\u{fe0f} For Judges".to_string(),
                                id: "🧑\u{200d}⚖\u{fe0f}-for-judges".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "💬 Final Thoughts".to_string(),
                                id: "💬-final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(21usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::PrideRsRelease {},
            ::use_mdbook::mdbook_shared::PageId(21usize),
        );
        pages
            .push((
                22usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 23 |---| 🏳\u{fe0f}\u{200d}⚧\u{fe0f} Pride Hero: LGBTQ+ Landing Page for WASM Frameworks |---| announcement |---| pride-hero-release |---| June 10 2025 |---| A customizable and accessible LGBTQ+ Landing Page component for rusty apps, powered by pride rs. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/xnhf4zbiu8azolmpc5wo.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/pride-hero-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/pride-hero-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/pride-hero-release"
                            .to_string(),
                        url: BookRoute::PrideHeroRelease {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Is ".to_string(),
                                id: "what-is".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Demo".to_string(),
                                id: "demo".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why This Exists".to_string(),
                                id: "why-this-exists".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Canon Update".to_string(),
                                id: "canon-update".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Future Ideas".to_string(),
                                id: "future-ideas".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Community Love".to_string(),
                                id: "community-love".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "For Judges".to_string(),
                                id: "for-judges".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Final Thoughts".to_string(),
                                id: "final-thoughts".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(22usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::PrideHeroRelease {},
            ::use_mdbook::mdbook_shared::PageId(22usize),
        );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 1 |---| Announcing Open SASS 🚀 |---| announcement |---| announcing-opensass |---| Nov 10 2024 |---| Welcome to Open SASS. Your open-source platform for building the future of SaaS with Rust and Wasm. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/announcing-opensass |---| https://x.com/intent/post?url=https://opensass.org/blogs/announcing-opensass |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/announcing-opensass"
                            .to_string(),
                        location: Some(BookRoute::AnnouncingOpensass {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 2 |---| ✨ Introducing X-AI |---| announcement |---| introducing-x-ai |---| Nov 18 2024 |---| Today, we are excited to announce the release of 𝕏-AI, your gateway to the X-AI API in Rust. |---| https://github.com/user-attachments/assets/e18b9fc2-7b7d-4125-86fe-c1b91fdb0f93 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/introducing-x-ai |---| https://x.com/intent/post?url=https://opensass.org/blogs/introducing-x-ai |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/introducing-x-ai"
                            .to_string(),
                        location: Some(BookRoute::IntroducingXAi {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 3 |---| 💡 AIBook v0.0.2 Release |---| announcement |---| aibook-v002-release |---| Nov 22 2024 |---| Welcome back to our blog! We are excited to announce the release of aibook. |---| https://github.com/user-attachments/assets/ec4e080f-37af-4e62-af40-f0bb92d28bff |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/aibook-v002-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/aibook-v002-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/aibook-v002-release"
                            .to_string(),
                        location: Some(BookRoute::AibookV002Release {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 4 |---| Introducing Tripper ✈\u{fe0f} |---| announcement |---| tripper-v001-release |---| Nov 28 2024 |---| Welcome back to our blog! We are excited to announce the release of aibook. |---| https://github.com/user-attachments/assets/d18cb450-f4c7-4455-a9c2-b0f165889487 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/tripper-v001-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/tripper-v001-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/tripper-v001-release"
                            .to_string(),
                        location: Some(BookRoute::TripperV001Release {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 5 |---| Beyond TypeScript |---| blog |---| beyond-typescript |---| Apr 20 2025 |---| Hey devs, and anyone still dealing with a 900MB node_modules folder. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/beyond-typescript |---| https://x.com/intent/post?url=https://opensass.org/blogs/beyond-typescript |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/beyond-typescript"
                            .to_string(),
                        location: Some(BookRoute::BeyondTypescript {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 6 |---| Biblically Accurate Rust 😇 |---| blog |---| rust-is-god-101 |---| Apr 22 2025 |---| Rust is the biblically accurate programming language; Fast, safe, and blessed with memory safety. |---| https://github.com/user-attachments/assets/a9fc3f6f-8ad3-40b9-92c8-233efa64acc0 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/rust-is-god-101 |---| https://x.com/intent/post?url=https://opensass.org/blogs/rust-is-god-101 |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/rust-is-god-101"
                            .to_string(),
                        location: Some(BookRoute::RustIsGod101 {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![6u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 7 |---| Open SASS vs ShadCN UI ⚔\u{fe0f} |---| blog |---| opensass-vs-shadcn |---| Apr 23 2025 |---| Open SASS obliterates ShadCN UI with framework-agnostic components, blazing performance, and unmatched versatility. |---| https://github.com/user-attachments/assets/5e2bf427-0401-4cf6-9c72-d9bffb445ee0 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/opensass-vs-shadcn |---| https://x.com/intent/post?url=https://opensass.org/blogs/opensass-vs-shadcn |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/opensass-vs-shadcn"
                            .to_string(),
                        location: Some(BookRoute::OpensassVsShadcn {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![7u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 8 |---| Image RS: Next-Gen WASM Image Component 🚀 |---| announcement |---| image-rs-release |---| Apr 26 2025 |---| Image RS launches as the ultimate image solution for Yew, Dioxus, and Leptos apps with smart lazy loading, responsive layouts, accessibility, and incredible flexibility. |---| https://raw.githubusercontent.com/opensass/image-rs/refs/heads/main/assets/logo.webp |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/image-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/image-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/image-rs-release"
                            .to_string(),
                        location: Some(BookRoute::ImageRsRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![8u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 9 |---| Image RS vs Next.js Image 🔥 |---| blog |---| image-rs-vs-next-js-image |---| Apr 27 2025 |---| A deep comparison proving why Yew Image RS outperforms Next.js Image with native WASM speed, fine-grained DOM control, better memory usage, and smoother performance at scale. |---| https://github.com/user-attachments/assets/9fa9ff50-32ea-4369-a263-0bb8c32197c1 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/image-rs-vs-next-js-image |---| https://x.com/intent/post?url=https://opensass.org/blogs/image-rs-vs-next-js-image |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/image-rs-vs-next-js-image"
                            .to_string(),
                        location: Some(BookRoute::ImageRsVsNextJsImage {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![9u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 10 |---| Table RS: Advanced Wasmy Table Component 📊 |---| announcement |---| table-rs-release |---| Apr 29 2025 |---| Table RS delivers a fully-featured, accessible, and customizable table component for Wasm apps with built-in search, sorting, pagination, and styling control. |---| https://raw.githubusercontent.com/opensass/table-rs/refs/heads/main/assets/logo.webp |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/table-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/table-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/table-rs-release"
                            .to_string(),
                        location: Some(BookRoute::TableRsRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![10u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 11 |---| Table RS: Why WASM Outperforms JS at Scale 📈 |---| blog |---| tanstack-table-vs-table-rs |---| Apr 29 2025 |---| A deep-dive benchmark comparing TanStack Table (React) vs Table RS (Yew + WASM). |---| https://github.com/user-attachments/assets/2cd8279a-9d9d-4f75-bf13-b61fbbb130da |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/tanstack-table-vs-table-rs |---| https://x.com/intent/post?url=https://opensass.org/blogs/tanstack-table-vs-table-rs |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/tanstack-table-vs-table-rs"
                            .to_string(),
                        location: Some(BookRoute::TanstackTableVsTableRs {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![11u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 12 |---| 🍔 Navbar: A Deliciously Simple Wasmy Navbar Component |---| announcement |---| navbar-release |---| May 01 2025 |---| A hands-on guide to adding a feature-rich, fully customizable Navbar component to your WASM app. |---| https://github.com/user-attachments/assets/1fa1e562-8861-4dd9-99af-060c768a23a7 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/navbar-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/navbar-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/navbar-release"
                            .to_string(),
                        location: Some(BookRoute::NavbarRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![12u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 13 |---| 🗃\u{fe0f} Sidebar: The Composable Wasmy Sidebar |---| announcement |---| sidebar-release |---| May 02 2025 |---| A deep dive into Sidebar: a modular, fully styleable sidebar component for Yew, Dioxus, and Leptos. |---| https://github.com/user-attachments/assets/60adb866-9821-4efc-a274-46eecfd48f48 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/sidebar-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/sidebar-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/sidebar-release"
                            .to_string(),
                        location: Some(BookRoute::SidebarRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![13u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 14 |---| ☁\u{fe0f} Keep Using AWS as Usual |---| blog |---| keep-using-aws |---| May 04 2025 |---| A no-BS, defense of AWS and why Rust should be your default stack language. |---| https://github.com/user-attachments/assets/e9b7f007-4337-4881-9d5f-ea42c98a567a |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/keep-using-aws |---| https://x.com/intent/post?url=https://opensass.org/blogs/keep-using-aws |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/keep-using-aws"
                            .to_string(),
                        location: Some(BookRoute::KeepUsingAws {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![14u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 15 |---| 🦴 Seamless Loading with Skeleton RS |---| announcement |---| skeleton-rs-release |---| May 06 2025 |---| A next-gen, zero-clutter skeleton loader for Rust WASM apps. |---| https://github.com/user-attachments/assets/eea87d4d-58a9-4a95-b8f3-57a600c1840b |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/skeleton-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/skeleton-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/skeleton-rs-release"
                            .to_string(),
                        location: Some(BookRoute::SkeletonRsRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![15u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 16 |---| 🎨 Infinite Theming with Theme |---| announcement |---| theme-release |---| May 09 2025 |---| A powerful theme manager for Rust + WASM apps with full system support and custom themes. |---| https://github.com/user-attachments/assets/bafca6ab-045b-48cf-bba5-f0f385b3bc3c |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/theme-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/theme-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/theme-release"
                            .to_string(),
                        location: Some(BookRoute::ThemeRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![16u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 17 |---| 🎚\u{fe0f} Precision Sliders with Slider RS |---| announcement |---| slider-rs-release |---| May 13 2025 |---| A highly customizable, accessible slider component for Rust + WASM apps with single & range support. |---| https://github.com/user-attachments/assets/77d554ca-09cd-4b36-99bf-e08f9154dab4 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/slider-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/slider-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/slider-rs-release"
                            .to_string(),
                        location: Some(BookRoute::SliderRsRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![17u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 18 |---| 🧰 Open SASS Kit: The Universal UI Toolkit |---| announcement |---| opensass-kit |---| May 22 2025 |---| A centralized, CSS-framework-agnostic component hub for building fast, modular Rust + WASM web apps with CLI-powered scaffolding. |---| https://github.com/user-attachments/assets/11fd714b-498b-4673-b659-ce19ab095492 |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/opensass-kit |---| https://x.com/intent/post?url=https://opensass.org/blogs/opensass-kit |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/opensass-kit"
                            .to_string(),
                        location: Some(BookRoute::OpensassKit {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![18u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 19 |---| 👨🏻\u{200d}💻 Hacking Dioxus: How Vibe Coding Is Destroying Software Engineering |---| blog |---| hacking-dioxus |---| May 26 2025 |---| A critical reflection on security flaws found in the Dioxus UI framework. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/jw39u3jm6nq5mu2qlre2.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/hacking-dioxus |---| https://x.com/intent/post?url=https://opensass.org/blogs/hacking-dioxus |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/hacking-dioxus"
                            .to_string(),
                        location: Some(BookRoute::HackingDioxus {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![19u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 20 |---| 🧭 Browser RS: A Wasmy Browser Frame Component |---| announcement |---| browser-rs-release |---| June 1 2025 |---| A highly customizable browser frame component. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/7ooyaxds6u8srabzky3y.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/browser-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/browser-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/browser-rs-release"
                            .to_string(),
                        location: Some(BookRoute::BrowserRsRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![20u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 21 |---| 🦸 Hero: A WASM-Ready Hero Section Component |---| announcement |---| hero-release |---| June 6 2025 |---| A fully customizable hero section component for Yew, Leptos, and Dioxus. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/im1r51obcsp1lc1xvwsv.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/hero-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/hero-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/hero-release"
                            .to_string(),
                        location: Some(BookRoute::HeroRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![21u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 22 |---| 🏳\u{fe0f}\u{200d}🌈 Pride RS: LGBTQ+ Flag Component for Rust Frontends |---| announcement |---| pride-rs-release |---| June 9 2025 |---| A customizable and accessible pride flag component for rusty apps, with built-in LGBTQ+ flag definitions and flexible layouts. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/ykytq28vzedk9t5bvm76.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/pride-rs-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/pride-rs-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/pride-rs-release"
                            .to_string(),
                        location: Some(BookRoute::PrideRsRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![22u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 23 |---| 🏳\u{fe0f}\u{200d}⚧\u{fe0f} Pride Hero: LGBTQ+ Landing Page for WASM Frameworks |---| announcement |---| pride-hero-release |---| June 10 2025 |---| A customizable and accessible LGBTQ+ Landing Page component for rusty apps, powered by pride rs. |---| https://dev-to-uploads.s3.amazonaws.com/uploads/articles/xnhf4zbiu8azolmpc5wo.png |---| https://www.facebook.com/sharer/sharer.php?u=https://opensass.org/blogs/pride-hero-release |---| https://x.com/intent/post?url=https://opensass.org/blogs/pride-hero-release |---| https://www.linkedin.com/feed/?shareActive=true&shareUrl=https://opensass.org/blogs/pride-hero-release"
                            .to_string(),
                        location: Some(BookRoute::PrideHeroRelease {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![23u32]),
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
#[component(no_case_check)]
pub fn SidebarRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p {
                strong { "Welcome 👋!" }
            }
        }
        p {
            "If you've ever built a "
            strong { "Rust-powered frontend" }
            ", you've probably felt this pain: "
            em {
                "\"How the heck do I make a sidebar that doesn't look like it crawled out of the year 2001?\""
            }
        }
        p {
            "We get it. Collapsibles, user profiles, nested menus, custom styles... you need a PhD in CSS gymnastics and a minor in JavaScript contortionism just to get a halfway-decent result."
        }
        p {
            strong { "That's so over, and we are so back!" }
        }
        p {
            "Introducing "
            a { href: "https://crates.io/crates/sidebar",
                strong { "Sidebar" }
            }
            ", the "
            strong { "ultimate plug-and-play sidebar component" }
            " for "
            strong { "Yew" }
            ", "
            strong { "Dioxus" }
            ", and "
            strong { "Leptos" }
            ". It's clean, fast, accessible, and stupidly customizable. And yep, "
            strong { "it just works" }
            "."
        }
        p { "Let's open it up 🧰." }
        p {
            img {
                src: "https://github.com/user-attachments/assets/c583f235-17c3-496e-9689-35db043ebff3",
                alt: "vibin",
                title: "",
            }
        }
        h2 { id: "-what-is-sidebar",
            a { href: "#-what-is-sidebar", class: "header", "🗃\u{fe0f} What Is Sidebar?" }
        }
        p {
            strong { "Sidebar" }
            " is a "
            strong { "fully modular" }
            ", "
            strong { "highly customizable" }
            " sidebar component built specifically for "
            strong { "WASM UI frameworks" }
            ". It's designed to make your app feel like a well-polished dashboard without you rage-quitting halfway through."
        }
        p { "Here's what it gives you out of the box:" }
        ul {
            li { "✅ Collapsible layout with smooth toggle." }
            li { "✅ Nested submenus (with disclosure arrows!)." }
            li { "✅ Badges, icons, and dynamic styling." }
            li { "✅ Built-in profile section with avatar + logout." }
            li {
                "✅ Full control via "
                code { "props" }
                ", "
                code { "class" }
                ", and "
                code { "style" }
                "."
            }
            li {
                "✅ Idiomatic Rust state management with "
                code { "UseStateHandle" }
                "."
            }
            li {
                "✅ Designed with "
                strong { "accessibility" }
                " in mind."
            }
        }
        p {
            "Whether you're building a SaaS dashboard, admin panel, or a hobby side project, "
            strong { "Sidebar" }
            " will save you hours of UI pain."
        }
        h2 { id: "-why-youll-love-it",
            a { href: "#-why-youll-love-it", class: "header", "⚡\u{fe0f} Why You'll Love It" }
        }
        p { "Let's break it down:" }
        table {
            thead {
                th { "Feature" }
                th { "Why it's awesome" }
            }
            tr {
                th {
                    strong { "Modular Components" }
                    ""
                }
                th {
                    "Use just what you need: "
                    code { "Sidebar" }
                    ", "
                    code { "Menu" }
                    ", "
                    code { "MenuItem" }
                    ", "
                    code { "Submenu" }
                    ", "
                    code { "Profile" }
                    ", or "
                    code { "Logo" }
                    ""
                }
            }
            tr {
                th {
                    strong { "Composability" }
                    ""
                }
                th { "Nest menus, add icons, toggle visibility—anything goes" }
            }
            tr {
                th {
                    strong { "Customization" }
                    ""
                }
                th {
                    "Every pixel is yours to style with "
                    code { "props" }
                    ", "
                    code { "class" }
                    ", or "
                    code { "style" }
                    ""
                }
            }
            tr {
                th {
                    strong { "Built for Rust" }
                    ""
                }
                th {
                    "Idiomatic use of "
                    code { "Callback" }
                    ", "
                    code { "UseStateHandle" }
                    ", and "
                    code { "Html" }
                    ""
                }
            }
            tr {
                th {
                    strong { "Responsive + Accessible" }
                    ""
                }
                th { "Auto-collapses on mobile; plays nice with screen readers" }
            }
        }
        p {
            "And the cherry on top? "
            strong { "It's built by Open SASS" }
            ", a community on a mission to make Rust web development suck less."
        }
        h2 { id: "-quick-start-with-yew",
            a { href: "#-quick-start-with-yew", class: "header", "🚀 Quick Start with Yew" }
        }
        p { "Getting up and running is easier than convincing Ferris to wear a bow tie." }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add sidebar </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        p { "Then plug it into your app:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n</pre>\n" }
        p {
            "You now have a sleek, collapsible sidebar with dynamic state and profile logout handling 🎉."
        }
        h2 { id: "-what-you-can-customize-spoiler-everything",
            a {
                href: "#-what-you-can-customize-spoiler-everything",
                class: "header",
                "🔍 What You Can Customize (Spoiler: Everything)"
            }
        }
        p { "Here's a taste of what you can tweak:" }
        h3 { id: "",
            a { href: "#", class: "header", "🧱 " }
            code { "Sidebar" }
            " Props"
        }
        ul {
            li {
                code { "style" }
                ", "
                code { "class" }
                ": Tweak the outer container to match your brand."
            }
            li {
                code { "user_name" }
                ", "
                code { "designation" }
                ", "
                code { "user_img" }
                ": Show off your user's profile."
            }
            li {
                code { "logo_img_url" }
                ", "
                code { "logo_href" }
                ": Your logo, your link."
            }
            li {
                code { "on_logout" }
                ": Handle logout like a pro (or a script kiddie, no judgment)."
            }
        }
        h3 { id: "",
            a { href: "#", class: "header", "📋 " }
            code { "Menu" }
            " & "
            code { "MenuItem" }
        }
        ul {
            li {
                code { "sub_heading" }
                ": Optional section labels."
            }
            li {
                code { "href" }
                ": Navigation links."
            }
            li {
                code { "icon_html" }
                ": Drop in custom SVGs or emojis (we love emojis, btw)."
            }
            li {
                code { "selected" }
                ": Use shared "
                code { "UseStateHandle<String>" }
                " for tracking."
            }
        }
        h3 { id: "",
            a { href: "#", class: "header", "🔽 " }
            code { "Submenu" }
        }
        ul {
            li { "Supports deep nesting." }
            li { "Expand/collapse toggle." }
            li { "Styled with arrows because UI polish." }
        }
        h3 { id: "",
            a { href: "#", class: "header", "🙋 " }
            code { "Profile" }
        }
        ul {
            li { "Automatically hides when sidebar is collapsed." }
            li { "Includes a logout button." }
            li { "Fully styleable." }
        }
        p {
            "All of this controlled with easy props like  "
            code { "style" }
            ",  "
            code { "class" }
            ", and  "
            code { "Callback" }
            "."
        }
        p {
            "Below 768px? "
            strong { "Sidebar auto-collapses" }
            " into a sleek mobile-friendly mode. No JavaScript hacks. Just Rust and vibes."
        }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "💡 Final Thoughts" }
        }
        p {
            strong { "Stop wasting time rebuilding sidebars from scratch" }
            ". Sidebar gives you everything you need, and nothing you don't, to ship polished, responsive, and composable UI components in your Rust WASM apps."
        }
        p { "So go ahead. Drop it in your app. Customize it. Hack it. Make it yours." }
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
pub fn KeepUsingAws() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Hiya 👋!" }
        }
        p {
            em {
                "So, it's Sunday, homeboy, and I usually escape debates and negative thoughts. But today, I was scouring "
                a { href: "https://dev.to", "dev.to" }
                " and stumbled across a post boldly titled "
                a { href: "https://dev.to/code42cate/stop-using-aws-4eg",
                    strong { "\"Stop Using AWS.\"" }
                }
            }
        }
        p {
            "It wasn't the first time I'd seen a call to drop Amazon Web Services for something \"simpler\", but this one hit a nerve, not because it was provocative, but because it misrepresents why AWS exists and what problems it solves. So, I decided to sit back, relax, and explain "
            strong { "why you should absolutely keep using AWS" }
            ", especially if you're serious about software, scalability, and sustainability."
        }
        h2 { id: "-stop-using-aws",
            a { href: "#-stop-using-aws", class: "header", "🛑 Stop Using AWS" }
        }
        p {
            "The post opens with a classic anecdote: a developer builds an MVP with all the AWS bells and whistles, Lambda, API Gateway, Cognito, S3, DynamoDB, and no one uses it. The conclusion? AWS was overkill."
        }
        p {
            "But let's be honest: blaming AWS for a failed product is like blaming your tools when your idea doesn't land. "
            strong {
                "AWS didn't kill your startup. Lack of validation, poor UX, or zero marketing likely did"
            }
            ". Using a battle-tested infrastructure does not guarantee success, but avoiding it can definitely guarantee future pain."
        }
        p {
            "AWS is "
            strong { "not about scale today, it's about resilience tomorrow" }
            ". You can start with one Lambda function and scale to millions of invocations without touching infrastructure again. The same service that powers your early prototype can grow into your production backbone. That's not overengineering, that's smart engineering."
        }
        h2 { id: "-copying-netflix-architecture-is-silly",
            a {
                href: "#-copying-netflix-architecture-is-silly",
                class: "header",
                "🚂 Copying Netflix Architecture Is Silly"
            }
        }
        p {
            "A key argument in the post is that copying architectures from giants like Netflix is absurd. But here's the thing: "
            strong { "copying proven solutions is how engineering works" }
            "."
        }
        p {
            "Bridges don't get new designs for every crossing. Airplanes don't redesign wings every year. Engineers "
            strong { "borrow from what works" }
            ", because "
            strong { "battle-tested architectures reduce risk and improve reliability" }
            "."
        }
        p {
            "The Netflix architecture may look complex, but many of its core principles, CI/CD, serverless scaling, stateless compute, IAM-based security, are just "
            strong { "good defaults" }
            " in AWS. These patterns exist not because they're cool, but because they "
            strong { "work repeatedly under pressure" }
            "."
        }
        p {
            "Reusing success isn't a flaw, it's "
            strong { "strategic acceleration" }
            "."
        }
        h2 { id: "-most-projects-die-from-lack-of-users-not-overengineering",
            a {
                href: "#-most-projects-die-from-lack-of-users-not-overengineering",
                class: "header",
                "🚀 Most Projects Die From Lack of Users, Not Overengineering"
            }
        }
        p {
            "The post says most early-stage projects fail due to lack of users, not bad infrastructure. But that's not always true."
        }
        p {
            "In reality, "
            strong { "most promising projects die from a lack of marketing and financial support" }
            ". As a solo founder currently building an open source project, I can tell you firsthand: the only reason it's still alive is because the community likes it. Not because of my infrastructure. But if tomorrow it catches fire on Hacker News, I know AWS will keep it alive "
            strong { "without migration, without panic, and without sysadmin duty" }
            "."
        }
        p {
            "And let's be real, "
            strong { "bad infrastructure won't get you users, but it will lose them fast" }
            ". Downtime, slow loading, broken auth, insecure APIs, these will kill user trust before your next commit. AWS lets you "
            strong { "ship confidently" }
            ", even with zero staff and zero budget."
        }
        h2 { id: "-just-use-a-vps--docker-compose",
            a { href: "#-just-use-a-vps--docker-compose", class: "header",
                "🔧 Just Use a VPS + Docker Compose"
            }
        }
        p {
            "Ah yes, the $5 VPS dream. The classic hacker manifesto. And while it's a romantic idea, "
            strong { "it quickly collapses under real-world pressure" }
            "."
        }
        ul {
            li {
                p { "Who patches your server?" }
            }
            li {
                p { "Who restores it after a reboot?" }
            }
            li {
                p { "Who handles DDoS protection?" }
            }
            li {
                p { "Who backs up your database?" }
            }
            li {
                p { "Who monitors CPU/memory spikes?" }
            }
            li {
                p { "Who gives you rolling deployments?" }
            }
            li {
                p { "Who gives you audit logs?" }
            }
            li {
                p { "Who keeps your secrets safe?" }
            }
            li {
                p { "Who helps during an outage?" }
            }
        }
        p {
            "When you deploy over SSH and  "
            code { "docker compose up" }
            ", "
            strong { "you are your own SRE" }
            ". That's fine for side projects. But if you're building anything real, anything you want to monetize, secure, or scale, "
            strong { "you need more than a hobby code viber stack" }
            "."
        }
        p {
            "AWS takes these headaches away. You get backup, observability, IAM, autoscaling, and global distribution out of the box. No extra effort. No late-night pager duties."
        }
        h2 { id: "-aws-is-too-complicated",
            a { href: "#-aws-is-too-complicated", class: "header", "🧠 AWS Is Too Complicated" }
        }
        p {
            "Yes, AWS can feel complex. It has over 200 services. It has dozens of ways to deploy. It can be overwhelming. But so is the Linux kernel. So is Kubernetes. So is React, honestly."
        }
        p {
            strong { "Complexity is not a flaw, lack of progressive learning is." }
        }
        p { "AWS is built for gradual discovery:" }
        ul {
            li {
                p {
                    "Start with "
                    a { href: "https://aws.amazon.com/amplify/", "Amplify" }
                    " or "
                    a { href: "https://aws.amazon.com/lightsail/", "Lightsail" }
                    "."
                }
            }
            li {
                p {
                    "Move to "
                    a { href: "https://aws.amazon.com/cdk/", "CDK" }
                    " or "
                    a { href: "https://aws.amazon.com/serverless/sam", "SAM" }
                    "."
                }
            }
            li {
                p { "Add services as you go." }
            }
        }
        p {
            "The best part? You "
            strong { "don't have to throw anything away" }
            " when you grow. That's the opposite of overengineering. That's "
            strong { "technical maturity baked into the platform" }
            "."
        }
        p { "You can stay lean and still have optionality for years." }
        h2 { id: "-when-aws-makes-sense-always",
            a { href: "#-when-aws-makes-sense-always", class: "header",
                "✅ When AWS Makes Sense: Always"
            }
        }
        p {
            "The post says AWS only makes sense in a few niche scenarios: job hunting, compliance, global scale. That's a narrow view."
        }
        p { "Here's when AWS actually makes sense:" }
        ul {
            li {
                p { "You care about security and compliance from day one." }
            }
            li {
                p { "You want an upgrade path without rewrite." }
            }
            li {
                p { "You hate ops and want managed services." }
            }
            li {
                p { "You care about observability and structured logs." }
            }
            li {
                p { "You want global latency under 100ms." }
            }
            li {
                p { "You want fine-grained access control." }
            }
            li {
                p { "You want infra as code and rollback." }
            }
            li {
                p {
                    "You want your infra to "
                    em { "just work" }
                    "."
                }
            }
        }
        p {
            "AWS isn't for everyone. But if you're serious about building something resilient, flexible, and long-lasting, "
            strong { "it's one of the smartest tools you can use." }
        }
        h2 { id: "-why-you-should-use-rust-for-everything-everywhere-always",
            a {
                href: "#-why-you-should-use-rust-for-everything-everywhere-always",
                class: "header",
                "🦀 Why You Should Use Rust for Everything, Everywhere, Always"
            }
        }
        p { "There's one more piece of the stack we haven't talked about: the programming language." }
        p {
            "If you're still choosing Node.js, Python, Ruby, or Go by default, it's time to stop and seriously consider "
            strong { "Rust" }
            ", not just as an option, but as your "
            strong { "primary tool for all stacks" }
            "."
        }
        p {
            "Rust is not a trend. It is "
            strong { "a revolution in software correctness, safety, and performance" }
            ". The following are some of the reasons why using Rust across your entire stack, CLI tools, web backends, infrastructure tooling, and WebAssembly frontends, "
            a { href: "https://github.com/opensass", "Open SASS" }
            " btw "
            strong { "is the most rational long-term bet you can make" }
            ":"
        }
        ul {
            li {
                p {
                    strong { "Memory Safety Without Garbage Collection:" }
                    " Rust guarantees memory safety at compile time without a GC. You write code that's fast and safe, with zero runtime overhead. This means fewer production crashes, fewer bugs, less stress, and happier developers."
                }
            }
            li {
                p {
                    strong { "Blazing Fast Performance:" }
                    " Rust consistently outperforms languages like Python, Ruby, and even Go in compute-intensive tasks. It gives you C-level speed with a sane, modern syntax and tooling."
                }
            }
            li {
                p {
                    strong { "First-Class WebAssembly Support:" }
                    " You can compile Rust to WebAssembly and run it in the browser, or even server-side. This allows "
                    strong { "one language to power both frontend and backend" }
                    ", closing the full-stack loop."
                }
            }
            li {
                p {
                    strong { "Ergonomic Dev Experience:" }
                    " With tools like "
                    code { "cargo" }
                    ", "
                    code { "rust-analyzer" }
                    ", and strong compiler messages, Rust is not just fast, it's a "
                    strong { "joy to work with" }
                    " once you get the hang of it. The ecosystem is mature and growing rapidly."
                }
            }
            li {
                p {
                    strong { "Concurrency Without Fear:" }
                    " Rust's ownership model makes concurrent programming safer by default. You don't fear multithreading. You embrace it."
                }
            }
            li {
                p {
                    strong { "Perfect for Cloud and Infrastructure:" }
                    " Major tools like "
                    a { href: "https://firecracker-microvm.github.io/",
                        code { "firecracker" }
                        " (AWS Lambda microVMs)"
                    }
                    ", "
                    a { href: "https://deno.com/",
                        code { "deno" }
                    }
                    ", and "
                    a { href: "https://vector.dev",
                        code { "vector.dev" }
                    }
                    " are all written in Rust. It's becoming the de facto language for "
                    strong { "next-gen DevOps, cloud infra, and edge computing" }
                    "."
                }
            }
            li {
                p {
                    strong { "Low-Level Power, High-Level Syntax:" }
                    " Need to write performant networking code? Cryptographic primitives? OS-level utilities? Rust does it all, and makes it readable and testable."
                }
            }
            li {
                p {
                    strong { "Future-Proof Your Codebase:" }
                    " Choosing Rust today is betting on stability, safety, and speed for the next 20 years. It's backed by major players, and already replacing legacy C/C++ in critical systems."
                }
            }
        }
        p {
            "Imagine this: a single language that powers your server, builds your CLI, compiles to your frontend, runs your Lambda functions, writes your Terraform replacements, and lets you deploy blazingly fast binaries to any architecture."
        }
        p { "That's not just a dream stack." }
        p {
            "That's "
            strong { "Rust." }
        }
        blockquote {
            p {
                "If you're already using AWS, adopting Rust makes even more sense. You can build ultra-efficient Lambda functions, optimize EC2 compute, write blazing-fast CLI tools, and reduce cold start times dramatically, all while writing safer code."
            }
        }
        p {
            "The world is slowly migrating to Rust, one subsystem at a time. The only question is: "
            strong { "why not be early?" }
        }
        h2 { id: "-final-thoughts-your-stack-shouldnt-be-a-toy",
            a {
                href: "#-final-thoughts-your-stack-shouldnt-be-a-toy",
                class: "header",
                "🔚 Final Thoughts: Your Stack Shouldn't Be a Toy"
            }
        }
        p {
            "So no, you don't "
            em { "need" }
            " AWS. But dismissing it as overkill is shortsighted. A startup doesn't need a Formula 1 car, but it also shouldn't build a go-kart out of duct tape and optimism."
        }
        p {
            "If your product succeeds, you'll wish you had AWS from the start. If it fails, it won't be because you used AWS. It'll be because no one wanted what you built. "
            strong { "Don't blame your infrastructure for your idea's failure." }
        }
        p {
            "Build lean. Build smart. But build with "
            strong { "forward motion" }
            "."
        }
        p {
            strong { "AWS ain't your enemy. It's your parachute." }
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
pub fn SkeletonRsRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Welcome 👋!" }
        }
        p {
            "When you're building blazingly fast WASM apps, there's one thing that silently makes or breaks the user experience: "
            strong { "loading states" }
            ". We've all been there, blank screens, layout jumps, etc. It's not just bad UX. It's "
            em { "lost trust" }
            "."
        }
        p {
            "That's why we're excited today to introduce "
            a { href: "https://crates.io/crates/skeleton-rs",
                strong { "Skeleton RS" }
            }
            " , a "
            strong { "highly-configurable" }
            " skeleton loader built natively for "
            strong { "Rust WASM frameworks" }
            ", starting with full Yew support. Think of it as a smart placeholder for your content , one that speaks fluent WASM."
        }
        p {
            "Let's unpack why "
            strong { "Skeleton RS" }
            " is your new best friend when it comes to loading UIs 🦴."
        }
        p {
            img {
                src: "https://github.com/user-attachments/assets/a130f1fc-891b-4d3b-878c-84310acd5759",
                alt: "sunglasses-glasses",
                title: "",
            }
        }
        h2 { id: "-what-is-skeleton-rs",
            a { href: "#-what-is-skeleton-rs", class: "header", "🦴 What Is Skeleton RS?" }
        }
        p {
            strong { "Skeleton RS" }
            " is a high-performance skeleton loader component designed for frontend Rust frameworks like "
            strong { "Yew" }
            ". Whether you're fetching data, lazy-loading routes, or animating components into view, "
            strong { "Skeleton RS" }
            " gives you graceful, styled placeholders that blend into your design system "
            em { "easily" }
            "."
        }
        p { "No JavaScript hacks. No extra boilerplate. Just clean, declarative Rust 🦀." }
        h3 { id: "built-in-ux-goodness",
            a { href: "#built-in-ux-goodness", class: "header", "Built-in UX Goodness" }
        }
        ul {
            li {
                strong { "Pulse & Wave animations" }
                " for that polished glow."
            }
            li {
                strong { "Auto-sizing" }
                " to match real content dimensions."
            }
            li {
                strong { "Visibility-based animation triggers" }
                " via "
                code { "IntersectionObserver" }
                "."
            }
            li {
                strong { "Fine-grained control" }
                " via props without sacrificing simplicity."
            }
        }
        h2 { id: "-why-youll-love-skeleton-rs",
            a { href: "#-why-youll-love-skeleton-rs", class: "header",
                "🚀 Why You'll Love Skeleton RS"
            }
        }
        blockquote {
            p { "You're not just faking a loading state. You're designing a seamless experience." }
        }
        p {
            "Skeleton RS isn't another \"spinner in disguise\". It's "
            strong { "intentionally minimal" }
            ", yet "
            strong { "insanely flexible" }
            ". Designed to look great out of the box, but gives you the controls when you need them."
        }
        p { "Here's why it stands out:" }
        ul {
            li {
                "🔍 "
                strong { "Context-Aware" }
                ": Animate only when visible , save cycles, look smoother."
            }
            li {
                "🎯 "
                strong { "Responsive" }
                ": Works with any layout, from dashboards to mobile-first views."
            }
            li {
                "🧱 "
                strong { "Composable" }
                ": Use it with or without children, nest it, theme it, customize it."
            }
            li {
                "🪶 "
                strong { "Lightweight" }
                ": No JS, no noise , pure Rust + WebAssembly."
            }
        }
        p { "This isn't a loading hack , it's a design utility." }
        h2 { id: "-quick-yew-setup",
            a { href: "#-quick-yew-setup", class: "header", "⚡ Quick Yew Setup" }
        }
        p {
            "Using Skeleton RS in your "
            strong { "Yew" }
            " project is dead simple:"
        }
        h3 { id: "1-add-it-to-your-dependencies",
            a { href: "#1-add-it-to-your-dependencies", class: "header",
                "1. Add it to your dependencies"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add skeleton</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        h3 { id: "2-import-the-component",
            a { href: "#2-import-the-component", class: "header", "2. Import the component" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">yew::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">skeleton_rs::yew::Skeleton;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">skeleton_rs::Variant;</span></pre>\n" }
        h3 { id: "3-drop-it-into-your-app",
            a { href: "#3-drop-it-into-your-app", class: "header", "3. Drop it into your app" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[function_component(App)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Html {{\n</span><span style=\"color:#f8f8f2;\">    html! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Skeleton\n</span><span style=\"color:#f8f8f2;\">            variant</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Variant::Text}}\n</span><span style=\"color:#f8f8f2;\">            width</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;100%&quot;\n</span><span style=\"color:#f8f8f2;\">            height</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;1.2em&quot;\n</span><span style=\"color:#f8f8f2;\">            animate_on_visible</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "Boom. Beautifully animated, accessible skeleton loaders, ready to roll." }
        h2 { id: "-features",
            a { href: "#-features", class: "header", "🧩 Features" }
        }
        table {
            thead {
                th { "Feature" }
                th { "What It Brings to Your App" }
            }
            tr {
                th { "Variant Support" }
                th {
                    "Choose from "
                    code { "Text" }
                    ", "
                    code { "Circle" }
                    ", "
                    code { "Rect" }
                    ", and more"
                }
            }
            tr {
                th { "Visibility-Based Anim" }
                th { "Trigger animations only when elements are visible" }
            }
            tr {
                th { "Delay + Fallback UX" }
                th {
                    "Avoid flicker with "
                    code { "delay_ms" }
                    ", improve perceived speed"
                }
            }
            tr {
                th { "Responsive Design" }
                th {
                    "Enable "
                    code { "responsive" }
                    " for fluid layouts"
                }
            }
            tr {
                th { "Theming" }
                th { "Seamlessly toggle between light and dark modes" }
            }
            tr {
                th { "Full Custom Styling" }
                th {
                    "Add your own "
                    code { "custom_style" }
                    ", "
                    code { "class" }
                    ", or radius"
                }
            }
        }
        p { "This is UX design, not just developer convenience." }
        h2 { id: "-full-control-with-props",
            a { href: "#-full-control-with-props", class: "header",
                "⚙\u{fe0f} Full Control with Props"
            }
        }
        p { "Skeleton RS gives you all the toggles and knobs you need , without overengineering." }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Skeleton\n</span><span style=\"color:#f8f8f2;\">    variant</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Variant::Rect}}\n</span><span style=\"color:#f8f8f2;\">    width</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;300px&quot;\n</span><span style=\"color:#f8f8f2;\">    height</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;200px&quot;\n</span><span style=\"color:#f8f8f2;\">    border_radius</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;8px&quot;\n</span><span style=\"color:#f8f8f2;\">    animation</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Animation::Wave}}\n</span><span style=\"color:#f8f8f2;\">    show</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    delay_ms</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ff80f4;\">300</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    theme</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Theme::Dark}}\n</span><span style=\"color:#f8f8f2;\">    responsive</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f92672;\">/&gt;</span></pre>\n",
        }
        p {
            "Want more? Use  "
            code { "infer_size" }
            " with children, tweak  "
            code { "margin" }
            ", plug into visibility with  "
            code { "node_ref" }
            ", and even animate on hover, focus, or click."
        }
        p { "Yes, it really is that powerful , and that simple." }
        p {
            "Let's say you're loading user cards or a product grid. Here's how you'd add meaningful skeletons "
            em { "without" }
            " breaking structure:"
        }
        h3 { id: "-text-loading-placeholder",
            a { href: "#-text-loading-placeholder", class: "header",
                "🧾 Text Loading Placeholder"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Skeleton variant</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Variant::Text}} width</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;80%&quot;</span><span style=\"color:#f8f8f2;\"> height</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;1.1em&quot; </span><span style=\"color:#f92672;\">/&gt;</span></pre>\n" }
        h3 { id: "-avatar-circle",
            a { href: "#-avatar-circle", class: "header", "🟦 Avatar Circle" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Skeleton variant</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Variant::Circle}} width</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;48px&quot;</span><span style=\"color:#f8f8f2;\"> height</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;48px&quot; </span><span style=\"color:#f92672;\">/&gt;</span></pre>\n" }
        h3 { id: "-card-block",
            a { href: "#-card-block", class: "header", "📦 Card Block" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Skeleton\n</span><span style=\"color:#f8f8f2;\">    variant</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Variant::Rect}}\n</span><span style=\"color:#f8f8f2;\">    width</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;100%&quot;\n</span><span style=\"color:#f8f8f2;\">    height</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;150px&quot;\n</span><span style=\"color:#f8f8f2;\">    animation</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Animation::Pulse}}\n</span><span style=\"color:#f8f8f2;\">    border_radius</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;12px&quot;\n</span><span style=\"color:#f92672;\">/&gt;</span></pre>\n" }
        h3 { id: "-with-child-content-infer-size",
            a { href: "#-with-child-content-infer-size", class: "header",
                "🔁 With Child Content (Infer Size)"
            }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Skeleton infer_size</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">}}</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">div class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;user-profile&quot;</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ffee99;\">&quot;@rustacean&quot;</span><span style=\"color:#f8f8f2;\">}}</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">div</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">Skeleton</span><span style=\"color:#f92672;\">&gt;</span></pre>\n",
        }
        h3 { id: "-animate-only-when-visible",
            a { href: "#-animate-only-when-visible", class: "header",
                "🕵\u{fe0f} Animate Only When Visible"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Skeleton animate_on_visible</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ff80f4;\">true</span><span style=\"color:#f8f8f2;\">}} </span><span style=\"color:#f92672;\">/&gt;</span></pre>\n" }
        p { "All this, and no layout shifts. Just butter-smooth transitions that feel native." }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "💡 Final Thoughts" }
        }
        p {
            "Modern apps need modern loading states , not just a spinning icon and a prayer. Whether you're building admin dashboards, real-time UIs, or portfolio sites with WASM, "
            strong { "Skeleton RS" }
            " gives you the power to design with empathy."
        }
        p {
            "It's not flashy. It's "
            strong { "functional elegance" }
            " , designed for today's Rust-native frontend."
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
pub fn ThemeRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Hey Rustacean 👋!" }
        }
        p {
            "Let's be real for a sec; Nothing kills a great user experience like an app that ignores your light/dark theme preference. One minute you're vibing in a chill dark mode, next minute: "
            em { "BLINDING WHITE SCREEN" }
        }
        p { "We've all been there." }
        p {
            "That's why we're excited to introduce "
            strong {
                a { href: "https://crates.io/crates/theme",
                    code { "Theme" }
                }
            }
            ", a slick, flexible, no-nonsense theme manager for WASM apps. It handles light, dark, and everything in between (yes, "
            em { "even custom solarized setups, you nerds" }
            " 🌞🌚)."
        }
        p {
            "It's the theming solution your app "
            em { "deserves" }
            ", easy to drop in, works out of the box, and plays nicely with Tailwind, DaisyUI, and your questionable late-night color choices."
        }
        p { "Let's take a look!" }
        p {
            img {
                src: "https://media2.giphy.com/media/v1.Y2lkPTc5MGI3NjExZmdocjNxd3Q1ZnhtenRjczl4ZXdzdmR1bzEyNGh6MXhsb3g2N3R3dCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/zZC2AqB84z7zFnlkbF/giphy.gif",
                alt: "waku waku",
                title: "",
            }
        }
        h2 { id: "-what-is",
            a { href: "#-what-is", class: "header", "🌈 What Is " }
            code { "Theme" }
            "?"
        }
        p {
            code { "Theme" }
            " is a "
            strong { "simple, powerful" }
            " component for managing theming in your WASM app. It does the hard work, like syncing across tabs, respecting system settings, and storing preferences, so you don't have to."
        }
        p {
            "You just wrap your app with a  "
            code { "ThemeProvider" }
            ", and BOOM: instant style wizardry."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">ThemeProvider default_theme</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Theme::System}}</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">App </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">ThemeProvider</span><span style=\"color:#f92672;\">&gt;</span></pre>\n" }
        p {
            "Yes, it even switches automatically between light and dark based on your OS settings. It's basically psychic."
        }
        h2 { id: "-quick-setup",
            a { href: "#-quick-setup", class: "header", "⚡ Quick Setup" }
        }
        h3 { id: "1-add-it-to-your",
            a { href: "#1-add-it-to-your", class: "header", "1. Add it to your " }
            code { "Cargo.toml" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add theme </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        h3 { id: "2-import-the-magic",
            a { href: "#2-import-the-magic", class: "header", "2. Import the magic" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">theme::yew::ThemeProvider;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">theme::{{Theme, StorageType}};</span></pre>\n" }
        h3 { id: "3-wrap-your-app",
            a { href: "#3-wrap-your-app", class: "header", "3. Wrap your app" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">ThemeProvider\n</span><span style=\"color:#f8f8f2;\">    default_theme</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Theme::System}}\n</span><span style=\"color:#f8f8f2;\">    storage_type</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{StorageType::LocalStorage}}\n</span><span style=\"color:#f8f8f2;\">    storage_name</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ffee99;\">&quot;theme&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    custom_themes</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{my_themes}}\n</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">App </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">ThemeProvider</span><span style=\"color:#f92672;\">&gt;</span></pre>\n",
        }
        p { "Congrats, your app is now self-aware and stylish." }
        h2 { id: "-add-your-own-themes",
            a { href: "#-add-your-own-themes", class: "header", "🎨 Add Your Own Themes" }
        }
        p { "Wanna roll your own vibes? You can define custom themes like so:" }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">custom_themes.</span><span style=\"color:#66d9ef;\">insert</span><span style=\"color:#f8f8f2;\">(\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;solarized&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">    Rc::new(CustomTheme {{\n</span><span style=\"color:#f8f8f2;\">        name: </span><span style=\"color:#ffee99;\">&quot;solarized&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">        base: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        tokens: ColorTokens {{\n</span><span style=\"color:#f8f8f2;\">            primary: </span><span style=\"color:#ffee99;\">&quot;#268bd2&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            secondary: </span><span style=\"color:#ffee99;\">&quot;#2aa198&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            background: </span><span style=\"color:#ffee99;\">&quot;#fdf6e3&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            text: </span><span style=\"color:#ffee99;\">&quot;#657b83&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">(),\n</span><span style=\"color:#f8f8f2;\">            error: </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;#dc322f&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">            warning: </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;#cb4b16&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">            success: </span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;#859900&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">        }},\n</span><span style=\"color:#f8f8f2;\">    }}),\n</span><span style=\"color:#f8f8f2;\">);</span></pre>\n",
        }
        p { "This is not a drill, your brand colors can finally shine in full glory." }
        h2 { id: "-theme-context-hook-yes-please",
            a { href: "#-theme-context-hook-yes-please", class: "header",
                "🧠 Theme Context Hook? Yes, please!"
            }
        }
        p {
            "Need to toggle themes from a button or keyboard shortcut? Use the  "
            code { "use_theme()" }
            " hook:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> ctx </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_theme</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> onclick </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> set_theme </span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\"> ctx.set_theme.</span><span style=\"color:#66d9ef;\">clone</span><span style=\"color:#f8f8f2;\">();\n</span><span style=\"color:#f8f8f2;\">    Callback::from(</span><span style=\"color:#f92672;\">move |_|</span><span style=\"color:#f8f8f2;\"> set_theme.</span><span style=\"color:#66d9ef;\">emit</span><span style=\"color:#f8f8f2;\">(Theme::Dark))\n</span><span style=\"color:#f8f8f2;\">}};</span></pre>\n",
        }
        blockquote {
            p {
                "🚨 Pro tip: You can also reset to system default or preview themes temporarily. No reloads. No drama."
            }
        }
        h2 { id: "-tailwind-meet-theme",
            a { href: "#-tailwind-meet-theme", class: "header", "🧰 Tailwind, Meet Theme" }
        }
        p {
            "Working with Tailwind (v3 or below) or using DaisyUI?  "
            code { "Theme" }
            " sets:"
        }
        ul {
            li {
                code { "data-theme" }
            }
            li {
                code { "class" }
            }
            li {
                code { "color-scheme" }
                " (on the root element)"
            }
        }
        p { "Automatically. You don't even need to lift a tail... er, finger 🐶." }
        p {
            img {
                src: "https://media3.giphy.com/media/v1.Y2lkPTc5MGI3NjExc2IxaXFieTIwZXN1cXprODE0bXg2M29sNWxpeW5hMjV0MXFmNXUwaiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/FWAcpJsFT9mvrv0e7a/giphy.gif",
                alt: "heh",
                title: "",
            }
        }
        h2 { id: "-full-control-with-props",
            a { href: "#-full-control-with-props", class: "header", "🧪 Full Control with Props" }
        }
        p {
            "Locking in a theme? Adding runtime validation? Syncing across windows? There's a prop for all of it."
        }
        table {
            thead {
                th { "Prop" }
                th { "What It Does" }
            }
            tr {
                th {
                    code { "default_theme" }
                    ""
                }
                th { "Starts the app in light, dark, or system mode." }
            }
            tr {
                th {
                    code { "storage_type" }
                    ""
                }
                th { "Local or session storage? You pick." }
            }
            tr {
                th {
                    code { "forced_theme" }
                    ""
                }
                th { "Lock to a specific theme (great for demos or trolling coworkers)." }
            }
            tr {
                th {
                    code { "custom_themes" }
                    ""
                }
                th { "Bring your own themes!." }
            }
            tr {
                th {
                    code { "reset_to_system" }
                    ", "
                    code { "apply_preview" }
                    ", "
                    code { "set_custom_theme" }
                    ""
                }
                th { "Hooks for advanced control & UX magic." }
            }
        }
        h2 { id: "-bonus-brainy-features",
            a { href: "#-bonus-brainy-features", class: "header", "🧠 Bonus Brainy Features" }
        }
        ul {
            li {
                "⏱ "
                strong { "Time-based fallback" }
                ": No preference? Default to light during the day, dark at night."
            }
            li {
                "🖇 "
                strong { "Cross-tab syncing" }
                ": Share themes across all open windows."
            }
            li {
                "🪝 "
                strong { "Hooks first" }
                ": Easy to access and control the current theme in any component."
            }
            li {
                "🧪 "
                strong { "Custom validation" }
                ": Every theme goes through a little quality check before being accepted."
            }
        }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "🚀 Final Thoughts" }
        }
        p {
            "Theming shouldn't be a pain. And with  "
            code { "Theme" }
            ", it isn't. From system-based switching to full control, or even total chaos with 10 custom themes, Theme has your back. It's lightweight, declarative, and built for WASM apps."
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
            p { "Till next time 👋" }
        }
    }
}
#[component(no_case_check)]
pub fn SliderRsRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Hey Rustacean 👋!" }
        }
        p { "You know what's surprisingly hard to get right? A good slider component." }
        p {
            "Sure, you can slap together a "
            a { href: "https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input/range",
                code { "<input type=\"range\">" }
            }
            " and call it a day, but once you start wanting "
            strong { "custom styling" }
            ", "
            strong { "accessibility" }
            ", "
            strong { "double-thumb range selectors" }
            ", and "
            em { "gasp" }
            " actual usability... well, things get messy."
        }
        p {
            "That's why we're beyond excited (and mildly sleep-deprived) to announce the release of "
            strong {
                a { href: "https://crates.io/crates/slider-rs", "Slider RS" }
            }
            ": an open-source, fully customizable slider component for your favorite Rust & WASM frameworks like "
            strong { "Yew" }
            ", "
            strong { "Dioxus" }
            ", and "
            strong { "Leptos" }
            "."
        }
        p {
            "It's slick. It's precise. It's accessible. And yes, "
            strong { "Ferris the Crab" }
            " personally approves 🦀."
        }
        p {
            img {
                src: "https://github.com/user-attachments/assets/9a2511a7-f626-4c8c-bc3d-7d01e20b6d45",
                alt: "dababy do be vibin",
                title: "",
            }
        }
        h2 { id: "-why-slider-rs-exists",
            a { href: "#-why-slider-rs-exists", class: "header", "🎉 Why Slider RS Exists" }
        }
        p {
            "Modern web apps need sliders that don't suck. Periodt. From fancy dashboards to media scrubbers and range filters, every app deserves a component that:"
        }
        p { "✅ Looks good (without writing a CSS thesis)." }
        p {
            "Slider RS gives you "
            strong { "fine-grained control" }
            " over styling, behaviors, and accessibility, all while staying efficient, reactive, and smooth. And yes, it even has "
            strong { "tooltips" }
            " for people who like their sliders with a touch of extra flair."
        }
        h2 { id: "-what-can-slider-rs-do",
            a { href: "#-what-can-slider-rs-do", class: "header", "🧰 What Can Slider RS Do?" }
        }
        p { "Let's break it down. Slider RS comes packed with features like:" }
        ul {
            li {
                strong { "🎚\u{fe0f} Single & Range Sliders" }
                ": One thumb? Two thumbs? Your choice."
            }
            li {
                strong { "🎨 Fully Customizable Styling" }
                ": Classes, inline styles, custom thumb content, icon slots, tweak it till it's yours."
            }
            li {
                strong { "🦽 Accessibility First" }
                ": ARIA attributes, keyboard navigation, focus management, it's all baked in."
            }
            li {
                strong { "⚡ Reactive & Efficient" }
                ": Optimized rendering with prop diffing."
            }
            li {
                strong { "🔢 Advanced Goodies" }
                ": Tick marks, tooltips, step indicators, horizontal & vertical orientation, you name it."
            }
            li {
                strong { "🖱\u{fe0f} Smooth UX" }
                ": Drag ranges, fine-tune with keyboard, hover for tooltips, smooth as butter."
            }
        }
        p {
            "Basically, it's the slider component you always "
            em { "wished" }
            " you had, now in Rust-flavored WASM form."
        }
        h2 { id: "-getting-started",
            a { href: "#-getting-started", class: "header", "🚀 Getting Started" }
        }
        p { "Here's how to get started with Yew:" }
        h3 { id: "1-add-slider-rs-to-your-project",
            a { href: "#1-add-slider-rs-to-your-project", class: "header",
                "1. Add Slider RS to Your Project"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add slider</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        h3 { id: "2-import-the-slider-component",
            a { href: "#2-import-the-slider-component", class: "header",
                "2. Import the Slider Component"
            }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">yew::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">slider_rs::yew::Slider;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">slider_rs::Orientation;</span></pre>\n" }
        h3 { id: "3-use-it-in-your-app",
            a { href: "#3-use-it-in-your-app", class: "header", "3. Use It In Your App" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[function_component(App)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Html {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> value </span><span style=\"color:#f92672;\">= </span><span style=\"color:#66d9ef;\">use_state</span><span style=\"color:#f8f8f2;\">(|| </span><span style=\"color:#ff80f4;\">50.0</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    html! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Slider\n</span><span style=\"color:#f8f8f2;\">            min</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ff80f4;\">0.0</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            max</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ff80f4;\">100.0</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            step</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ff80f4;\">1.0</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">            value</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"font-style:italic;color:#66d9ef;\">Some</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">value)}}\n</span><span style=\"color:#f8f8f2;\">            on_change</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Callback::from(|</span><span style=\"font-style:italic;color:#fd971f;\">val</span><span style=\"color:#f8f8f2;\">| log::info</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Slider changed to: {{}}&quot;</span><span style=\"color:#f8f8f2;\">, val))}}\n</span><span style=\"color:#f8f8f2;\">            orientation</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Orientation::Horizontal}}\n</span><span style=\"color:#f8f8f2;\">            show_value</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">            show_steps</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ff80f4;\">true\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "And boom, you've got a fully functional, accessible slider in your Rust app." }
        p {
            "Even "
            strong { "Ferris" }
            " gave it a test drive (and you know how picky he is about UX)."
        }
        h2 { id: "-but-wait-theres-more",
            a { href: "#-but-wait-theres-more", class: "header", "🎨 But Wait, There's More" }
        }
        p {
            "Want to customize everything? Good news, you can. Slider RS comes with a rich set of props to tweak appearance and behavior to your heart's content:"
        }
        table {
            thead {
                th { "✅ What You Can Do" }
                th { "📝 How It Works" }
            }
            tr {
                th { "Single or Range Mode" }
                th {
                    code { "double={{true}}" }
                    " for range selectors"
                }
            }
            tr {
                th { "Control Size & Color" }
                th {
                    "Use "
                    code { "size" }
                    " and "
                    code { "color" }
                    " props"
                }
            }
            tr {
                th { "Custom Icons & Thumbs" }
                th {
                    "Slots like "
                    code { "icon_start" }
                    ", "
                    code { "icon_end" }
                    ", "
                    code { "custom_thumb_html" }
                    ""
                }
            }
            tr {
                th { "Full Styling Control" }
                th { "Classes & inline styles for every part" }
            }
            tr {
                th { "Show Values, Steps & Tooltips" }
                th {
                    code { "show_value" }
                    ", "
                    code { "show_steps" }
                    ", "
                    code { "show_tooltip" }
                    ""
                }
            }
            tr {
                th { "Accessible by Default" }
                th { "ARIA labels, keyboard nav, focus events" }
            }
            tr {
                th { "Reactive Callbacks" }
                th {
                    code { "on_change" }
                    ", "
                    code { "on_change_range" }
                    ", "
                    code { "on_focus" }
                    ", "
                    code { "on_blur" }
                    ""
                }
            }
        }
        p { "It's like a Swiss Army knife, but for sliders. And it won't poke you in the pocket." }
        h2 { id: "-ferris-approved-open-sass-blessed",
            a { href: "#-ferris-approved-open-sass-blessed", class: "header",
                "🦀 Ferris Approved, Open SASS Blessed"
            }
        }
        p {
            "We built Slider RS because "
            strong { "Rust+WASM developers deserve better components" }
            ". It's open-source, MIT-licensed, and ready to be your new favorite UI friend. Whether you're building dashboards, music players, data visualizations, or just a very fancy volume knob, Slider RS has your back."
        }
        blockquote {
            p {
                "Ferris himself said:"
                em { "\"This slider is smoother than a freshly compiled release build.\"" }
            }
        }
        h2 { id: "-join-the-party",
            a { href: "#-join-the-party", class: "header", "💬 Join the Party" }
        }
        p { "Got ideas? Found a bug? Want to show off your slick Slider RS-powered app?" }
        blockquote {
            p {
                "Come hang out with us on "
                a { href: "https://discord.gg/b5JbvHW5nv", "Discord" }
                "."
            }
        }
        p { "We're a friendly bunch of Rustaceans, building cool things for the web." }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "🚀 Final Thoughts" }
        }
        p {
            "If you're building Rust-based web apps, "
            strong { "Slider RS" }
            " is the slider component you didn't know you needed, but now you do. It's precise. It's pretty. It's accessible. And it's yours."
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
            p { "Till next time 👋" }
        }
    }
}
#[component(no_case_check)]
pub fn OpensassKit() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p {
                strong { "Hey Rustacean 👋" }
            }
        }
        p {
            "Let's play a little game: Imagine you're deep in the weeds building a shiny new frontend app. You've got your Rust + WASM stack humming, your  "
            code { "Dioxus" }
            " components are crisp, and your state management is so clean it practically sparkles. But then... the inevitable happens:"
        }
        p {
            "You need a dropdown. Or an accordion. Or a form layout. You google for 45 minutes, copy-paste something from five different GitHub gists, fight with some Tailwind utility class monsters, and then... you cry softly into your keyboard."
        }
        p { "We've all been there. Ferris too 🦀." }
        p {
            "That's exactly why we built "
            strong {
                a { href: "https://github.com/opensass/kit", "Open SASS Kit" }
            }
            ": your new best friend in the chaos of web UI."
        }
        p {
            img {
                src: "https://c.tenor.com/5_xNa2QYmFYAAAAd/tenor.gif",
                alt: "ohmygodtu",
                title: "",
            }
        }
        h2 { id: "-what-is-it",
            a { href: "#-what-is-it", class: "header", "🧰 What Is It?" }
        }
        p {
            strong { "Open SASS Kit" }
            " is not just a toolkit. It's not just a component library. It's not even just a CLI. It's the "
            strong { "centralized home for open, reusable, modular Open SASS style components" }
            ", handcrafted for modern web apps, and especially for our beloved WASM world. Think of it as the "
            strong { "Wikipedia of Open SASS UI components" }
            "."
        }
        p { "We're talking framework-agnostic, no-bloat, plug-and-play components that work with:" }
        ul {
            li { "Tailwind" }
            li { "Bootstrap" }
            li { "Bulma" }
            li { "Foundation" }
            li { "Plain ol' vanilla CSS" }
            li {
                "Or your own secret in-house monstrosity that you definitely promise to refactor \"later\""
            }
        }
        h2 { id: "-but-why",
            a { href: "#-but-why", class: "header", "🎨 But, Why?" }
        }
        p {
            "Let's just say it: CSS frameworks are great... "
            em { "until they're not" }
            ". You're either locked into a giant opinionated stack or stuck rewriting the same \"responsive card\" component 23 times per project. "
            strong { "Open SASS Kit" }
            " lets you break free."
        }
        ul {
            li {
                "🧩 "
                strong { "Modular components" }
                ": Import only what you need. Keep your bundle tight and your sanity intact."
            }
            li {
                "🔁 "
                strong { "Composable & reusable" }
                ": Components that don't fight back. Write once, use forever."
            }
            li {
                "⚙\u{fe0f} "
                strong { "Powerful CLI" }
                ": Want to scaffold an accordion in your Yew project? One command. "
                code { "os add accordion-rs yew" }
                ". Done."
            }
            li {
                "🌐 "
                strong { "WASM-ready" }
                ": Native support for Rust + WASM frontend stacks like Yew, Dioxus, and Leptos."
            }
            li {
                "🧼 "
                strong { "No vendor lock-in" }
                ": Use with "
                em { "any" }
                " CSS framework. Or none. Your stack, your rules."
            }
        }
        p {
            "It's like Tailwind UI and Bootstrap had a baby, and then raised it with Rustacean values: freedom, composability, and just a touch of healthy minimalism."
        }
        h2 { id: "-how-it-works",
            a { href: "#-how-it-works", class: "header", "⚡ How It Works?" }
        }
        p { "You start by installing the CLI:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo install opensass</span></pre>\n" }
        p { "Then you grab a component:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">os add accordion</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs yew</span></pre>\n" }
        p {
            "That's it. No boilerplate. No duct-tape. The CLI does the file generation, wiring, and styling integration for you. And yes, it "
            em { "actually" }
            " works."
        }
        p {
            "Under the hood, Open SASS Kit is powered by carefully crafted, lightweight Open SASS style components that are fully compatible with the latest Rust-based frontend frameworks. And because it's fully open, you can tweak, extend, or fork to your heart's content. Add some dark mode. Replace our icons with your favorite set of crabs. Make it your own 🦀."
        }
        h2 { id: "-a-living-library-of-components",
            a { href: "#-a-living-library-of-components", class: "header",
                "📚 A Living Library of Components"
            }
        }
        p {
            "At its core, Open SASS Kit is more than a toolkit, it's a "
            strong { "growing centralized catalog" }
            ". Every component we add is carefully documented, versioned, and designed to be useful across stacks."
        }
        p {
            "It's not just for Rust folks (though let's be honest, we "
            em { "do" }
            " have the best taste). It's for anyone tired of bloated UI kits, rigid frameworks, or reinventing the same dang dropdowns."
        }
        p {
            "Our goal is to make Open SASS the "
            em { "universal toolkit" }
            " for modern web projects, the one place where you can explore everything available, pick what you need, and get on with shipping actual features."
        }
        blockquote {
            p {
                "TL;DR: No more scrolling through 20 Medium articles to style a button. Just run  "
                code { "os add accordion-rs yew" }
                ", and keep moving."
            }
        }
        h2 { id: "-built-for-wasm-frameworks",
            a { href: "#-built-for-wasm-frameworks", class: "header",
                "🧪 Built for WASM Frameworks"
            }
        }
        p {
            "We know where the future's headed, and it's full of  "
            code { "wasm32-unknown-unknown" }
            " targets. That's why every Open SASS component plays beautifully with:"
        }
        ul {
            li {
                a { href: "https://yew.rs",
                    code { "Yew" }
                }
            }
            li {
                a { href: "https://dioxuslabs.com",
                    code { "Dioxus" }
                }
            }
            li {
                a { href: "https://leptos.dev",
                    code { "Leptos" }
                }
            }
        }
        p { "Each component comes with integration examples and sensible defaults." }
        h2 { id: "-join-the-party",
            a { href: "#-join-the-party", class: "header", "🤝 Join the Party" }
        }
        p {
            "We're building Open SASS Kit "
            em { "with" }
            " the community, not just "
            em { "for" }
            " it. That means:"
        }
        ul {
            li { "Found a bug? Tell us." }
            li { "Got a better version of a component? PR it." }
            li {
                "Built something awesome with Open SASS? Show it off in our "
                a { href: "https://discord.gg/b5JbvHW5nv", "Discord" }
                "."
            }
        }
        p { "Contributing is simple:" }
        ol {
            li { "Fork the repo" }
            li {
                "Create a branch ("
                code { "feature/my-awesome-component" }
                ")"
            }
            li { "Submit a pull request" }
        }
        p {
            "Bonus points if you name your branch something like  "
            code { "fix/ferris-got-stuck-in-the-carousel" }
            "."
        }
        h2 { id: "-this-kit-is-just-getting-started",
            a { href: "#-this-kit-is-just-getting-started", class: "header",
                "📈 This Kit Is Just Getting Started"
            }
        }
        p {
            "Open SASS Kit is still young, but it's growing fast, like Ferris after a long nap and four cups of espresso. New components are being added regularly. Docs are being improved. The CLI is evolving. In other words: "
            strong { "now's the best time to get involved" }
            ". Whether you're a grizzled Rustacean, a newbie web dev, or just someone who wants to stop styling buttons manually, Open SASS Kit has something for you."
        }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "🎤 Final Thoughts" }
        }
        p {
            "Look, building modern UIs shouldn't feel like assembling IKEA furniture in the dark, with no instructions, and a screaming crab on your shoulder. With Open SASS Kit, it doesn't have to. We're giving you the tools to "
            strong { "move fast, build confidently" }
            ", and never write the same accordion component twice. Ever again. So stop scrolling through old Gists. Stop fighting your CSS framework. Start building like you mean it, with a toolkit that's actually on your side."
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
pub fn HackingDioxus() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h2 { id: "tldr",
            a { href: "#tldr", class: "header", "TL;DR" }
        }
        p {
            "If you're hiring vibe coders, think again before it's too late. This post isn't about dissing Dioxus; It's about raising awareness around the fragility of modern software engineering, especially when inexperienced developers use powerful tools. Always hire engineers with the proper experience, particularly when working in critical areas like full-stack development. Vibe coding isn't inherently bad, but when handed to the wrong people, it becomes a dangerous practice."
        }
        h2 { id: "introduction",
            a { href: "#introduction", class: "header", "Introduction" }
        }
        blockquote {
            p { "Hello friends 👋!" }
        }
        p {
            "Today I want to share with you a deep frustration, a boiling discontent built up over two sleepless weekends trying to report and explain multiple security vulnerabilities to a well-known and publicly available open-source project in the Rust ecosystem: "
            a { href: "https://github.com/dioxusLabs/dioxus", "Dioxus" }
            ". For those unfamiliar, Dioxus is a modern full-stack UI framework for Rust, and its promises are big. It aims to deliver the kind of seamless development experience we've grown to expect from JavaScript ecosystems, but powered by Rust's memory safety and type system. While its goals are admirable and its developers clearly passionate, what I've encountered has exposed a darker side of the modern \"move fast and break things\" mindset that has mixed into even our most robust and theoretically secure ecosystems."
        }
        p {
            "This blog post is a reflection on not just Dioxus itself, but the overall degeneration of software engineering practices caused by what I refer to as \"vibe coding\". It's not an attack on Dioxus as a tool or its creators personally. Instead, this is a petition to the industry to take security seriously. Dioxus merely serves as a case study, a concrete example of the dangers of prioritizing developer experience and hype over secure engineering principles. And before anyone gets defensive, understand: I've been in this field for years, if you measure experience by sleepless hours, millions of lines of code written, and real systems that were actually used, and I've dealt with real security threats. This isn't my first time doing this."
        }
        p {
            "The title is provocative, yes, but it is also literal. Vibe coding "
            em { "is" }
            " destroying the very foundation of what software engineering used to mean: precision, responsibility, reliability, security. I completely understand that product managers and investors breathe down your neck for fast delivery. I've shipped code under pressure. I've dealt with the \"why isn't it done yet?\" beam. But none of that justifies ignoring the fundamentals. We're not building TikTok filters, we're constructing software that handles real data, affects real users, and, in some cases, controls real-world infrastructure. A culture of shipping at all costs with minimal inspection is not only irresponsible, it's dangerous."
        }
        p {
            "This post will take you through multiple security flaws discovered in Dioxus over the past two weeks. It will explain how these issues could have been avoided with proper engineering practices and why vibe coding, where the developer is more concerned about the aesthetic or the \"feel\" of writing code rather than understanding the underlying system, leads to systemic failures. And yes, while some Dioxus maintainers may claim these aren't \"security issues\", the principles of secure software design demand we treat them as such."
        }
        h2 { id: "what-is-a-software-engineer--developer",
            a {
                href: "#what-is-a-software-engineer--developer",
                class: "header",
                "What is a Software Engineer / Developer?"
            }
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/hi9vga85glmaycw2hzf3.webp",
                alt: "Vibe Coding",
                title: "",
            }
        }
        p {
            "Before we get into the complicated stuff, let's clarify some terminology that the industry often uses interchangeably but should not. A software engineer is fundamentally different from a developer. While both write code, the responsibilities and mindset between the two are worlds apart. A software engineer is someone who is capable of designing systems from first principles. They understand memory models, design constraints, threat surfaces, performance bottlenecks, and scalability trade-offs. They don't just write code, they architect robust, maintainable, and secure systems that solve real-world problems in an optimized manner."
        }
        p {
            "On the other hand, a software developer is typically focused on building within the confines of those systems. They may be responsible for adding features, fixing bugs, tweaking performance, or even just connecting APIs together in a frontend interface. There's nothing inherently wrong with being a developer. But conflating the two leads to mismatched expectations, especially in hiring processes or in open-source communities where contributions affect thousands of users."
        }
        p {
            "Why is this distinction important in a post about Dioxus and vibe coding? Because Dioxus, like many modern frameworks, abstracts away a lot of complexity, which makes it very inviting to developers who are new or who may not understand the consequences of their actions. This abstraction isn't evil by itself. In fact, well-designed abstraction is one of the cornerstones of good engineering. But when those abstractions are handed to people who don't fully understand the implications of the code they're writing, particularly in a systems language like Rust, we get insecure software."
        }
        p {
            "Software engineers are responsible not just for the code they write but for the consequences of that code in production. When a type system is bypassed, when untrusted input is handled without validation, when unsafe code is introduced without proper justification and bounds checking, that is not a momentary lapse in judgment. That is a systemic failure in design responsibility. That's what vibe coding enables."
        }
        p {
            "Ultimately, the security and integrity of an application lies in the hands of the engineers behind it. If something goes wrong, pointing fingers at the \"framework\" or the \"community\" is insufficient. The engineer must accept responsibility. And so, if a tool encourages insecure patterns or fails to enforce good practices, it's not just a developer problem, it's a failure of engineering culture, documentation, and design. This is where Dioxus, as a case study, becomes illustrative."
        }
        h2 { id: "what-is-vibe-coding",
            a { href: "#what-is-vibe-coding", class: "header", "What is Vibe Coding?" }
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/2v8i3jpte4e79yo1phll.webp",
                alt: "Vibe Coding",
                title: "",
            }
        }
        p {
            "Let's now take a moment to define this central concept: "
            em { "vibe coding" }
            ". Vibe coding is the act of programming based on \"feel\" rather than understanding. It's when a developer wires together code snippets they've seen in documentation, StackOverflow, or AI-generated outputs without understanding the system's internals. It's the act of treating your framework or language as a magical black box, feeding it inputs, hoping for the right outputs, and assuming that \"if it compiles, it's probably fine\"."
        }
        p {
            "Vibe coding is particularly dangerous in systems programming. Languages like Rust were designed to enforce safety through strict compile-time guarantees. But even Rust cannot protect your system if you deliberately (or ignorantly) bypass those guarantees. Unsafe blocks, dynamic plugin systems, stringly-typed APIs, all of these are opportunities for subtle, dangerous bugs when wired without understanding."
        }
        p {
            "When someone vibe codes a UI in React, the worst-case scenario might be a broken button or a misaligned div. When someone vibe codes a server function in Dioxus using unsafe function pointers, CSRF-vulnerable APIs, or SSRF-prone static site generation, the consequences scale up quickly. And yet, because modern frameworks accommodate developer ergonomics, they don't always poke the developer toward secure or well-reasoned patterns. They prioritize speed, simplicity, and joy. But joy in programming should not come at the expense of discipline."
        }
        p {
            "It raises an important question: when software fails, who is responsible? The developer who vibe coded it? The framework that encouraged it? The runtime that failed to detect it? In truth, all share a slice of the blame. But the lion's share falls upon the person who wrote the code. Because the machine does not write your software. It merely executes it. No matter how advanced LLMs become or how \"magical\" our dev tools feel, the human remains in the loop."
        }
        p {
            "This is not just a philosophical discussion. It has real-world implications, as the next section will demonstrate. Because over the course of two weekends, I went deep into Dioxus's internals, not as a casual user, but as a software engineer intent on breaking it open and seeing what lies beneath. What I found was troubling. What I reported was met with dismissal. And what I learned is that we are sleepwalking into a new era of insecure software built by developers who don't know what they're building."
        }
        h2 { id: "hacking-dioxus",
            a { href: "#hacking-dioxus", class: "header", "Hacking Dioxus" }
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/ki379976xe3gn59e2g6w.png",
                alt: "Hacking Dioxus",
                title: "",
            }
        }
        p {
            "I've been using Dioxus full-time for over 7 months. Not passively. Not as a toy. I've used it in production systems, built fullstack apps, integrated with WebAssembly, deployed to cloud environments, and experimented with server-side rendering and static generation. My feedback comes not from a place of friction, but from a place of personal familiarity. And what I found during a deep audit of its codebase should concern anyone using or contributing to the project."
        }
        p {
            "Let's begin with the first vulnerability: "
            strong {
                "Open Redirect in the "
                code { "Link" }
                " component"
            }
            "."
        }
        h3 { id: "open-redirect-vulnerability",
            a { href: "#open-redirect-vulnerability", class: "header",
                "Open Redirect Vulnerability"
            }
        }
        p {
            "This issue was reported "
            a { href: "https://github.com/DioxusLabs/dioxus/issues/4134", "here" }
            ". It may sound minor, but its implications are not. The Dioxus "
            code { "Link" }
            " component currently accepts arbitrary strings as its "
            code { "to" }
            " parameter. That means a developer can write something like:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">Link {{ to: </span><span style=\"color:#ffee99;\">&quot;https://some-malicious-website.com&quot; </span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p {
            "The maintainers argue that this is fine, it's up to the developer to decide whether they're linking internally or externally. But here's the catch: the  "
            code { "Link" }
            " component is part of the Dioxus router, which is meant for "
            em { "internal" }
            " navigation. It exists to manage in-app routing, maintain client-side history, and provide a seamless UX without full page reloads. Allowing arbitrary external URLs through this component breaks the contract of trust between the developer and the router system. It blurs the line between internal routing and external redirection in a way that opens up phishing, and redirect attack vectors."
        }
        p {
            "Compare this to "
            a { href: "https://yew.rs", "Yew" }
            ", a Rust framework that does this correctly. "
            a { href: "https://docs.rs/yew-router/latest/yew_router/components/struct.Link.html",
                "Yew's  "
                code { "Link" }
            }
            " component only accepts values from a "
            a { href: "https://docs.rs/yew-router/latest/yew_router/trait.Routable.html",
                code { "Routable" }
            }
            " enum. This enforces compile-time guarantees that a route is valid and internal. You cannot accidentally pass in a user-controlled string and redirect them to a malicious site. That's type safety. That's Rust's promise. And that's what Dioxus breaks."
        }
        p {
            "So when I filed this as a vulnerability, I wasn't just nitpicking. I was advocating for Dioxus to honor Rust's philosophy: "
            em { "preventing errors at compile time wherever possible" }
            ". The maintainers disagreed. One even said, dismissively, \"This is not a security vulnerability and it's ridiculous to claim it is.\" That kind of tone is not only unprofessional, it's dangerous. It discourages responsible disclosure. It chills open source security culture. And it misses the point entirely."
        }
        p {
            "I proposed a simple fix: if a user wants to link to an external site, use the standard HTML  "
            code { "a" }
            " tag:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">a {{ href: </span><span style=\"color:#ffee99;\">&quot;https://google.com&quot; </span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        p { "And if the user wants in-app routing, use:" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">Link {{ to: Route::Home }}</span></pre>\n" }
        p {
            "This ensures type safety, separation of concerns, and better DX "
            em { "without" }
            " compromising security. A win-win. But the maintainers were not interested. Perhaps because this fix wasn't \"vibe\" enough."
        }
        h2 { id: "csrf-in-server-functions",
            a { href: "#csrf-in-server-functions", class: "header", "CSRF in Server Functions" }
        }
        p {
            "Let's start by analyzing how CSRF works at a technical level and why it becomes a critical threat when not properly mitigated in server-side APIs, especially those serving frontend clients over the web. Cross-Site Request Forgery is a well-known and dangerous form of attack where an attacker tricks a user into performing unwanted actions on a web application in which they're currently authenticated. These actions can range from changing account settings, submitting data, or even making financial transactions. The core vulnerability comes from the fact that browsers automatically attach cookies to requests, including authentication tokens and session cookies, making the victim's authenticated state exploitable if not validated properly. In this case, Dioxus's server functions, designed to provide developer-friendly async APIs with automatic serialization and routing, lack built-in CSRF protection, which puts developers in a precarious position where secure-by-default isn't guaranteed."
        }
        p {
            a { href: "https://github.com/DioxusLabs/dioxus/issues/4190#issuecomment-2907717675",
                "When I proposed the  "
                code { "#[with_csrf]" }
                " macro"
            }
            ", it wasn't just about syntactic sugar or convenience, it was about aligning the Dioxus stack with the principle of secure defaults, something that the Rust language and its ecosystem pride themselves on. Rust's core philosophy is centered around safety and correctness. The moment we shift that burden of correctness entirely onto the developer, we break that implicit promise. Let's take frameworks like "
            code { "Next.js" }
            " as an example. Even though it's written in JavaScript, an infamously permissive and unsafe language, "
            code { "Next.js" }
            " still goes out of its way to encourage "
            a { href: "https://nextjs.org/blog/security-nextjs-server-components-actions#csrf",
                "CSRF"
            }
            " tokens and offers "
            a { href: "https://nextjs.org/docs/app/building-your-application/routing/middleware",
                "middleware"
            }
            " and utilities that reduce the chance of such oversights. The argument that Dioxus shouldn't be responsible for CSRF because it doesn't manage sessions or authentication directly is, in my opinion, insufficient. Providing security primitives like CSRF tokens should be the "
            em { "minimum" }
            " any modern fullstack web framework offers, this isn't asking for a feature; this is about foundational safety in a connected world where web exploitation is the norm, not the exception."
        }
        p {
            "Furthermore, from a developer experience standpoint, introducing a  "
            code { "#[with_csrf]" }
            " procedural macro adds virtually no additional cognitive overhead, but dramatically improves the likelihood that server functions are protected against CSRF attacks. The proposed implementation could easily check for a valid  "
            code { "X-CSRF-Token" }
            " in the request headers and validate it against a signed session token. This is similar to what popular frameworks like "
            a { href: "https://docs.djangoproject.com/en/5.2/ref/csrf/", "Django" }
            " and "
            a { href: "https://laravel.com/docs/12.x/csrf", "Laravel" }
            " have done for years. It's a battle-tested pattern. What I'm asking for isn't new or revolutionary, it's standard, mature, and secure. What makes Rust unique is that it allows us to do all this at compile time with strong type checking, minimizing room for human error."
        }
        p {
            "Now, when I raised this as an "
            a { href: "https://github.com/DioxusLabs/dioxus/issues/4132", "issue" }
            ", I was met with the counterpoint that enforcing CSRF tokens universally would restrict valid use cases, like calling server functions from unauthenticated APIs or external clients. And yes, that's technically true, but that's precisely why I suggested making it an "
            em { "opt-in" }
            " system. This is not about enforcing behavior globally, but about giving developers the "
            em { "option" }
            " to choose the secure path with minimal friction. If you're building an app where CSRF is not a concern, you simply don't add the macro. If you're building an app that deals with forms, user inputs, account management, or anything remotely sensitive, you slap "
            code { "#[with_csrf]" }
            " on top of the server function and move on with confidence. How is that a bad tradeoff?"
        }
        p {
            "It also seems like the Dioxus team is deeply committed to keeping the framework light and developer-friendly, which I greatly respect. In fact, I admire the effort that has gone into the router, server functions, and CLI tooling. However, friendliness should not come at the cost of safety. Even if we assume that most developers are smart and security-conscious, we cannot assume that "
            em { "every developer is" }
            ". Security must be idiot-proof, not because we think developers are idiots, but because the stakes are just that high. A forgotten CSRF token is not an academic problem; it's a potential PR disaster or a data leak. And in today's hyper-connected world, one data breach is all it takes to lose user trust, investor confidence, and sometimes even legal ground under "
            a { href: "https://en.wikipedia.org/wiki/General_Data_Protection_Regulation",
                "GDPR"
            }
            " or "
            a { href: "https://en.wikipedia.org/wiki/California_Consumer_Privacy_Act",
                "CCPA"
            }
            "."
        }
        p {
            "If the Dioxus maintainers are concerned about maintaining clarity in the macro system, there are several routes to improve this. The macro could emit compile-time warnings if used incorrectly. We could even generate server logs that explain what the CSRF system is doing during runtime in debug mode. Better yet, we could allow users to configure a CSRF strategy at a higher level, something like  "
            code { "ServerConfig::enable_csrf_protection(true)" }
            " which makes every server function CSRF-aware by default, unless explicitly opted out. There are a dozen sane, ergonomic design paths we can follow to achieve this goal, and none of them degrade the developer experience."
        }
        p {
            "I want to emphasize again that this isn't just about Dioxus. The Rust ecosystem needs to have a broader conversation about security ergonomics. We've gotten excellent at zero-cost abstractions, safe concurrency, and data race prevention, but web security, in the fullstack space, still feels like a second-class citizen. Libraries like  "
            code { "axum" }
            ",  "
            code { "actix" }
            ", and  "
            code { "warp" }
            " have CSRF middleware maintained by third parties. This fragmentation is bad for the ecosystem and makes it harder for new developers to follow best practices. A modern fullstack web framework like Dioxus is "
            em { "the perfect place" }
            " to show leadership and establish secure defaults out-of-the-box. It sets a strong precedent for other libraries to follow."
        }
        p {
            "I've seen too many developers brush off CSRF as \"not their problem\", only to end up doing damage control after their app is compromised. A decade ago, this was forgivable. Today, it's not. So let me say it loud and clear for anyone reading this: if your app handles user input, authentication, or sensitive data, and you're not protecting your server functions from CSRF, then you are shipping a vulnerable app. It doesn't matter whether you used Rust or Brainfuck or Haskell. Security is language-agnostic. And it's high time we stop excusing unsafe defaults just because they make onboarding easier."
        }
        h2 { id: "dos-caused-by-arbitrary-function-pointer-transmute",
            a {
                href: "#dos-caused-by-arbitrary-function-pointer-transmute",
                class: "header",
                "DoS Caused by Arbitrary Function Pointer Transmute"
            }
        }
        p {
            "This next "
            a { href: "https://github.com/DioxusLabs/dioxus/issues/4189", "issue" }
            " is probably one of the most outstandingly and technically shocking vulnerabilities I came across while exploring the internals of the Dioxus fullstack server runtime. Specifically, it involves the use of unsafe Rust code to transmute raw function pointers during hot reload operations in development mode. The relevant code resides in the hot reload path and involves this line:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> new_root </span><span style=\"color:#f92672;\">= unsafe </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">    std::mem::transmute::&lt;</span><span style=\"background-color:#f92672;color:#f8f8f0;\">*</span><span style=\"color:#f8f8f2;\">const (), </span><span style=\"font-style:italic;color:#66d9ef;\">fn</span><span style=\"color:#f8f8f2;\">() -&gt; Element</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">(new_root_addr)\n</span><span style=\"color:#f8f8f2;\">}};</span></pre>\n" }
        p {
            "At a glance, the casual observer might not see why this is an issue, especially since this block of code is clearly marked as part of the development hot reload infrastructure. But anyone with experience in systems programming, or anyone who has ever been burned by undefined behavior in C or C++, should immediately see red flags here. Transmuting a raw pointer to a function pointer is a classically dangerous move unless you are absolutely sure that the pointer is valid and correctly aligned. In this case, the assumption is that the hot reload system can blindly accept any memory address provided via the loader and call it like a proper function. This is just asking for a segmentation fault, or worse."
        }
        p {
            "And indeed, that's exactly what happens. If an invalid or malformed function pointer is introduced, intentionally or otherwise, the result is a runtime crash. You can trivially reproduce this by sending a bogus pointer to the reload system, causing the entire Dioxus fullstack server to segfault. This is not merely a bug. It is a weaponizable DoS vector, especially if an attacker can influence the plugin loading system. Even though the maintainers are correct that this only affects development builds, we cannot ignore the implications: developers running hot reload tools are now at risk of crashing their dev servers with malformed inputs. More importantly, this kind of unchecked unsafe logic sends the wrong message about Rust safety practices in high-level frameworks."
        }
        p {
            "Let me be clear: the use of  "
            code { "unsafe" }
            " in Rust is sometimes unavoidable. Rust gives you the  "
            code { "unsafe" }
            " keyword not to avoid safety, but to isolate and explicitly contain unsound operations that would otherwise be impossible in safe code. But when using  "
            code { "unsafe" }
            ", you are entering a contract with the compiler and the runtime: "
            em { "you must manually uphold all the guarantees that Rust normally provides for you" }
            ". That includes pointer validity, lifetime correctness, memory alignment, and type soundness. Transmuting raw pointers violates all of these unless handled with surgical precision. The current Dioxus code does none of that validation. It simply transmutes and executes."
        }
        p {
            "So what's the fix? Actually, there are several. The most straightforward one is to validate the pointer before transmutation. If the pointer is null, or misaligned, the system should refuse to execute and return an explicit panic with a diagnostic message. This would prevent the segmentation fault and provide developers with a clear understanding of what went wrong. Alternatively, the system could require signed metadata for the loaded functions, ensuring that only trusted code paths are executed. This would effectively sandbox the reload system and dramatically reduce the attack surface. We could also adopt the pattern seen in hot-reloading plugins from "
            a { href: "https://docs.unity3d.com/2022.3/Documentation/Manual/PluginInspector.html",
                "Unity"
            }
            ", where plugin registration is explicit and compile-time-checked. This would mean that hot reload targets would need to register their function exports explicitly, allowing the compiler to generate safe entry points. This way, any change to the application logic would require an explicit recompile of the plugin manifest, and all the address resolution could be validated against a known set of safe signatures."
        }
        p {
            "In response to this report, the Dioxus maintainers asserted that since "
            a { href: "https://github.com/DioxusLabs/dioxus/issues/4189#issuecomment-2907099136",
                "this only affects development"
            }
            ", the priority of fixing it is low. While I understand their reasoning, I respectfully disagree. Development-time tooling is often the first contact point for new users and teams evaluating a technology. If a developer encounters a crash during hot reload because of malformed function pointers, it doesn't matter that it's a dev-only issue, their perception of Dioxus as a reliable toolchain is already dulled. Worse, in large enterprise environments where development is done at scale across multiple teams and sandboxes, a rogue reload bug can bring down a staging environment or corrupt a shared cache. It's not just about safety; it's about trust."
        }
        p {
            "What makes this particularly ironic is that Rust's strongest selling point, its promise of memory safety and crash resistance, is completely nullified by poor usage of  "
            code { "unsafe" }
            ". We don't get to brag about \"fearless concurrency\" and \"safe systems programming\" if we turn around and write code that would make a C compiler blush. Every  "
            code { "unsafe" }
            " block is a loaded gun. And it's the framework's job, not the developer's, to make sure that trigger isn't pulled without proper safeguards in place."
        }
        p {
            "In short, this vulnerability is demonstrative of a deeper issue: vibe coding is creeping into the core libraries of a safety-first ecosystem. When we cut corners for the sake of speed, especially in  "
            code { "unsafe" }
            " contexts, we end up with fragile foundations that betray everything Rust stands for. That's not just poor engineering. That's a breach of trust with the entire community."
        }
        h3 { id: "ssrf-in-cli-ssg-loop",
            a { href: "#ssrf-in-cli-ssg-loop", class: "header", "SSRF in CLI SSG Loop" }
        }
        p {
            "When you have a build tool like "
            a { href: "https://crates.io/crates/dioxus-cli", "the Dioxus CLI" }
            ", which includes server-side rendering (SSR) and static site generation (SSG) capabilities, you're already operating in a semi-privileged environment. Even if \"dx serve\" is labeled as a development tool, the implications of introducing unvalidated input routes into a loop that performs HTTP requests should raise significant red flags, especially considering the increasing number of teams using these tools in CI/CD pipelines or for local staging environments. As reported in "
            a { href: "https://github.com/DioxusLabs/dioxus/issues/4137", "this issue" }
            ", by blindly fetching each route using a formatted HTTP GET request, without sanitizing or validating these paths, you introduce a clear vector for Server-Side Request Forgery (SSRF). SSRF, as widely recognized in the "
            a { href: "https://owasp.org/Top10/", "OWASP Top 10 list" }
            " of security vulnerabilities, allows attackers to trick servers into making requests to unintended destinations, like internal systems, cloud metadata services (e.g., "
            a { href: "https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instancedata-data-retrieval.html",
                "AWS'  "
                code { "169.254.169.254" }
            }
            "), or even other vulnerable services that trust internal traffic."
        }
        p {
            "To quote OWASP directly: "
            em {
                "\"SSRF flaws occur whenever a web application is fetching a remote resource without validating the user-supplied URL.\""
            }
            " ("
            a { href: "https://owasp.org/Top10/A10_2021-Server-Side_Request_Forgery_(SSRF)/#description",
                "OWASP SSRF"
            }
            "). Dioxus's current CLI behavior precisely fits that description. And while the maintainers may have dismissed this by saying "
            a { href: "https://github.com/DioxusLabs/dioxus/issues/4137#issuecomment-2888929498",
                "\"it's just a dev tool\""
            }
            ", I challenge the logic behind accepting insecure defaults, even in dev-mode utilities. A dev tool used today might be embedded in tomorrow's automation pipeline. We all know that lines between \"development\" and \"production\" blur quickly in modern workflows."
        }
        p {
            "Moreover, an attacker who can poison the route list, via environmental variables, or other misconfigurations, can easily exploit the SSRF vector. Even if it's \"just development\", the potential risk of leaking internal system data, or pivoting into more sensitive areas should warrant at least a minimum validation layer. We're not talking about rewriting the tool, just sanitize route strings before they're passed into the request loop. Ensure routes are relative paths, prevent  "
            code { "http://" }
            " or  "
            code { "https://" }
            " prefixed routes, and error out when absolute domains are encountered. These are trivial patches that could stop an entire class of attack vectors dead in their tracks."
        }
        p {
            "It's ironic that Dioxus, a Rust framework meant to leverage Rust's strengths like type safety and security, is allowing basic security hygiene issues to go unchecked. Developers pivote to Rust because they want control and reliability. If your toolchain weakens the very foundation the language was built on, then you're not shipping Rust. You're shipping Vibe Rust™, a watered-down promise of what secure systems programming could be."
        }
        h2 { id: "reflection",
            a { href: "#reflection", class: "header", "Reflection" }
        }
        p {
            "Let's be crystal clear here: this isn't about ego, disputation, or any personal campaign against a project. This is a much-needed wake-up call that echoes across the entire open-source Rust ecosystem. When I reported these issues, I did so not to nitpick or throw shade, but because I genuinely care about the quality of the tools we are all using. I've written countless lines of Rust over the past years, read through core Rust source, and followed the design philosophies of Rust since its early stable days. I've watched as people celebrated Rust's memory safety, but ignored the growing cracks in userland safety, especially in web frameworks and glue code."
        }
        p {
            "Security is not an afterthought, nor should it ever be one. When you say, \"developers are responsible for using the tools safely\", you're technically correct, but morally negligent. Let me ask this: Would you trust a car company that says, \"Oh, the brakes work if the driver applies them correctly, but sometimes they don't respond unless you write your own brake controller\"? That's exactly what some libraries and frameworks are saying to developers right now. Shifting security responsibilities to the end user, especially when such issues can be proactively mitigated in the framework layer, is irresponsible engineering."
        }
        p {
            "We're not talking about exotic attack vectors. We're talking about "
            strong { "Open Redirects" }
            ", "
            strong { "CSRF" }
            ", "
            strong { "SSRF" }
            ", and "
            strong { "segfaults" }
            ". These are day-one bugs. These are the types of vulnerabilities that show up in bug bounty reports and make headlines when they go unpatched in production apps. It is deeply concerning when the response to these reports is outright dismissal, immediate issue closure, or marking them as \"spam\" instead of engaging in a technical discussion. That's not just poor communication, it's bad governance."
        }
        p {
            "I fully understand open source is a labor of love. Maintainers often work on nights and weekends, unpaid and underappreciated. But part of being an open source master is knowing how to respond to critical input. Even if you don't agree, the least you can do is not treat the messenger like the enemy. Otherwise, the entire idea of open collaboration collapses into a gatekept monoculture where critique is seen as attack and security is seen as \"someone else's job.\""
        }
        h2 { id: "final-thoughts",
            a { href: "#final-thoughts", class: "header", "Final Thoughts" }
        }
        p {
            "Let's revisit the big picture. Vibe coding isn't a term coined to insult people. It's a shorthand for describing a pattern where tools are used intuitively without foundational understanding. It's what happens when libraries become too easy to use at the expense of correctness. Developers start composing applications by copying examples, skimming docs, and leaning on autocomplete. And while that's okay for prototyping, it's absolutely unacceptable in production."
        }
        p {
            "The Rust community has often prided itself on going slow, doing things right, and shipping quality code. But if we don't hold our frameworks to the same standard, we're just gaslighting ourselves. Security doesn't just live in the compiler. It lives in APIs. It lives in interfaces. It lives in assumptions baked into design. That's why we need ergonomic "
            em { "and" }
            " secure frameworks. That's why I wrote this post."
        }
        p {
            "If you build systems that encourage cargo-culting, where people use features without understanding their impact, you are on the hook for making those features safer. If you dismiss every security report with \"you're using it wrong\", then you've built an unsafe abstraction. Periodt."
        }
        p {
            "Dioxus has tremendous potential. The community is growing. The DX is excellent. But none of that matters if the foundation is riddled with avoidable pitfalls. My hope is that this post helps others understand that being fast and fun doesn't excuse being careless. Rust deserves better. Developers deserve better. Users deserve better."
        }
        p {
            "If you're a Dioxus user, take a second look at how you're handling routing, server functions, and any unsafe FFI magic. If you're using Dioxus in a production deployment, even \"just testing\", audit your setup. Is there an open redirect? Are you issuing GETs to dynamic routes? Do your server functions validate inputs and defend against CSRF?"
        }
        p { "If you're a maintainer, of Dioxus or any Rust project, please consider the following:" }
        ul {
            li { "Don't treat security reports like spam. Take 5-10 minutes to validate the concern." }
            li { "Use type systems to prevent misuse, especially in routing and I/O APIs." }
            li { "Provide optional guardrails (macros, features, traits) for common security needs." }
            li { "Document security assumptions clearly. Make it hard to use things incorrectly." }
            li { "When in doubt, err on the side of safety. Rust taught us that. Live it." }
        }
        p {
            "Despite the critiques, I want to end this on a positive note. Dioxus is a brilliant project. The work being done to unify desktop, mobile, and web apps in a type-safe Rust UI layer is nothing short of visionary. The fact that I can run the same app on WASM, and native platforms with minimal code changes is incredible. Server functions are powerful. The SSG capabilities are ahead of their time. And OpenSASS being built entirely on top of it proves that Dioxus isn't just hype, it's usable and scalable."
        }
        p {
            "But a great tool is not immune from critique. It's through feedback, yes, sometimes harsh, that tools evolve from \"good enough\" to \"industry standard\". I sincerely hope this blog is read in that spirit."
        }
        p {
            "So, thank you to the Dioxus team. Thank you for the hours of work. Thank you for the documentation, the examples, the bug fixes, all of which I am actively contributing to and improving. Just please, take security as seriously as you take developer experience. Because in the end, DX without security is just fast failure."
        }
        blockquote {
            p {
                "At Open SASS, we're working tirelessly on making Rust web development extremely easy for everyone."
            }
        }
        blockquote {
            p {
                "If you made it this far, it would be nice if you could "
                a { href: "https://discord.gg/b5JbvHW5nv", "join us on Discord" }
                "."
            }
        }
        blockquote {
            p { "Till next time, keep building, but build responsibly 👋!" }
        }
    }
}
#[component(no_case_check)]
pub fn BrowserRsRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Welcome 👋!" }
        }
        p {
            "If you've ever tried building a polished, production-ready WASM UI in "
            strong { "Rust" }
            ", you know the drill: getting your app to "
            em { "feel" }
            " native often involves duct tape, a prayer to the compiler gods, and a suspicious amount of copy-pasted CSS. And when it comes to wrapping content in a clean, browser-like frame? Forget it, until now."
        }
        p {
            "That's why we're excited to announce "
            a { href: "https://crates.io/crates/browser-rs",
                strong { "Browser RS" }
            }
            ", the "
            strong { "drop-in browser frame component" }
            " built natively for Rust. It's sleek, it's accessible, it's got style for days, and yes, it even plays nice with Tailwind. Whether you're embedding an iframe, a widget, or just want your app to look like it belongs inside a miniature Chrome tab, "
            strong { "Browser RS" }
            " delivers "
            strong { "maximum polish with minimum fuss" }
            "."
        }
        p {
            "Let's dive in and see why Browser RS is the browser "
            em { "inside" }
            " your browser you didn't know you needed."
        }
        p {
            img {
                src: "https://media.tenor.com/dW17uHQlonMAAAAi/dance-anime.gif",
                alt: "bro do be vibin",
                title: "",
            }
        }
        h2 { id: "-what-is-browser-rs",
            a { href: "#-what-is-browser-rs", class: "header", "🌐 What Is Browser RS?" }
        }
        p {
            strong { "Browser RS" }
            " is a "
            strong { "fully customizable browser-frame component" }
            " built specifically for "
            strong { "WASM Frameworks" }
            ", Rust's powerhouse frontend frameworks. It lets you "
            strong { "wrap any HTML content" }
            " inside a mock browser UI, complete with a header bar, address field, window controls, and optional custom buttons, all while giving you fine-grained control over styling, behavior, and accessibility. Basically, it's like giving your content a tuxedo and sending it to prom."
        }
        h2 { id: "-why-youll-love-browser-rs",
            a { href: "#-why-youll-love-browser-rs", class: "header",
                "🚀 Why You'll Love Browser RS"
            }
        }
        p {
            "Not all browser frames are created equal. Some are rigid. Others are pure CSS gimmicks. But Browser RS? It's got "
            strong {
                "substance "
                em { "and" }
                " style"
            }
            ". Here's what sets it apart:"
        }
        ul {
            li {
                strong { "Plug-and-Play Simplicity" }
                ": Add it to your project in seconds. No dark magic or manual wiring required."
            }
            li {
                strong { "Full Control" }
                ": Customize "
                em { "everything" }
                ", from the close button to the ARIA labels."
            }
            li {
                strong { "Event-Driven" }
                ": Hook into user actions like close, maximize, and minimize. It's like window management, but without the OS."
            }
            li {
                strong { "Accessible by Default" }
                ": Screen reader support, keyboard nav, ARIA labels, your app's users (and auditors) will thank you."
            }
            li {
                strong { "Dark Mode Friendly" }
                ": Browser RS fits in with your Tailwind-based design system, including that moody, stylish dark mode."
            }
        }
        p { "Even Ferris the crab gave it claws up 🦀👍." }
        h2 { id: "-getting-started-with-yew",
            a { href: "#-getting-started-with-yew", class: "header",
                "🔥 Getting Started with Yew"
            }
        }
        p {
            "If you're already in the Yew ecosystem, integrating "
            strong { "Browser RS" }
            " is smoother than a fresh "
            code { "cargo build" }
            "."
        }
        h3 { id: "add-the-crate",
            a { href: "#add-the-crate", class: "header", "Add the Crate" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add browser</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        h3 { id: "import-the-component",
            a { href: "#import-the-component", class: "header", "Import the Component" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">yew::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">browser_rs::yew::BrowserFrame;</span></pre>\n" }
        h3 { id: "wrap-your-content",
            a { href: "#wrap-your-content", class: "header", "Wrap Your Content" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[function_component(App)]\n</span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Html {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> on_close </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Callback::from(|_| log::info</span><span style=\"color:#f92672;\">!</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Closed like it&#39;s 2003 and your mom needs the phone line&quot;</span><span style=\"color:#f8f8f2;\">));\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    html! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">BrowserFrame url</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ffee99;\">&quot;https://opensass.org&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()}} on_close</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{on_close}}</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot;Here&#39;s some magical Yew-powered content.&quot; </span><span style=\"color:#f8f8f2;\">}}</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">BrowserFrame</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        h2 { id: "-features-that-matter",
            a { href: "#-features-that-matter", class: "header", "🧩 Features That Matter" }
        }
        table {
            thead {
                th { "Feature" }
                th { "Why It's Awesome" }
            }
            tr {
                th {
                    code { "url" }
                    " & "
                    code { "placeholder" }
                    ""
                }
                th {
                    "Looks real, works seamlessly, no "
                    code { "<iframe>" }
                    " nightmares needed"
                }
            }
            tr {
                th {
                    code { "on_close" }
                    " / "
                    code { "on_minimize" }
                    " / "
                    code { "on_maximize" }
                    ""
                }
                th { "Build dynamic, responsive UIs like a pro" }
            }
            tr {
                th {
                    code { "custom_buttons" }
                    ""
                }
                th { "Add fun buttons, even a \"Launch Ferris\" rocket icon 🚀🦀" }
            }
            tr {
                th {
                    code { "class" }
                    ", "
                    code { "style" }
                    ""
                }
                th { "Tailor every pixel to your liking" }
            }
            tr {
                th { "ARIA & keyboard support" }
                th { "Accessible by default, no extra effort required" }
            }
            tr {
                th { "Size & Variant" }
                th { "Choose from small to full screen, minimal to rich" }
            }
        }
        p {
            "And let's not forget: it "
            strong { "works everywhere" }
            ". Editors, sandboxes, dashboards, iframes, whatever you're building, it makes it look like it belongs on a browser within a browser, which is "
            em { "almost" }
            " as cool as Inception."
        }
        h2 { id: "-styled-to-match",
            a { href: "#-styled-to-match", class: "header", "🎨 Styled to Match" }
        }
        p {
            "Want that clean, modern look without writing a single line of CSS? Just pass in your Tailwind classes:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">BrowserFrame\n</span><span style=\"color:#f8f8f2;\">    url</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ffee99;\">&quot;https://opensass.org&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()}}\n</span><span style=\"color:#f8f8f2;\">    class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ffee99;\">&quot;rounded-xl shadow-xl&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    input_class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ffee99;\">&quot;bg-gray-200 text-gray-900&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    container_class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ffee99;\">&quot;flex-1 mx-4&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt;</span><span style=\"color:#f8f8f2;\">{{ </span><span style=\"color:#ffee99;\">&quot;Styled browser frame!&quot; </span><span style=\"color:#f8f8f2;\">}}</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">p</span><span style=\"color:#f92672;\">&gt;\n</span><span style=\"color:#f92672;\">&lt;/</span><span style=\"color:#f8f8f2;\">BrowserFrame</span><span style=\"color:#f92672;\">&gt;</span></pre>\n",
        }
        p { "Your designers will think you've leveled up overnight." }
        h2 { id: "-full-control-via-props",
            a { href: "#-full-control-via-props", class: "header",
                "🎛\u{fe0f} Full Control via Props"
            }
        }
        p {
            "Browser RS comes with "
            strong { "a buffet of props" }
            ", you can style, control, and wire up everything:"
        }
        ul {
            li {
                code { "url" }
                ", "
                code { "placeholder" }
                ", "
                code { "read_only" }
            }
            li {
                code { "show_controls" }
                ", "
                code { "show_address_bar" }
            }
            li {
                code { "on_url_change" }
                ", "
                code { "on_close" }
                ", "
                code { "on_minimize" }
                ", "
                code { "on_maximize" }
            }
            li {
                code { "custom_buttons" }
            }
            li {
                "Styling: "
                code { "class" }
                ", "
                code { "style" }
                ", "
                code { "container_class" }
                ", "
                code { "input_class" }
            }
            li {
                "Accessibility: "
                code { "aria_label" }
                ", "
                code { "aria_describedby" }
            }
            li {
                "Visual size & variant controls: "
                code { "size" }
                ", "
                code { "variant" }
            }
            li {
                "Full control of internal buttons: "
                code { "close_*" }
                ", "
                code { "maximize_*" }
                ", "
                code { "share_*" }
                ", etc."
            }
        }
        p {
            "If you want "
            em { "boring" }
            ", you've come to the wrong crate."
        }
        h2 { id: "-real-world-use-cases",
            a { href: "#-real-world-use-cases", class: "header",
                "🛠\u{fe0f} Real-World Use Cases"
            }
        }
        p {
            "Here's where "
            strong { "Browser RS" }
            " really shines:"
        }
        ul {
            li {
                strong { "Interactive Demos" }
                ": Want to showcase your app with embedded code or tools? Wrap it in a "
                code { "BrowserFrame" }
                "."
            }
            li {
                strong { "Developer Tools" }
                ": Give your in-app dev tools a visual shell."
            }
            li {
                strong { "Mini-browser Widgets" }
                ": Use it for previews, sandboxed environments, or even static content."
            }
            li {
                strong { "Figma-like Interfaces" }
                ": Great for app builders and drag-n-drop UIs."
            }
        }
        p {
            "And yes, you can make it look like Safari, Firefox, or even Netscape Navigator (if you're "
            em { "that" }
            " nostalgic)."
        }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "💬 Final Thoughts" }
        }
        p {
            "If you're serious about building delightful web apps with "
            strong { "Yew" }
            " and "
            strong { "WASM" }
            ", Browser RS is a no-brainer."
        }
        ul {
            li {
                p { "✅ It makes your app look polished without extra design work." }
            }
            li {
                p { "✅ It provides real interactivity through events and dynamic props." }
            }
            li {
                p { "✅ It keeps your app accessible, responsive, and customizable." }
            }
            li {
                p {
                    "✅ It's built by Rust devs, "
                    em { "for" }
                    " Rust devs."
                }
            }
        }
        p {
            "Ferris didn't have a browser frame when Rust was born, but if he did, "
            strong { "he'd use this one" }
            ", and probably paint it red 🦀."
        }
        blockquote {
            p {
                "At Open SASS, we're working tirelessly on making Rust web development extremely easy for everyone."
            }
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
pub fn HeroRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Welcome 👋!" }
        }
        p {
            "If you've ever tried building a stunning landing page in Rust with a WASM frontend framework like "
            strong { "Yew" }
            ", "
            strong { "Leptos" }
            ", or "
            strong { "Dioxus" }
            ", you've probably run into the same issue: your \"hero section\" looks more like a background NPC."
        }
        p {
            "You want that perfect first impression, bold headline, snappy call-to-action, beautiful image layout, but you don't want to fight the layout engine or hard-code 47 utility classes every time."
        }
        p {
            "That's why we're excited to announce "
            a { href: "https://crates.io/crates/hero",
                strong { "Hero" }
            }
            ", the "
            em { "zero-to-hero" }
            " crate for building powerful, customizable hero sections in Rust's favorite WASM UI frameworks."
        }
        p {
            "And yes, Ferris the crab gave it a big claw-five 🦀✋ because this crate is "
            em { "shell-shockingly" }
            " good."
        }
        p {
            img {
                src: "https://c.tenor.com/wqe1eEp7Bb0AAAAC/tenor.gif",
                alt: "high five",
                title: "",
            }
        }
        h2 { id: "what-is-hero",
            a { href: "#what-is-hero", class: "header", "What Is Hero?" }
        }
        p {
            strong { "Hero" }
            " is a battle-tested collection of "
            strong { "drop-in hero section components" }
            " designed to work seamlessly with Yew, Leptos, and Dioxus. From launching a landing page to showcasing a new product or giving your Rust app the glow-up it missed in the '90s, "
            code { "hero" }
            " is here to back you up. It's lightweight, unopinionated, accessible, responsive by default, and "
            em { "ridiculously easy to customize" }
            ". Think of it as your frontend starter spell, just a few lines of code, and boom: instant wow factor."
        }
        h2 { id: "why-hero",
            a { href: "#why-hero", class: "header", "Why Hero?" }
        }
        p { "Let's break it down like Ferris breakdancing at a RustConf afterparty:" }
        ul {
            li {
                p {
                    strong { "Responsive by Default" }
                    " Mobile-first layouts without even trying. It just works."
                }
            }
            li {
                p {
                    strong { "Totally Customizable" }
                    ": Want Tailwind? Inline styles? Class overrides? No problem, you do you."
                }
            }
            li {
                p {
                    strong { "Pluggable Components" }
                    ": Drop in custom headings, buttons, images, even other components like it's Lego."
                }
            }
            li {
                p {
                    strong { "Theme-agnostic" }
                    ": No hardcoded styles here. Light theme? Dark theme? Custom gradients and funky fonts? Hero doesn't judge."
                }
            }
            li {
                p {
                    strong { "Framework Agnostic" }
                    ": Works out-of-the-box with "
                    strong { "Yew" }
                    ", "
                    strong { "Leptos" }
                    ", and "
                    strong { "Dioxus" }
                    ", plus anything else brave enough to touch the WASM void."
                }
            }
        }
        h2 { id: "getting-started-with-yew",
            a { href: "#getting-started-with-yew", class: "header", "Getting Started with Yew" }
        }
        h3 { id: "add-the-crate",
            a { href: "#add-the-crate", class: "header", "Add the Crate" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add hero </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        h3 { id: "import-the-hero-you-need",
            a { href: "#import-the-hero-you-need", class: "header", "Import the Hero You Need" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">hero::yew::hero1::Hero </span><span style=\"color:#f92672;\">as</span><span style=\"color:#f8f8f2;\"> Hero1;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">hero::yew::hero2::Hero </span><span style=\"color:#f92672;\">as</span><span style=\"color:#f8f8f2;\"> Hero2;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">hero::yew::hero4::Hero </span><span style=\"color:#f92672;\">as</span><span style=\"color:#f8f8f2;\"> Hero3;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">hero::yew::hero3::Hero </span><span style=\"color:#f92672;\">as</span><span style=\"color:#f8f8f2;\"> Hero4;</span></pre>\n" }
        h3 { id: "basic-example",
            a { href: "#basic-example", class: "header", "Basic Example" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">yew::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">hero::yew::hero1::Hero;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[function_component(App)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Html {{\n</span><span style=\"color:#f8f8f2;\">    html! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Hero\n</span><span style=\"color:#f8f8f2;\">            heading</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Build Fast in Rust&quot;\n</span><span style=\"color:#f8f8f2;\">            description</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Drop-in hero sections for Yew, Leptos, and Dioxus.&quot;\n</span><span style=\"color:#f8f8f2;\">            title_style</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;font-size: 3rem; font-weight: bold; color: #4F46E5;&quot;\n</span><span style=\"color:#f8f8f2;\">            description_style</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;font-size: 1.25rem; color: #6B7280;&quot;\n</span><span style=\"color:#f8f8f2;\">            cta_style</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;padding: 0.75rem 1.5rem; background-color: #4F46E5; color: white; border-radius: 0.5rem;&quot;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Congrats, you just went from \"blank screen\" to \"production-ready landing page\" in under 10 lines of Rust."
        }
        h2 { id: "tailwind-friendly",
            a { href: "#tailwind-friendly", class: "header", "Tailwind Friendly" }
        }
        p {
            "Prefer class-based styling?  "
            code { "hero" }
            " doesn't mind."
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Hero\n</span><span style=\"color:#f8f8f2;\">    heading</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Launch Ultra-Fast Apps&quot;\n</span><span style=\"color:#f8f8f2;\">    description</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;Style your hero section with Tailwind, inline styles, or any CSS framework.&quot;\n</span><span style=\"color:#f8f8f2;\">    container_class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;max-w-6xl mx-auto px-4 py-24&quot;\n</span><span style=\"color:#f8f8f2;\">    title_class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;text-5xl font-extrabold text-center text-white&quot;\n</span><span style=\"color:#f8f8f2;\">    description_class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;mt-4 text-xl text-center text-gray-300&quot;\n</span><span style=\"color:#f8f8f2;\">    cta_class</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;mt-6 bg-white text-black px-6 py-3 rounded-full shadow-lg hover:bg-gray-100&quot;\n</span><span style=\"color:#f92672;\">/&gt;</span></pre>\n",
        }
        p {
            "Mix and match styles, props, and components like you're playing modded Minecraft, except instead of creepers, you're battling CSS bugs."
        }
        h2 { id: "full-control-via-props",
            a { href: "#full-control-via-props", class: "header", "Full Control via Props" }
        }
        p {
            "Each  "
            code { "Hero" }
            " component exposes a smorgasbord of props for maximum flexibility:"
        }
        ul {
            li {
                strong { "Content Props" }
                ": "
                code { "heading" }
                ", "
                code { "description" }
                ", "
                code { "cta" }
                ", "
                code { "tabs" }
                ", etc."
            }
            li {
                strong { "Styling Props" }
                ": "
                code { "title_style" }
                ", "
                code { "cta_style" }
                ", "
                code { "container_style" }
            }
            li {
                strong { "Class Props" }
                ": "
                code { "container_class" }
                ", "
                code { "title_class" }
                ", etc."
            }
            li {
                strong { "Accessibility Props" }
                ": "
                code { "aria_label" }
                ", "
                code { "heading_tag" }
                ", "
                code { "role" }
            }
        }
        p {
            "Want a  "
            code { "div" }
            "? You got it. Want it to be an  "
            code { "h2" }
            " with ARIA support? Easy."
        }
        h2 { id: "feature-recap",
            a { href: "#feature-recap", class: "header", "Feature Recap" }
        }
        table {
            thead {
                th { "Feature" }
                th { "Why You'll Love It" }
            }
            tr {
                th { "4 layout variants" }
                th { "Variety without complexity" }
            }
            tr {
                th { "Full theming support" }
                th { "Dark mode, light mode, even cyberpunk" }
            }
            tr {
                th { "Works in Yew/Leptos/Dioxus" }
                th { "Build wherever Ferris dares to roam" }
            }
            tr {
                th { "Easy integration" }
                th { "No config files, no head-scratching" }
            }
            tr {
                th { "Custom components support" }
                th { "Drop in tabs, buttons, or an actual Ferris GIF" }
            }
        }
        p { "Speaking of which, Ferris says \"this hero section slaps\"." }
        h2 { id: "final-thoughts",
            a { href: "#final-thoughts", class: "header", "Final Thoughts" }
        }
        p {
            "Hero isn't just another frontend wrapper, it's "
            strong { "your first impression, delivered in idiomatic Rust" }
            "."
        }
        ul {
            li {
                p { "✅ Works with your stack." }
            }
            li {
                p { "✅ Respects your styling choices." }
            }
            li {
                p { "✅ Looks great on every device." }
            }
            li {
                p { "✅ Comes with four layout variants and infinite potential." }
            }
        }
        p {
            "Ferris didn't "
            em { "choose" }
            " to be the face of Rust. But if he had a website, we're pretty sure he'd use "
            code { "hero" }
            " to put his best claw forward."
        }
        p {
            strong { "Go be the hero your frontend deserves." }
        }
        blockquote {
            p {
                "At Open SASS, we're working tirelessly on making Rust web development extremely easy for everyone."
            }
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
pub fn PrideRsRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Welcome 👋!" }
        }
        p {
            "So I came across the "
            a { href: "https://dev.to/challenges/frontend-2025-06-04",
                strong { "Frontend Challenge: June Celebrations (CSS Art)" }
            }
            " on "
            a { href: "https://dev.to", "dev.to" }
            ", and I thought: "
            em {
                "\"Hey, what if I build a handy dandy crate for our gay friends that they can slap onto their rusty websites?\""
            }
            " This way, I learn a bit more about CSS, make something useful, and give Ferris the crab 🦀 a chance to finally come out of the shell."
        }
        p {
            "And so, "
            strong { "Pride RS" }
            " was born."
        }
        h2 { id: "-what-is-pride-rs",
            a { href: "#-what-is-pride-rs", class: "header",
                "🏳\u{fe0f}\u{200d}🌈 What Is Pride RS?"
            }
        }
        p {
            strong { "Pride RS" }
            " is a drop-in, customizable rusty component for rendering LGBTQ+ pride flags directly in your Rust frontend. From celebrating Pride Month, to adding some rainbow love to your app, or just want a vertical NonBinary flag for reasons between you and your browser, "
            strong { "Pride RS" }
            " makes it extremely easy."
        }
        p {
            "Flags are rendered using "
            a { href: "https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction",
                code { "flex-direction" }
            }
            ", composed of CSS-powered color stripes that flex in all the right directions: "
            strong { "horizontally" }
            " or "
            strong { "vertically" }
            ". No SVGs. No dependencies on assets. Just pure HTML and CSS, generated at runtime."
        }
        p { "And yes, Ferris is now canonically queer. You're welcome 🦀🏳\u{fe0f}\u{200d}🌈." }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/c5d9zu5u9igr5aal5vy2.webp",
                alt: "queer ferris",
                title: "",
            }
        }
        h2 { id: "-under-the-hood",
            a { href: "#-under-the-hood", class: "header", "⚙\u{fe0f} Under the Hood" }
        }
        p {
            "Each flag is defined in a build-time-generated hashmap using the magic of "
            a { href: "https://docs.rs/phf",
                code { "phf" }
            }
            ", which means "
            strong { "zero runtime overhead" }
            ", and fast constant lookups. The following is how one of the configurations looks:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">static </span><span style=\"color:#ff80f4;\">FLAG_CONFIGURATIONS</span><span style=\"color:#f8f8f2;\">: phf::Map&lt;</span><span style=\"color:#f92672;\">&amp;&#39;static </span><span style=\"font-style:italic;color:#66d9ef;\">str</span><span style=\"color:#f8f8f2;\">, FlagConfig&gt; </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">phf_map! {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;Rainbow&quot; </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> FlagConfig {{\n</span><span style=\"color:#f8f8f2;\">        colors: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;#e40303&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;#ff8c00&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;#ffed00&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;#008018&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;#0066ff&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;#732982&quot;</span><span style=\"color:#f8f8f2;\">],\n</span><span style=\"color:#f8f8f2;\">        direction: Direction::Horizontal,\n</span><span style=\"color:#f8f8f2;\">        name: </span><span style=\"color:#ffee99;\">&quot;Pride Rainbow Flag&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        description: </span><span style=\"color:#ffee99;\">&quot;The original rainbow pride flag representing LGBTQ+ community&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#ffee99;\">&quot;Transgender&quot; </span><span style=\"color:#f92672;\">=&gt;</span><span style=\"color:#f8f8f2;\"> FlagConfig {{\n</span><span style=\"color:#f8f8f2;\">        colors: </span><span style=\"color:#f92672;\">&amp;</span><span style=\"color:#f8f8f2;\">[</span><span style=\"color:#ffee99;\">&quot;#5bcffa&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;#f5abb9&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;#ffffff&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;#f5abb9&quot;</span><span style=\"color:#f8f8f2;\">, </span><span style=\"color:#ffee99;\">&quot;#5bcffa&quot;</span><span style=\"color:#f8f8f2;\">],\n</span><span style=\"color:#f8f8f2;\">        direction: Direction::Horizontal,\n</span><span style=\"color:#f8f8f2;\">        name: </span><span style=\"color:#ffee99;\">&quot;Transgender Flag&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">        description: </span><span style=\"color:#ffee99;\">&quot;Flag representing transgender community with light blue, pink, and white stripes&quot;</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">    }},\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#8c8c8c;\">// ... more flags!\n</span><span style=\"color:#f8f8f2;\">}};</span></pre>\n",
        }
        p {
            "Each flag can be rendered either "
            strong { "horizontally" }
            " ("
            code { "flex-direction: column" }
            ") or "
            strong { "vertically" }
            " ("
            code { "flex-direction: row" }
            "). You control the direction, size, and style directly via props."
        }
        p {
            "We've got a solid collection of flags (sourced from "
            a { href: "https://en.wikipedia.org/wiki/Pride_flag", "Wikipedia" }
            " with love and hex codes):"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(\n</span><span style=\"color:#f8f8f2;\">    EnumString, EnumIter, AsRefStr, Display, Debug, Eq, PartialEq, Hash, Clone, Copy, Default,\n</span><span style=\"color:#f8f8f2;\">)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Type {{\n</span><span style=\"color:#f8f8f2;\">    #[default]\n</span><span style=\"color:#f8f8f2;\">    Rainbow,\n</span><span style=\"color:#f8f8f2;\">    Transgender,\n</span><span style=\"color:#f8f8f2;\">    Bisexual,\n</span><span style=\"color:#f8f8f2;\">    Lesbian,\n</span><span style=\"color:#f8f8f2;\">    Pansexual,\n</span><span style=\"color:#f8f8f2;\">    Asexual,\n</span><span style=\"color:#f8f8f2;\">    NonBinary,\n</span><span style=\"color:#f8f8f2;\">    Aromantic,\n</span><span style=\"color:#f8f8f2;\">    Demisexual,\n</span><span style=\"color:#f8f8f2;\">    Genderfluid,\n</span><span style=\"color:#f8f8f2;\">    Agender,\n</span><span style=\"color:#f8f8f2;\">    Polysexual,\n</span><span style=\"color:#f8f8f2;\">    Omnisexual,\n</span><span style=\"color:#f8f8f2;\">    Demiromantic,\n</span><span style=\"color:#f8f8f2;\">    Graysexual,\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "Got a flag not listed? PRs welcome. Future updates will support more "
            strong { "complex shapes" }
            " too: think chevrons, triangles. Geometry is gay now."
        }
        p { "You can tweak everything:" }
        h3 { id: "flag-sizes",
            a { href: "#flag-sizes", class: "header", "Flag Sizes" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(Debug, Clone, PartialEq, Default)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Size {{\n</span><span style=\"color:#f8f8f2;\">    Small,\n</span><span style=\"color:#f8f8f2;\">    #[default]\n</span><span style=\"color:#f8f8f2;\">    Medium,\n</span><span style=\"color:#f8f8f2;\">    Large,\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        h3 { id: "direction-layout",
            a { href: "#direction-layout", class: "header", "Direction (Layout)" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">#[derive(EnumString, EnumIter, AsRefStr, Display, Debug, Clone, Copy, Default, PartialEq)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">enum </span><span style=\"color:#f8f8f2;\">Direction {{\n</span><span style=\"color:#f8f8f2;\">    #[default]\n</span><span style=\"color:#f8f8f2;\">    Horizontal,\n</span><span style=\"color:#f8f8f2;\">    Vertical,\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n" }
        h3 { id: "style-everything",
            a { href: "#style-everything", class: "header", "Style Everything" }
        }
        p {
            "The  "
            code { "Flag" }
            " and  "
            code { "FlagSection" }
            " components give you full control over:"
        }
        ul {
            li { "Stripe styling" }
            li { "Container layout" }
            li { "ARIA accessibility" }
            li { "Tooltip behavior" }
            li { "Custom CSS classes" }
        }
        p {
            "Everything is built with "
            strong { "accessibility-first" }
            " principles: screen-reader labels, keyboard operability, and polite announcements for empty sections."
        }
        h2 { id: "-getting-started-with-yew",
            a { href: "#-getting-started-with-yew", class: "header",
                "🧰 Getting Started with Yew"
            }
        }
        p {
            "If you're already cozy with "
            strong { "Yew" }
            ", using Pride RS is pretty straightforward, i mean gayforward ;-):"
        }
        h3 { id: "step-1-add-the-crate",
            a { href: "#step-1-add-the-crate", class: "header", "Step 1: Add the Crate" }
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">cargo add pride</span><span style=\"color:#f92672;\">-</span><span style=\"color:#f8f8f2;\">rs </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">features</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">yew</span></pre>\n" }
        h3 { id: "step-2-use-the-component",
            a { href: "#step-2-use-the-component", class: "header", "Step 2: Use the Component" }
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">yew::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">pride_rs::yew::{{FlagSection, Flag}};\n</span><span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">pride_rs::{{Size, Type}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[function_component(App)]\n</span><span style=\"color:#f92672;\">pub </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">app</span><span style=\"color:#f8f8f2;\">() -&gt; Html {{\n</span><span style=\"color:#f8f8f2;\">    html! {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;&gt;\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Flag r</span><span style=\"color:#f92672;\">#</span><span style=\"font-style:italic;color:#66d9ef;\">type</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Type::Bisexual}} </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">Flag size</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{Size::Large}} </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">FlagSection\n</span><span style=\"color:#f8f8f2;\">                title</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ffee99;\">&quot;Pride Flags&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()}}\n</span><span style=\"color:#f8f8f2;\">                flags</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{vec![Type::Rainbow, Type::Transgender, Type::NonBinary]}}\n</span><span style=\"color:#f8f8f2;\">                id</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;pride&quot;\n</span><span style=\"color:#f8f8f2;\">            </span><span style=\"color:#f92672;\">/&gt;\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"color:#f92672;\">&lt;/&gt;\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p { "That's it. You're rendering flags like a boss." }
        h2 { id: "-real-use-cases",
            a { href: "#-real-use-cases", class: "header", "🧑\u{200d}🎨 Real Use Cases" }
        }
        ul {
            li {
                "🎉 "
                strong { "Celebrate Pride Month (e.g. This Dev Challenge)" }
                " without hand-rolling rainbow divs."
            }
            li {
                "🧪 "
                strong { "Build inclusive UI demos" }
                " for Rust-based component libraries."
            }
            li {
                "📚 "
                strong { "Educate with pride" }
                ": show flag tooltips and screen-reader descriptions."
            }
            li {
                "🏳\u{fe0f}\u{200d}⚧\u{fe0f} "
                strong { "Trans visibility?" }
                " Add the Transgender flag to your footer."
            }
        }
        p { "Want a whole grid of flags grouped under a section title?" }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">&lt;</span><span style=\"color:#f8f8f2;\">FlagSection\n</span><span style=\"color:#f8f8f2;\">    title</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{</span><span style=\"color:#ffee99;\">&quot;Non-Cis Energy&quot;</span><span style=\"color:#f8f8f2;\">}}\n</span><span style=\"color:#f8f8f2;\">    flags</span><span style=\"color:#f92672;\">=</span><span style=\"color:#f8f8f2;\">{{vec![Type::Agender, Type::Genderfluid, Type::Transgender]}}\n</span><span style=\"color:#f8f8f2;\">    id</span><span style=\"color:#f92672;\">=</span><span style=\"color:#ffee99;\">&quot;non-cis&quot;\n</span><span style=\"color:#f92672;\">/&gt;</span></pre>\n" }
        p { "Yes, even your flag containers can slay." }
        p {
            "Just imagine Ferris the crab walking across a NonBinary flag, wearing a tiny hat and waving a trans-colored claw. That's the vibe we're channeling."
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// Rust + CSS + Queer joy = Pride RS 🏳\u{fe0f}\u{200d}🌈🦀</span></pre>\n" }
        h2 { id: "-whats-next",
            a { href: "#-whats-next", class: "header", "🛠\u{fe0f} What's Next?" }
        }
        ul {
            li { "🌀 More complex flags (with shapes!)" }
            li { "🌍 International pride flags" }
            li { "🧩 SSR compatibility (if that's your thing)" }
        }
        h2 { id: "-for-judges",
            a { href: "#-for-judges", class: "header", "🧑\u{200d}⚖\u{fe0f} For Judges" }
        }
        p {
            "If you'd like to test this project locally, you can do so using either "
            a { href: "http://github.com/opensass/pride-rs/tree/main/examples/dioxus",
                strong { "Dioxus" }
            }
            " or "
            a { href: "https://github.com/opensass/pride-rs/tree/main/examples/yew",
                strong { "Yew" }
            }
            ". Please refer to the README files for detailed instructions on how to run it locally."
        }
        h2 { id: "-final-thoughts",
            a { href: "#-final-thoughts", class: "header", "💬 Final Thoughts" }
        }
        p {
            "If you're building a Rust-based web UI and want to include a splash of "
            strong { "queer pride" }
            ", accessibility, and joy, "
            strong { "Pride RS" }
            " is your new best friend."
        }
        ul {
            li {
                p { "✅ Built with Rust" }
            }
            li {
                p { "✅ Powered by CSS flex" }
            }
            li {
                p { "✅ Fully accessible" }
            }
            li {
                p { "✅ Entirely customizable" }
            }
            li {
                p { "✅ Backed by Ferris's rainbow crab energy" }
            }
        }
        blockquote {
            p { "Add it. Ship it. Celebrate it 🏳\u{fe0f}\u{200d}🌈🦀." }
        }
        p {
            "And hey, if you made it this far, consider "
            a { href: "https://discord.gg/b5JbvHW5nv", "joining the Open SASS Discord" }
            ". We've got a dedicated channel for the queer rusty web, one div at a time."
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/8a7r5iibqyvsz4pf1tql.png",
                alt: "discord",
                title: "",
            }
        }
        p {
            "Till next time: "
            em { "Keep Rustin', keep Pride'n" }
            " 💖"
        }
    }
}
#[component(no_case_check)]
pub fn PrideHeroRelease() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        blockquote {
            p { "Hello 👋!" }
        }
        p {
            "So, yesterday, I released "
            a { href: "https://github.com/opensass/pride-rs",
                code { "pride-rs" }
            }
            " to allow Rustaceans to easily add pride flags to their landing pages. But then I thought, \"Hmm. This is nice, but it's not nearly gay "
            em { "enough." }
            "\" There was no theatrical entrance, no bright attractiveness, no Ferris the crab doing a dance move on a Progress Pride flag."
        }
        p {
            "So obviously, I needed to take things to the "
            em { "next level" }
            "."
        }
        p {
            "I should push it to the absolute extreme. I needed to put flags somewhere big, bold, fabulous, and meaningful. Naturally, I used "
            a { href: "https://github.com/opensass/hero",
                strong { "hero" }
            }
            " crate: a package for making hero sections in WASM: and decided it was time to inject it with unapologetic queer energy."
        }
        p {
            "Thus, the "
            strong {
                code { "pride" }
            }
            " component was born."
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/hz3hzfxdxtm5p6ie8na9.webp",
                alt: "drinking rust",
                title: "",
            }
        }
        h2 { id: "what-is",
            a { href: "#what-is", class: "header", "What Is " }
            code { "pride" }
        }
        p {
            "Think of  "
            code { "pride" }
            " as the drag queen cousin of the classic hero section: big text, big flags, big feels."
        }
        p {
            "It's a "
            em { "hero landing page component" }
            ", built with the same modularity and power of "
            code { "hero" }
            ", but now capable of screaming \"🏳\u{fe0f}\u{200d}🌈 Gay Rights!\" on page load. It's designed for events, queer community shout-outs, or just asserting your gender in the HTML."
        }
        p { "You can now build pages like:" }
        ul {
            li { "\"Hey trans Rust devs, meet up at 5PM for iced coffee and WASM talk!\"" }
            li { "\"Lesbian Rustaceans Unite: Queer systems programming social.\"" }
            li { "\"Ferris the crab is nonbinary now. Come to our journal night.\"" }
        }
        h2 { id: "demo",
            a { href: "#demo", class: "header", "Demo" }
        }
        p {
            "Here's what the  "
            code { "pride" }
            " landing page look like using Yew:"
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/2h7rdsw83semx2zlhmuv.gif",
                alt: "Demo 1",
                title: "",
            }
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/bdtvyvwq5wt2rlvxtkn8.gif",
                alt: "Demo 2",
                title: "",
            }
        }
        p {
            "That's it. You now have a stunning, queer-coded hero section greeting your users like a warm hug and a sparkle bomb."
        }
        h2 { id: "why-this-exists",
            a { href: "#why-this-exists", class: "header", "Why This Exists" }
        }
        p {
            "Let's be real: dev events, even Rust ones, can feel a little... cis-het-middle-aged-microservice-coded."
        }
        p { "And while we love robust type systems, sometimes what we really want is a space to say:" }
        blockquote {
            p {
                "\"Hey, let's just hang out, talk open source, and also maybe be a little gay about it.\""
            }
        }
        p { "This component is an invitation. A declaration. A banner saying:" }
        blockquote {
            p { "\"Queer Rust devs are here, and we brought snacks.\"" }
        }
        p {
            "It helps organizers quickly build landing pages for events, meetups, game jams, mutual aid projects, journal nights, or \"let's write poetry in Rust\" circles."
        }
        h2 { id: "canon-update",
            a { href: "#canon-update", class: "header", "Canon Update" }
        }
        p {
            "Ferris the Crab? You mean "
            em { "Fabulous" }
            " Ferris? Yeah, Ferris is canonically queer. We've decided."
        }
        p { "They/them pronouns. Wears eyeliner. Possibly dating Clippy." }
        p {
            "In fact, we believe every flag in  "
            code { "pride-rs" }
            " is a little home Ferris can march across."
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/hivupl6bxztiexnff64w.png",
                alt: "dababy side eye",
                title: "",
            }
        }
        h2 { id: "future-ideas",
            a { href: "#future-ideas", class: "header", "Future Ideas" }
        }
        p {
            "Coming soon to  "
            code { "pride" }
            ":"
        }
        ul {
            li { "Mobile responsiveness" }
            li { "Document the Design and thought process" }
            li { "Animated flag transitions" }
            li { "Confetti drop on CTA click" }
            li { "Server-side rendering support" }
            li { "Ferris doing a little jig on hover" }
        }
        h2 { id: "community-love",
            a { href: "#community-love", class: "header", "Community Love" }
        }
        p {
            "Building queer-inclusive spaces in tech isn't just about HR policies, it's about what we ship. Even a single pride flag on a dev landing page can make someone feel seen."
        }
        p {
            "Let's queer up the web one  "
            code { "div" }
            " at a time. In Rust. With  "
            code { "hero" }
            ". With Pride."
        }
        h2 { id: "for-judges",
            a { href: "#for-judges", class: "header", "For Judges" }
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/5izpe9uaqrzdmed7kiz5.jpg",
                alt: "show me",
                title: "",
            }
        }
        p {
            "If you'd like to test this project locally, you can do so using "
            a { href: "https://github.com/opensass/hero/tree/main/examples/pride-hero",
                strong { "Yew" }
            }
            ". Please refer to the "
            a { href: "https://github.com/opensass/hero/blob/main/examples/pride-hero/README.md",
                "README file"
            }
            " and "
            a { href: "https://github.com/opensass/hero/blob/main/src/yew/pride.rs",
                "the source code"
            }
            " for detailed instructions on how to run it locally."
        }
        h2 { id: "final-thoughts",
            a { href: "#final-thoughts", class: "header", "Final Thoughts" }
        }
        p {
            "If you're planning a queer-friendly event, building a Rust community initiative, or just want your site to slay, the  "
            code { "pride" }
            " component in  "
            code { "hero" }
            " has your back."
        }
        p {
            "It's fast, accessible, and deeply, "
            em { "flamboyantly" }
            " Rust."
        }
        blockquote {
            p { "Add it. Ship it. Celebrate it 🏳\u{fe0f}\u{200d}🌈🦀." }
        }
        p {
            "P.S. Join us in the "
            a { href: "https://discord.gg/b5JbvHW5nv", "Open SASS Discord" }
            ": we have a queer rust web channel and it is exactly as lovely as you'd hope."
        }
        p {
            img {
                src: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/8a7r5iibqyvsz4pf1tql.png",
                alt: "discord",
                title: "",
            }
        }
        p {
            "Till next time: "
            em { "Keep Rustin', keep Pride'n" }
            " 💖"
        }
    }
}

use dioxus::prelude::*;
use crate::components::blog::code::CodeBlock;
