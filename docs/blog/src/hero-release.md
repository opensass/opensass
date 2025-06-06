> Welcome 👋!

If you've ever tried building a stunning landing page in Rust with a WASM frontend framework like **Yew**, **Leptos**, or **Dioxus**, you've probably run into the same issue: your "hero section" looks more like a background NPC.

You want that perfect first impression, bold headline, snappy call-to-action, beautiful image layout, but you don't want to fight the layout engine or hard-code 47 utility classes every time.

That's why we're excited to announce [**Hero**](https://crates.io/crates/hero), the _zero-to-hero_ crate for building powerful, customizable hero sections in Rust's favorite WASM UI frameworks.

And yes, Ferris the crab gave it a big claw-five 🦀✋ because this crate is _shell-shockingly_ good.

![high five](https://c.tenor.com/wqe1eEp7Bb0AAAAC/tenor.gif)

## What Is Hero?

**Hero** is a battle-tested collection of **drop-in hero section components** designed to work seamlessly with Yew, Leptos, and Dioxus. From launching a landing page to showcasing a new product or giving your Rust app the glow-up it missed in the '90s, `hero` is here to back you up. It's lightweight, unopinionated, accessible, responsive by default, and _ridiculously easy to customize_. Think of it as your frontend starter spell, just a few lines of code, and boom: instant wow factor.

## Why Hero?

Let's break it down like Ferris breakdancing at a RustConf afterparty:

- **Responsive by Default** Mobile-first layouts without even trying. It just works.

- **Totally Customizable**: Want Tailwind? Inline styles? Class overrides? No problem, you do you.

- **Pluggable Components**: Drop in custom headings, buttons, images, even other components like it's Lego.

- **Theme-agnostic**: No hardcoded styles here. Light theme? Dark theme? Custom gradients and funky fonts? Hero doesn't judge.

- **Framework Agnostic**: Works out-of-the-box with **Yew**, **Leptos**, and **Dioxus**, plus anything else brave enough to touch the WASM void.

## Getting Started with Yew

### Add the Crate

```bash
cargo add hero --features=yew
```

### Import the Hero You Need

```rust
use hero::yew::hero1::Hero as Hero1;
use hero::yew::hero2::Hero as Hero2;
use hero::yew::hero4::Hero as Hero3;
use hero::yew::hero3::Hero as Hero4;
```

### Basic Example

```rust
use yew::prelude::*;
use hero::yew::hero1::Hero;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Hero
            heading="Build Fast in Rust"
            description="Drop-in hero sections for Yew, Leptos, and Dioxus."
            title_style="font-size: 3rem; font-weight: bold; color: #4F46E5;"
            description_style="font-size: 1.25rem; color: #6B7280;"
            cta_style="padding: 0.75rem 1.5rem; background-color: #4F46E5; color: white; border-radius: 0.5rem;"
        />
    }
}
```

Congrats, you just went from "blank screen" to "production-ready landing page" in under 10 lines of Rust.

## Tailwind Friendly

Prefer class-based styling? `hero` doesn't mind.

```rust
<Hero
    heading="Launch Ultra-Fast Apps"
    description="Style your hero section with Tailwind, inline styles, or any CSS framework."
    container_class="max-w-6xl mx-auto px-4 py-24"
    title_class="text-5xl font-extrabold text-center text-white"
    description_class="mt-4 text-xl text-center text-gray-300"
    cta_class="mt-6 bg-white text-black px-6 py-3 rounded-full shadow-lg hover:bg-gray-100"
/>
```

Mix and match styles, props, and components like you're playing modded Minecraft, except instead of creepers, you're battling CSS bugs.

## Full Control via Props

Each `Hero` component exposes a smorgasbord of props for maximum flexibility:

- **Content Props**: `heading`, `description`, `cta`, `tabs`, etc.
- **Styling Props**: `title_style`, `cta_style`, `container_style`
- **Class Props**: `container_class`, `title_class`, etc.
- **Accessibility Props**: `aria_label`, `heading_tag`, `role`

Want a `div`? You got it. Want it to be an `h2` with ARIA support? Easy.

## Feature Recap

| Feature                    | Why You'll Love It                             |
| -------------------------- | ---------------------------------------------- |
| 4 layout variants          | Variety without complexity                     |
| Full theming support       | Dark mode, light mode, even cyberpunk          |
| Works in Yew/Leptos/Dioxus | Build wherever Ferris dares to roam            |
| Easy integration           | No config files, no head-scratching            |
| Custom components support  | Drop in tabs, buttons, or an actual Ferris GIF |

Speaking of which, Ferris says "this hero section slaps".

## Final Thoughts

Hero isn't just another frontend wrapper, it's **your first impression, delivered in idiomatic Rust**.

- ✅ Works with your stack.

- ✅ Respects your styling choices.

- ✅ Looks great on every device.

- ✅ Comes with four layout variants and infinite potential.

Ferris didn't _choose_ to be the face of Rust. But if he had a website, we're pretty sure he'd use `hero` to put his best claw forward.

**Go be the hero your frontend deserves.**

> At Open SASS, we're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋!
