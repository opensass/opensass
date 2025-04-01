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
                        title: "Announcing Open SASS 🚀 |---| announcement |---| announcing-open-sass |---| Nov 10 2024 |---| Welcome to Open SASS. Your open-source platform for building the future of SaaS with Rust and Wasm. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
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
                        title: "✨ Introducing X-AI: A Blazingly Fast 🚀 𝕏-AI Rusty CLI, TUI and SDK. |---| announcement |---| introducing-x-ai |---| Nov 18 2024 |---| Today, we are excited to announce the release of 𝕏-AI, your gateway to the X-AI API in Rust. |---| https://github.com/user-attachments/assets/e18b9fc2-7b7d-4125-86fe-c1b91fdb0f93"
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
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "Announcing Open SASS 🚀 |---| announcement |---| announcing-open-sass |---| Nov 10 2024 |---| Welcome to Open SASS. Your open-source platform for building the future of SaaS with Rust and Wasm. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
                            .to_string(),
                        location: Some(BookRoute::AnnouncingOpensass {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: "✨ Introducing X-AI: A Blazingly Fast 🚀 𝕏-AI Rusty CLI, TUI and SDK. |---| announcement |---| introducing-x-ai |---| Nov 18 2024 |---| Today, we are excited to announce the release of 𝕏-AI, your gateway to the X-AI API in Rust. |---| https://github.com/user-attachments/assets/e18b9fc2-7b7d-4125-86fe-c1b91fdb0f93"
                            .to_string(),
                        location: Some(BookRoute::IntroducingXAi {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
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
            em { "© 2024 Open SASS | Built with ❤\u{fe0f} by and for the Rust community." }
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
            em { "© 2024 Open SASS | Built with ❤\u{fe0f} by and for the Rust community." }
        }
    }
}

use dioxus::prelude::*;
use crate::components::blog::code::CodeBlock;
