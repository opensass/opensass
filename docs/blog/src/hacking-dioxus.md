## TL;DR

If you're hiring vibe coders, think again before it's too late. This post isn't about dissing Dioxus; It's about raising awareness around the fragility of modern software engineering, especially when inexperienced developers use powerful tools. Always hire engineers with the proper experience, particularly when working in critical areas like full-stack development. Vibe coding isn't inherently bad, but when handed to the wrong people, it becomes a dangerous practice.

## Introduction

> Hello friends 👋!

Today I want to share with you a deep frustration, a boiling discontent built up over two sleepless weekends trying to report and explain multiple security vulnerabilities to a well-known and publicly available open-source project in the Rust ecosystem: [Dioxus](https://github.com/dioxusLabs/dioxus). For those unfamiliar, Dioxus is a modern full-stack UI framework for Rust, and its promises are big. It aims to deliver the kind of seamless development experience we've grown to expect from JavaScript ecosystems, but powered by Rust's memory safety and type system. While its goals are admirable and its developers clearly passionate, what I've encountered has exposed a darker side of the modern "move fast and break things" mindset that has mixed into even our most robust and theoretically secure ecosystems.

This blog post is a reflection on not just Dioxus itself, but the overall degeneration of software engineering practices caused by what I refer to as "vibe coding". It's not an attack on Dioxus as a tool or its creators personally. Instead, this is a petition to the industry to take security seriously. Dioxus merely serves as a case study, a concrete example of the dangers of prioritizing developer experience and hype over secure engineering principles. And before anyone gets defensive, understand: I've been in this field for years, if you measure experience by sleepless hours, millions of lines of code written, and real systems that were actually used, and I've dealt with real security threats. This isn't my first time doing this.

The title is provocative, yes, but it is also literal. Vibe coding _is_ destroying the very foundation of what software engineering used to mean: precision, responsibility, reliability, security. I completely understand that product managers and investors breathe down your neck for fast delivery. I've shipped code under pressure. I've dealt with the "why isn't it done yet?" beam. But none of that justifies ignoring the fundamentals. We're not building TikTok filters, we're constructing software that handles real data, affects real users, and, in some cases, controls real-world infrastructure. A culture of shipping at all costs with minimal inspection is not only irresponsible, it's dangerous.

This post will take you through multiple security flaws discovered in Dioxus over the past two weeks. It will explain how these issues could have been avoided with proper engineering practices and why vibe coding, where the developer is more concerned about the aesthetic or the "feel" of writing code rather than understanding the underlying system, leads to systemic failures. And yes, while some Dioxus maintainers may claim these aren't "security issues", the principles of secure software design demand we treat them as such.

## What is a Software Engineer / Developer?

![Vibe Coding](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/hi9vga85glmaycw2hzf3.webp)

Before we get into the complicated stuff, let's clarify some terminology that the industry often uses interchangeably but should not. A software engineer is fundamentally different from a developer. While both write code, the responsibilities and mindset between the two are worlds apart. A software engineer is someone who is capable of designing systems from first principles. They understand memory models, design constraints, threat surfaces, performance bottlenecks, and scalability trade-offs. They don't just write code, they architect robust, maintainable, and secure systems that solve real-world problems in an optimized manner.

On the other hand, a software developer is typically focused on building within the confines of those systems. They may be responsible for adding features, fixing bugs, tweaking performance, or even just connecting APIs together in a frontend interface. There's nothing inherently wrong with being a developer. But conflating the two leads to mismatched expectations, especially in hiring processes or in open-source communities where contributions affect thousands of users.

Why is this distinction important in a post about Dioxus and vibe coding? Because Dioxus, like many modern frameworks, abstracts away a lot of complexity, which makes it very inviting to developers who are new or who may not understand the consequences of their actions. This abstraction isn't evil by itself. In fact, well-designed abstraction is one of the cornerstones of good engineering. But when those abstractions are handed to people who don't fully understand the implications of the code they're writing, particularly in a systems language like Rust, we get insecure software.

Software engineers are responsible not just for the code they write but for the consequences of that code in production. When a type system is bypassed, when untrusted input is handled without validation, when unsafe code is introduced without proper justification and bounds checking, that is not a momentary lapse in judgment. That is a systemic failure in design responsibility. That's what vibe coding enables.

Ultimately, the security and integrity of an application lies in the hands of the engineers behind it. If something goes wrong, pointing fingers at the "framework" or the "community" is insufficient. The engineer must accept responsibility. And so, if a tool encourages insecure patterns or fails to enforce good practices, it's not just a developer problem, it's a failure of engineering culture, documentation, and design. This is where Dioxus, as a case study, becomes illustrative.

## What is Vibe Coding?

![Vibe Coding](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/2v8i3jpte4e79yo1phll.webp)

Let's now take a moment to define this central concept: _vibe coding_. Vibe coding is the act of programming based on "feel" rather than understanding. It's when a developer wires together code snippets they've seen in documentation, StackOverflow, or AI-generated outputs without understanding the system's internals. It's the act of treating your framework or language as a magical black box, feeding it inputs, hoping for the right outputs, and assuming that "if it compiles, it's probably fine".

Vibe coding is particularly dangerous in systems programming. Languages like Rust were designed to enforce safety through strict compile-time guarantees. But even Rust cannot protect your system if you deliberately (or ignorantly) bypass those guarantees. Unsafe blocks, dynamic plugin systems, stringly-typed APIs, all of these are opportunities for subtle, dangerous bugs when wired without understanding.

When someone vibe codes a UI in React, the worst-case scenario might be a broken button or a misaligned div. When someone vibe codes a server function in Dioxus using unsafe function pointers, CSRF-vulnerable APIs, or SSRF-prone static site generation, the consequences scale up quickly. And yet, because modern frameworks accommodate developer ergonomics, they don't always poke the developer toward secure or well-reasoned patterns. They prioritize speed, simplicity, and joy. But joy in programming should not come at the expense of discipline.

It raises an important question: when software fails, who is responsible? The developer who vibe coded it? The framework that encouraged it? The runtime that failed to detect it? In truth, all share a slice of the blame. But the lion's share falls upon the person who wrote the code. Because the machine does not write your software. It merely executes it. No matter how advanced LLMs become or how "magical" our dev tools feel, the human remains in the loop.

This is not just a philosophical discussion. It has real-world implications, as the next section will demonstrate. Because over the course of two weekends, I went deep into Dioxus's internals, not as a casual user, but as a software engineer intent on breaking it open and seeing what lies beneath. What I found was troubling. What I reported was met with dismissal. And what I learned is that we are sleepwalking into a new era of insecure software built by developers who don't know what they're building.

## Hacking Dioxus

![Hacking Dioxus](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/ki379976xe3gn59e2g6w.png)

I've been using Dioxus full-time for over 7 months. Not passively. Not as a toy. I've used it in production systems, built fullstack apps, integrated with WebAssembly, deployed to cloud environments, and experimented with server-side rendering and static generation. My feedback comes not from a place of friction, but from a place of personal familiarity. And what I found during a deep audit of its codebase should concern anyone using or contributing to the project.

Let's begin with the first vulnerability: **Open Redirect in the `Link` component**.

### Open Redirect Vulnerability

This issue was reported [here](https://github.com/DioxusLabs/dioxus/issues/4134). It may sound minor, but its implications are not. The Dioxus `Link` component currently accepts arbitrary strings as its `to` parameter. That means a developer can write something like:

```rust
Link { to: "https://some-malicious-website.com" }
```

The maintainers argue that this is fine, it's up to the developer to decide whether they're linking internally or externally. But here's the catch: the `Link` component is part of the Dioxus router, which is meant for _internal_ navigation. It exists to manage in-app routing, maintain client-side history, and provide a seamless UX without full page reloads. Allowing arbitrary external URLs through this component breaks the contract of trust between the developer and the router system. It blurs the line between internal routing and external redirection in a way that opens up phishing, and redirect attack vectors.

Compare this to [Yew](https://yew.rs), a Rust framework that does this correctly. [Yew's `Link`](https://docs.rs/yew-router/latest/yew_router/components/struct.Link.html) component only accepts values from a [`Routable`](https://docs.rs/yew-router/latest/yew_router/trait.Routable.html) enum. This enforces compile-time guarantees that a route is valid and internal. You cannot accidentally pass in a user-controlled string and redirect them to a malicious site. That's type safety. That's Rust's promise. And that's what Dioxus breaks.

So when I filed this as a vulnerability, I wasn't just nitpicking. I was advocating for Dioxus to honor Rust's philosophy: _preventing errors at compile time wherever possible_. The maintainers disagreed. One even said, dismissively, "This is not a security vulnerability and it's ridiculous to claim it is." That kind of tone is not only unprofessional, it's dangerous. It discourages responsible disclosure. It chills open source security culture. And it misses the point entirely.

I proposed a simple fix: if a user wants to link to an external site, use the standard HTML `a` tag:

```rust
a { href: "https://google.com" }
```

And if the user wants in-app routing, use:

```rust
Link { to: Route::Home }
```

This ensures type safety, separation of concerns, and better DX _without_ compromising security. A win-win. But the maintainers were not interested. Perhaps because this fix wasn't "vibe" enough.

## CSRF in Server Functions

Let's start by analyzing how CSRF works at a technical level and why it becomes a critical threat when not properly mitigated in server-side APIs, especially those serving frontend clients over the web. Cross-Site Request Forgery is a well-known and dangerous form of attack where an attacker tricks a user into performing unwanted actions on a web application in which they're currently authenticated. These actions can range from changing account settings, submitting data, or even making financial transactions. The core vulnerability comes from the fact that browsers automatically attach cookies to requests, including authentication tokens and session cookies, making the victim's authenticated state exploitable if not validated properly. In this case, Dioxus's server functions, designed to provide developer-friendly async APIs with automatic serialization and routing, lack built-in CSRF protection, which puts developers in a precarious position where secure-by-default isn't guaranteed.

[When I proposed the `#[with_csrf]` macro](https://github.com/DioxusLabs/dioxus/issues/4190#issuecomment-2907717675), it wasn't just about syntactic sugar or convenience, it was about aligning the Dioxus stack with the principle of secure defaults, something that the Rust language and its ecosystem pride themselves on. Rust's core philosophy is centered around safety and correctness. The moment we shift that burden of correctness entirely onto the developer, we break that implicit promise. Let's take frameworks like `Next.js` as an example. Even though it's written in JavaScript, an infamously permissive and unsafe language, `Next.js` still goes out of its way to encourage [CSRF](https://nextjs.org/blog/security-nextjs-server-components-actions#csrf) tokens and offers [middleware](https://nextjs.org/docs/app/building-your-application/routing/middleware) and utilities that reduce the chance of such oversights. The argument that Dioxus shouldn't be responsible for CSRF because it doesn't manage sessions or authentication directly is, in my opinion, insufficient. Providing security primitives like CSRF tokens should be the _minimum_ any modern fullstack web framework offers, this isn't asking for a feature; this is about foundational safety in a connected world where web exploitation is the norm, not the exception.

Furthermore, from a developer experience standpoint, introducing a `#[with_csrf]` procedural macro adds virtually no additional cognitive overhead, but dramatically improves the likelihood that server functions are protected against CSRF attacks. The proposed implementation could easily check for a valid `X-CSRF-Token` in the request headers and validate it against a signed session token. This is similar to what popular frameworks like [Django](https://docs.djangoproject.com/en/5.2/ref/csrf/) and [Laravel](https://laravel.com/docs/12.x/csrf) have done for years. It's a battle-tested pattern. What I'm asking for isn't new or revolutionary, it's standard, mature, and secure. What makes Rust unique is that it allows us to do all this at compile time with strong type checking, minimizing room for human error.

Now, when I raised this as an [issue](https://github.com/DioxusLabs/dioxus/issues/4132), I was met with the counterpoint that enforcing CSRF tokens universally would restrict valid use cases, like calling server functions from unauthenticated APIs or external clients. And yes, that's technically true, but that's precisely why I suggested making it an _opt-in_ system. This is not about enforcing behavior globally, but about giving developers the _option_ to choose the secure path with minimal friction. If you're building an app where CSRF is not a concern, you simply don't add the macro. If you're building an app that deals with forms, user inputs, account management, or anything remotely sensitive, you slap `#[with_csrf]` on top of the server function and move on with confidence. How is that a bad tradeoff?

It also seems like the Dioxus team is deeply committed to keeping the framework light and developer-friendly, which I greatly respect. In fact, I admire the effort that has gone into the router, server functions, and CLI tooling. However, friendliness should not come at the cost of safety. Even if we assume that most developers are smart and security-conscious, we cannot assume that _every developer is_. Security must be idiot-proof, not because we think developers are idiots, but because the stakes are just that high. A forgotten CSRF token is not an academic problem; it's a potential PR disaster or a data leak. And in today's hyper-connected world, one data breach is all it takes to lose user trust, investor confidence, and sometimes even legal ground under [GDPR](https://en.wikipedia.org/wiki/General_Data_Protection_Regulation) or [CCPA](https://en.wikipedia.org/wiki/California_Consumer_Privacy_Act).

If the Dioxus maintainers are concerned about maintaining clarity in the macro system, there are several routes to improve this. The macro could emit compile-time warnings if used incorrectly. We could even generate server logs that explain what the CSRF system is doing during runtime in debug mode. Better yet, we could allow users to configure a CSRF strategy at a higher level, something like `ServerConfig::enable_csrf_protection(true)` which makes every server function CSRF-aware by default, unless explicitly opted out. There are a dozen sane, ergonomic design paths we can follow to achieve this goal, and none of them degrade the developer experience.

I want to emphasize again that this isn't just about Dioxus. The Rust ecosystem needs to have a broader conversation about security ergonomics. We've gotten excellent at zero-cost abstractions, safe concurrency, and data race prevention, but web security, in the fullstack space, still feels like a second-class citizen. Libraries like `axum`, `actix`, and `warp` have CSRF middleware maintained by third parties. This fragmentation is bad for the ecosystem and makes it harder for new developers to follow best practices. A modern fullstack web framework like Dioxus is _the perfect place_ to show leadership and establish secure defaults out-of-the-box. It sets a strong precedent for other libraries to follow.

I've seen too many developers brush off CSRF as "not their problem", only to end up doing damage control after their app is compromised. A decade ago, this was forgivable. Today, it's not. So let me say it loud and clear for anyone reading this: if your app handles user input, authentication, or sensitive data, and you're not protecting your server functions from CSRF, then you are shipping a vulnerable app. It doesn't matter whether you used Rust or Brainfuck or Haskell. Security is language-agnostic. And it's high time we stop excusing unsafe defaults just because they make onboarding easier.

## DoS Caused by Arbitrary Function Pointer Transmute

This next [issue](https://github.com/DioxusLabs/dioxus/issues/4189) is probably one of the most outstandingly and technically shocking vulnerabilities I came across while exploring the internals of the Dioxus fullstack server runtime. Specifically, it involves the use of unsafe Rust code to transmute raw function pointers during hot reload operations in development mode. The relevant code resides in the hot reload path and involves this line:

```rust
let new_root = unsafe {
    std::mem::transmute::<*const (), fn() -> Element>(new_root_addr)
};
```

At a glance, the casual observer might not see why this is an issue, especially since this block of code is clearly marked as part of the development hot reload infrastructure. But anyone with experience in systems programming, or anyone who has ever been burned by undefined behavior in C or C++, should immediately see red flags here. Transmuting a raw pointer to a function pointer is a classically dangerous move unless you are absolutely sure that the pointer is valid and correctly aligned. In this case, the assumption is that the hot reload system can blindly accept any memory address provided via the loader and call it like a proper function. This is just asking for a segmentation fault, or worse.

And indeed, that's exactly what happens. If an invalid or malformed function pointer is introduced, intentionally or otherwise, the result is a runtime crash. You can trivially reproduce this by sending a bogus pointer to the reload system, causing the entire Dioxus fullstack server to segfault. This is not merely a bug. It is a weaponizable DoS vector, especially if an attacker can influence the plugin loading system. Even though the maintainers are correct that this only affects development builds, we cannot ignore the implications: developers running hot reload tools are now at risk of crashing their dev servers with malformed inputs. More importantly, this kind of unchecked unsafe logic sends the wrong message about Rust safety practices in high-level frameworks.

Let me be clear: the use of `unsafe` in Rust is sometimes unavoidable. Rust gives you the `unsafe` keyword not to avoid safety, but to isolate and explicitly contain unsound operations that would otherwise be impossible in safe code. But when using `unsafe`, you are entering a contract with the compiler and the runtime: _you must manually uphold all the guarantees that Rust normally provides for you_. That includes pointer validity, lifetime correctness, memory alignment, and type soundness. Transmuting raw pointers violates all of these unless handled with surgical precision. The current Dioxus code does none of that validation. It simply transmutes and executes.

So what's the fix? Actually, there are several. The most straightforward one is to validate the pointer before transmutation. If the pointer is null, or misaligned, the system should refuse to execute and return an explicit panic with a diagnostic message. This would prevent the segmentation fault and provide developers with a clear understanding of what went wrong. Alternatively, the system could require signed metadata for the loaded functions, ensuring that only trusted code paths are executed. This would effectively sandbox the reload system and dramatically reduce the attack surface. We could also adopt the pattern seen in hot-reloading plugins from [Unity](https://docs.unity3d.com/2022.3/Documentation/Manual/PluginInspector.html), where plugin registration is explicit and compile-time-checked. This would mean that hot reload targets would need to register their function exports explicitly, allowing the compiler to generate safe entry points. This way, any change to the application logic would require an explicit recompile of the plugin manifest, and all the address resolution could be validated against a known set of safe signatures.

In response to this report, the Dioxus maintainers asserted that since [this only affects development](https://github.com/DioxusLabs/dioxus/issues/4189#issuecomment-2907099136), the priority of fixing it is low. While I understand their reasoning, I respectfully disagree. Development-time tooling is often the first contact point for new users and teams evaluating a technology. If a developer encounters a crash during hot reload because of malformed function pointers, it doesn't matter that it's a dev-only issue, their perception of Dioxus as a reliable toolchain is already dulled. Worse, in large enterprise environments where development is done at scale across multiple teams and sandboxes, a rogue reload bug can bring down a staging environment or corrupt a shared cache. It's not just about safety; it's about trust.

What makes this particularly ironic is that Rust's strongest selling point, its promise of memory safety and crash resistance, is completely nullified by poor usage of `unsafe`. We don't get to brag about "fearless concurrency" and "safe systems programming" if we turn around and write code that would make a C compiler blush. Every `unsafe` block is a loaded gun. And it's the framework's job, not the developer's, to make sure that trigger isn't pulled without proper safeguards in place.

In short, this vulnerability is demonstrative of a deeper issue: vibe coding is creeping into the core libraries of a safety-first ecosystem. When we cut corners for the sake of speed, especially in `unsafe` contexts, we end up with fragile foundations that betray everything Rust stands for. That's not just poor engineering. That's a breach of trust with the entire community.

### SSRF in CLI SSG Loop

When you have a build tool like [the Dioxus CLI](https://crates.io/crates/dioxus-cli), which includes server-side rendering (SSR) and static site generation (SSG) capabilities, you're already operating in a semi-privileged environment. Even if "dx serve" is labeled as a development tool, the implications of introducing unvalidated input routes into a loop that performs HTTP requests should raise significant red flags, especially considering the increasing number of teams using these tools in CI/CD pipelines or for local staging environments. As reported in [this issue](https://github.com/DioxusLabs/dioxus/issues/4137), by blindly fetching each route using a formatted HTTP GET request, without sanitizing or validating these paths, you introduce a clear vector for Server-Side Request Forgery (SSRF). SSRF, as widely recognized in the [OWASP Top 10 list](https://owasp.org/Top10/) of security vulnerabilities, allows attackers to trick servers into making requests to unintended destinations, like internal systems, cloud metadata services (e.g., [AWS' `169.254.169.254`](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instancedata-data-retrieval.html)), or even other vulnerable services that trust internal traffic.

To quote OWASP directly: _"SSRF flaws occur whenever a web application is fetching a remote resource without validating the user-supplied URL."_ ([OWASP SSRF](<https://owasp.org/Top10/A10_2021-Server-Side_Request_Forgery_(SSRF)/#description>)). Dioxus's current CLI behavior precisely fits that description. And while the maintainers may have dismissed this by saying ["it's just a dev tool"](https://github.com/DioxusLabs/dioxus/issues/4137#issuecomment-2888929498), I challenge the logic behind accepting insecure defaults, even in dev-mode utilities. A dev tool used today might be embedded in tomorrow's automation pipeline. We all know that lines between "development" and "production" blur quickly in modern workflows.

Moreover, an attacker who can poison the route list, via environmental variables, or other misconfigurations, can easily exploit the SSRF vector. Even if it's "just development", the potential risk of leaking internal system data, or pivoting into more sensitive areas should warrant at least a minimum validation layer. We're not talking about rewriting the tool, just sanitize route strings before they're passed into the request loop. Ensure routes are relative paths, prevent `http://` or `https://` prefixed routes, and error out when absolute domains are encountered. These are trivial patches that could stop an entire class of attack vectors dead in their tracks.

It's ironic that Dioxus, a Rust framework meant to leverage Rust's strengths like type safety and security, is allowing basic security hygiene issues to go unchecked. Developers pivote to Rust because they want control and reliability. If your toolchain weakens the very foundation the language was built on, then you're not shipping Rust. You're shipping Vibe Rust™, a watered-down promise of what secure systems programming could be.

## Reflection

Let's be crystal clear here: this isn't about ego, disputation, or any personal campaign against a project. This is a much-needed wake-up call that echoes across the entire open-source Rust ecosystem. When I reported these issues, I did so not to nitpick or throw shade, but because I genuinely care about the quality of the tools we are all using. I've written countless lines of Rust over the past years, read through core Rust source, and followed the design philosophies of Rust since its early stable days. I've watched as people celebrated Rust's memory safety, but ignored the growing cracks in userland safety, especially in web frameworks and glue code.

Security is not an afterthought, nor should it ever be one. When you say, "developers are responsible for using the tools safely", you're technically correct, but morally negligent. Let me ask this: Would you trust a car company that says, "Oh, the brakes work if the driver applies them correctly, but sometimes they don't respond unless you write your own brake controller"? That's exactly what some libraries and frameworks are saying to developers right now. Shifting security responsibilities to the end user, especially when such issues can be proactively mitigated in the framework layer, is irresponsible engineering.

We're not talking about exotic attack vectors. We're talking about **Open Redirects**, **CSRF**, **SSRF**, and **segfaults**. These are day-one bugs. These are the types of vulnerabilities that show up in bug bounty reports and make headlines when they go unpatched in production apps. It is deeply concerning when the response to these reports is outright dismissal, immediate issue closure, or marking them as "spam" instead of engaging in a technical discussion. That's not just poor communication, it's bad governance.

I fully understand open source is a labor of love. Maintainers often work on nights and weekends, unpaid and underappreciated. But part of being an open source master is knowing how to respond to critical input. Even if you don't agree, the least you can do is not treat the messenger like the enemy. Otherwise, the entire idea of open collaboration collapses into a gatekept monoculture where critique is seen as attack and security is seen as "someone else's job."

## Final Thoughts

Let's revisit the big picture. Vibe coding isn't a term coined to insult people. It's a shorthand for describing a pattern where tools are used intuitively without foundational understanding. It's what happens when libraries become too easy to use at the expense of correctness. Developers start composing applications by copying examples, skimming docs, and leaning on autocomplete. And while that's okay for prototyping, it's absolutely unacceptable in production.

The Rust community has often prided itself on going slow, doing things right, and shipping quality code. But if we don't hold our frameworks to the same standard, we're just gaslighting ourselves. Security doesn't just live in the compiler. It lives in APIs. It lives in interfaces. It lives in assumptions baked into design. That's why we need ergonomic _and_ secure frameworks. That's why I wrote this post.

If you build systems that encourage cargo-culting, where people use features without understanding their impact, you are on the hook for making those features safer. If you dismiss every security report with "you're using it wrong", then you've built an unsafe abstraction. Periodt.

Dioxus has tremendous potential. The community is growing. The DX is excellent. But none of that matters if the foundation is riddled with avoidable pitfalls. My hope is that this post helps others understand that being fast and fun doesn't excuse being careless. Rust deserves better. Developers deserve better. Users deserve better.

If you're a Dioxus user, take a second look at how you're handling routing, server functions, and any unsafe FFI magic. If you're using Dioxus in a production deployment, even "just testing", audit your setup. Is there an open redirect? Are you issuing GETs to dynamic routes? Do your server functions validate inputs and defend against CSRF?

If you're a maintainer, of Dioxus or any Rust project, please consider the following:

- Don't treat security reports like spam. Take 5-10 minutes to validate the concern.
- Use type systems to prevent misuse, especially in routing and I/O APIs.
- Provide optional guardrails (macros, features, traits) for common security needs.
- Document security assumptions clearly. Make it hard to use things incorrectly.
- When in doubt, err on the side of safety. Rust taught us that. Live it.

Despite the critiques, I want to end this on a positive note. Dioxus is a brilliant project. The work being done to unify desktop, mobile, and web apps in a type-safe Rust UI layer is nothing short of visionary. The fact that I can run the same app on WASM, and native platforms with minimal code changes is incredible. Server functions are powerful. The SSG capabilities are ahead of their time. And OpenSASS being built entirely on top of it proves that Dioxus isn't just hype, it's usable and scalable.

But a great tool is not immune from critique. It's through feedback, yes, sometimes harsh, that tools evolve from "good enough" to "industry standard". I sincerely hope this blog is read in that spirit.

So, thank you to the Dioxus team. Thank you for the hours of work. Thank you for the documentation, the examples, the bug fixes, all of which I am actively contributing to and improving. Just please, take security as seriously as you take developer experience. Because in the end, DX without security is just fast failure.

> At Open SASS, we're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time, keep building, but build responsibly 👋!
