> **Hey Rustacean 👋**

Let's play a little game: Imagine you're deep in the weeds building a shiny new frontend app. You've got your Rust + WASM stack humming, your `Dioxus` components are crisp, and your state management is so clean it practically sparkles. But then... the inevitable happens:

You need a dropdown. Or an accordion. Or a form layout. You google for 45 minutes, copy-paste something from five different GitHub gists, fight with some Tailwind utility class monsters, and then... you cry softly into your keyboard.

We've all been there. Ferris too 🦀.

That's exactly why we built **[Open SASS Kit](https://github.com/opensass/kit)**: your new best friend in the chaos of web UI.

![ohmygodtu](https://c.tenor.com/5_xNa2QYmFYAAAAd/tenor.gif)

## 🧰 What Is It?

**Open SASS Kit** is not just a toolkit. It's not just a component library. It's not even just a CLI. It's the **centralized home for open, reusable, modular Open SASS style components**, handcrafted for modern web apps, and especially for our beloved WASM world. Think of it as the **Wikipedia of Open SASS UI components**.

We're talking framework-agnostic, no-bloat, plug-and-play components that work with:

- Tailwind
- Bootstrap
- Bulma
- Foundation
- Plain ol' vanilla CSS
- Or your own secret in-house monstrosity that you definitely promise to refactor "later"

## 🎨 But, Why?

Let's just say it: CSS frameworks are great... _until they're not_. You're either locked into a giant opinionated stack or stuck rewriting the same "responsive card" component 23 times per project. **Open SASS Kit** lets you break free.

- 🧩 **Modular components**: Import only what you need. Keep your bundle tight and your sanity intact.
- 🔁 **Composable & reusable**: Components that don't fight back. Write once, use forever.
- ⚙️ **Powerful CLI**: Want to scaffold an accordion in your Yew project? One command. `os add accordion-rs yew`. Done.
- 🌐 **WASM-ready**: Native support for Rust + WASM frontend stacks like Yew, Dioxus, and Leptos.
- 🧼 **No vendor lock-in**: Use with _any_ CSS framework. Or none. Your stack, your rules.

It's like Tailwind UI and Bootstrap had a baby, and then raised it with Rustacean values: freedom, composability, and just a touch of healthy minimalism.

## ⚡ How It Works?

You start by installing the CLI:

```sh
cargo install opensass
```

Then you grab a component:

```sh
os add accordion-rs yew
```

That's it. No boilerplate. No duct-tape. The CLI does the file generation, wiring, and styling integration for you. And yes, it _actually_ works.

Under the hood, Open SASS Kit is powered by carefully crafted, lightweight Open SASS style components that are fully compatible with the latest Rust-based frontend frameworks. And because it's fully open, you can tweak, extend, or fork to your heart's content. Add some dark mode. Replace our icons with your favorite set of crabs. Make it your own 🦀.

## 📚 A Living Library of Components

At its core, Open SASS Kit is more than a toolkit, it's a **growing centralized catalog**. Every component we add is carefully documented, versioned, and designed to be useful across stacks.

It's not just for Rust folks (though let's be honest, we _do_ have the best taste). It's for anyone tired of bloated UI kits, rigid frameworks, or reinventing the same dang dropdowns.

Our goal is to make Open SASS the _universal toolkit_ for modern web projects, the one place where you can explore everything available, pick what you need, and get on with shipping actual features.

> TL;DR: No more scrolling through 20 Medium articles to style a button. Just run `os add accordion-rs yew`, and keep moving.

## 🧪 Built for WASM Frameworks

We know where the future's headed, and it's full of `wasm32-unknown-unknown` targets. That's why every Open SASS component plays beautifully with:

- [`Yew`](https://yew.rs)
- [`Dioxus`](https://dioxuslabs.com)
- [`Leptos`](https://leptos.dev)

Each component comes with integration examples and sensible defaults.

## 🤝 Join the Party

We're building Open SASS Kit _with_ the community, not just _for_ it. That means:

- Found a bug? Tell us.
- Got a better version of a component? PR it.
- Built something awesome with Open SASS? Show it off in our [Discord](https://discord.gg/b5JbvHW5nv).

Contributing is simple:

1. Fork the repo
1. Create a branch (`feature/my-awesome-component`)
1. Submit a pull request

Bonus points if you name your branch something like `fix/ferris-got-stuck-in-the-carousel`.

## 📈 This Kit Is Just Getting Started

Open SASS Kit is still young, but it's growing fast, like Ferris after a long nap and four cups of espresso. New components are being added regularly. Docs are being improved. The CLI is evolving. In other words: **now's the best time to get involved**. Whether you're a grizzled Rustacean, a newbie web dev, or just someone who wants to stop styling buttons manually, Open SASS Kit has something for you.

## 🎤 Final Thoughts

Look, building modern UIs shouldn't feel like assembling IKEA furniture in the dark, with no instructions, and a screaming crab on your shoulder. With Open SASS Kit, it doesn't have to. We're giving you the tools to **move fast, build confidently**, and never write the same accordion component twice. Ever again. So stop scrolling through old Gists. Stop fighting your CSS framework. Start building like you mean it, with a toolkit that's actually on your side.

> **We are Open SASS, babe!**.

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋!
