> Welcome 👋!

When you're building blazingly fast WASM apps, there's one thing that silently makes or breaks the user experience: **loading states**. We've all been there, blank screens, layout jumps, etc. It's not just bad UX. It's _lost trust_.

That's why we're excited today to introduce [**Skeleton RS**](https://crates.io/crates/skeleton-rs) , a **highly-configurable** skeleton loader built natively for **Rust WASM frameworks**, starting with full Yew support. Think of it as a smart placeholder for your content , one that speaks fluent WASM.

Let's unpack why **Skeleton RS** is your new best friend when it comes to loading UIs 🦴.

![sunglasses-glasses](https://github.com/user-attachments/assets/a130f1fc-891b-4d3b-878c-84310acd5759)

## 🦴 What Is Skeleton RS?

**Skeleton RS** is a high-performance skeleton loader component designed for frontend Rust frameworks like **Yew**. Whether you're fetching data, lazy-loading routes, or animating components into view, **Skeleton RS** gives you graceful, styled placeholders that blend into your design system _easily_.

No JavaScript hacks. No extra boilerplate. Just clean, declarative Rust 🦀.

### Built-in UX Goodness

- **Pulse & Wave animations** for that polished glow.
- **Auto-sizing** to match real content dimensions.
- **Visibility-based animation triggers** via `IntersectionObserver`.
- **Fine-grained control** via props without sacrificing simplicity.

## 🚀 Why You'll Love Skeleton RS

> You're not just faking a loading state. You're designing a seamless experience.

Skeleton RS isn't another "spinner in disguise". It's **intentionally minimal**, yet **insanely flexible**. Designed to look great out of the box, but gives you the controls when you need them.

Here's why it stands out:

- 🔍 **Context-Aware**: Animate only when visible , save cycles, look smoother.
- 🎯 **Responsive**: Works with any layout, from dashboards to mobile-first views.
- 🧱 **Composable**: Use it with or without children, nest it, theme it, customize it.
- 🪶 **Lightweight**: No JS, no noise , pure Rust + WebAssembly.

This isn't a loading hack , it's a design utility.

## ⚡ Quick Yew Setup

Using Skeleton RS in your **Yew** project is dead simple:

### 1. Add it to your dependencies

```sh
cargo add skeleton-rs --features=yew
```

### 2. Import the component

```rust
use yew::prelude::*;
use skeleton_rs::yew::Skeleton;
use skeleton_rs::Variant;
```

### 3. Drop it into your app

```rust
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Skeleton
            variant={Variant::Text}
            width="100%"
            height="1.2em"
            animate_on_visible={true}
        />
    }
}
```

Boom. Beautifully animated, accessible skeleton loaders, ready to roll.

## 🧩 Features

| Feature               | What It Brings to Your App                             |
| --------------------- | ------------------------------------------------------ |
| Variant Support       | Choose from `Text`, `Circle`, `Rect`, and more         |
| Visibility-Based Anim | Trigger animations only when elements are visible      |
| Delay + Fallback UX   | Avoid flicker with `delay_ms`, improve perceived speed |
| Responsive Design     | Enable `responsive` for fluid layouts                  |
| Theming               | Seamlessly toggle between light and dark modes         |
| Full Custom Styling   | Add your own `custom_style`, `class`, or radius        |

This is UX design, not just developer convenience.

## ⚙️ Full Control with Props

Skeleton RS gives you all the toggles and knobs you need , without overengineering.

```rust
<Skeleton
    variant={Variant::Rect}
    width="300px"
    height="200px"
    border_radius="8px"
    animation={Animation::Wave}
    show={true}
    delay_ms={300}
    theme={Theme::Dark}
    responsive={true}
/>
```

Want more? Use `infer_size` with children, tweak `margin`, plug into visibility with `node_ref`, and even animate on hover, focus, or click.

Yes, it really is that powerful , and that simple.

Let's say you're loading user cards or a product grid. Here's how you'd add meaningful skeletons _without_ breaking structure:

### 🧾 Text Loading Placeholder

```rust
<Skeleton variant={Variant::Text} width="80%" height="1.1em" />
```

### 🟦 Avatar Circle

```rust
<Skeleton variant={Variant::Circle} width="48px" height="48px" />
```

### 📦 Card Block

```rust
<Skeleton
    variant={Variant::Rect}
    width="100%"
    height="150px"
    animation={Animation::Pulse}
    border_radius="12px"
/>
```

### 🔁 With Child Content (Infer Size)

```rust
<Skeleton infer_size={true}>
    <div class="user-profile">{"@rustacean"}</div>
</Skeleton>
```

### 🕵️ Animate Only When Visible

```rust
<Skeleton animate_on_visible={true} />
```

All this, and no layout shifts. Just butter-smooth transitions that feel native.

## 💡 Final Thoughts

Modern apps need modern loading states , not just a spinning icon and a prayer. Whether you're building admin dashboards, real-time UIs, or portfolio sites with WASM, **Skeleton RS** gives you the power to design with empathy.

It's not flashy. It's **functional elegance** , designed for today's Rust-native frontend.

> **We are Open SASS, babe!**.

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋!
