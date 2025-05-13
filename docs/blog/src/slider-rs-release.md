> Hey Rustacean 👋!

You know what's surprisingly hard to get right? A good slider component.

Sure, you can slap together a [`<input type="range">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input/range) and call it a day, but once you start wanting **custom styling**, **accessibility**, **double-thumb range selectors**, and _gasp_ actual usability... well, things get messy.

That's why we're beyond excited (and mildly sleep-deprived) to announce the release of **[Slider RS](https://crates.io/crates/slider-rs)**: an open-source, fully customizable slider component for your favorite Rust & WASM frameworks like **Yew**, **Dioxus**, and **Leptos**.

It's slick. It's precise. It's accessible. And yes, **Ferris the Crab** personally approves 🦀.

![dababy do be vibin](https://github.com/user-attachments/assets/9a2511a7-f626-4c8c-bc3d-7d01e20b6d45)

## 🎉 Why Slider RS Exists

Modern web apps need sliders that don't suck. Periodt. From fancy dashboards to media scrubbers and range filters, every app deserves a component that:

✅ Looks good (without writing a CSS thesis).
✅ Works with screen readers (hello, accessibility!).
✅ Supports single & double thumbs (because sometimes, one thumb just ain't enough).
✅ Plays nice with WASM frameworks.
✅ Doesn't make you cry at 2 AM.

Slider RS gives you **fine-grained control** over styling, behaviors, and accessibility, all while staying efficient, reactive, and smooth. And yes, it even has **tooltips** for people who like their sliders with a touch of extra flair.

## 🧰 What Can Slider RS Do?

Let's break it down. Slider RS comes packed with features like:

- **🎚️ Single & Range Sliders**: One thumb? Two thumbs? Your choice.
- **🎨 Fully Customizable Styling**: Classes, inline styles, custom thumb content, icon slots, tweak it till it's yours.
- **🦽 Accessibility First**: ARIA attributes, keyboard navigation, focus management, it's all baked in.
- **⚡ Reactive & Efficient**: Optimized rendering with prop diffing.
- **🔢 Advanced Goodies**: Tick marks, tooltips, step indicators, horizontal & vertical orientation, you name it.
- **🖱️ Smooth UX**: Drag ranges, fine-tune with keyboard, hover for tooltips, smooth as butter.

Basically, it's the slider component you always _wished_ you had, now in Rust-flavored WASM form.

## 🚀 Getting Started

Here's how to get started with Yew:

### 1. Add Slider RS to Your Project

```sh
cargo add slider-rs --features=yew
```

### 2. Import the Slider Component

```rust
use yew::prelude::*;
use slider_rs::yew::Slider;
use slider_rs::Orientation;
```

### 3. Use It In Your App

```rust
#[function_component(App)]
pub fn app() -> Html {
    let value = use_state(|| 50.0);

    html! {
        <Slider
            min={0.0}
            max={100.0}
            step={1.0}
            value={Some(*value)}
            on_change={Callback::from(|val| log::info!("Slider changed to: {}", val))}
            orientation={Orientation::Horizontal}
            show_value=true
            show_steps=true
        />
    }
}
```

And boom, you've got a fully functional, accessible slider in your Rust app.

Even **Ferris** gave it a test drive (and you know how picky he is about UX).

## 🎨 But Wait, There's More

Want to customize everything? Good news, you can. Slider RS comes with a rich set of props to tweak appearance and behavior to your heart's content:

| ✅ What You Can Do            | 📝 How It Works                                          |
| ----------------------------- | -------------------------------------------------------- |
| Single or Range Mode          | `double={true}` for range selectors                      |
| Control Size & Color          | Use `size` and `color` props                             |
| Custom Icons & Thumbs         | Slots like `icon_start`, `icon_end`, `custom_thumb_html` |
| Full Styling Control          | Classes & inline styles for every part                   |
| Show Values, Steps & Tooltips | `show_value`, `show_steps`, `show_tooltip`               |
| Accessible by Default         | ARIA labels, keyboard nav, focus events                  |
| Reactive Callbacks            | `on_change`, `on_change_range`, `on_focus`, `on_blur`    |

It's like a Swiss Army knife, but for sliders. And it won't poke you in the pocket.

## 🦀 Ferris Approved, Open SASS Blessed

We built Slider RS because **Rust+WASM developers deserve better components**. It's open-source, MIT-licensed, and ready to be your new favorite UI friend. Whether you're building dashboards, music players, data visualizations, or just a very fancy volume knob, Slider RS has your back.

> Ferris himself said:
> _"This slider is smoother than a freshly compiled release build."_

## 💬 Join the Party

Got ideas? Found a bug? Want to show off your slick Slider RS-powered app?

> Come hang out with us on [Discord](https://discord.gg/b5JbvHW5nv).

We're a friendly bunch of Rustaceans, building cool things for the web.

## 🚀 Final Thoughts

If you're building Rust-based web apps, **Slider RS** is the slider component you didn't know you needed, but now you do. It's precise. It's pretty. It's accessible. And it's yours.

> **We are Open SASS, babe!**.

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋
