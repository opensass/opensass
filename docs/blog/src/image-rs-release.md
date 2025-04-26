> Welcome 👋!

When it comes to building lightning-fast, highly interactive WASM apps, the devil's in the details, and images are a **huge** part of that. Traditionally, working with images in Yew, Dioxus, or Leptos has been... well, clunky. Limited flexibility, poor accessibility, bad loading UX, you name it.

That's why we're excited to announce [**Image RS**](https://github.com/opensass/image-rs), a **next-gen** image component designed from the ground up for **WASM-based** frameworks. Not just another helper, but a fully-loaded, highly-optimized solution that **removes** the bottlenecks we as developers usually run into when dealing with images.

Let's explore why **Image RS** doesn't just meet your needs, it **exceeds** them 🚀.

## 📸 What Is Image RS?

**Image RS** is a **highly optimized**, **feature-rich** image component crafted specifically for the new wave of **Rust-powered frontend frameworks** like **Yew**, **Dioxus**, and **Leptos**.

It's **fast** (thanks to smart lazy loading and `IntersectionObserver`), **flexible** (supporting a range of layouts and styles), and **accessible** (full ARIA support baked in). Basically, it's everything you ever wanted from an image component, but for the WASM era.

## 🚀 Why You'll Love Image RS

Other image components in the wild are either too basic, too bloated, or too locked into specific frameworks. **Image RS** gives you **the best of all worlds**, right out of the box:

- **Performance Obsessed**: Lazy loading, blur placeholders, async decoding, all built-in and tuned for maximum speed.
- **Layout Freedom**: Fixed, Intrinsic, Responsive, Fill, Stretch, ScaleDown, pick whatever fits your design best.
- **Fully Accessible**: ARIA labels, roles, dynamic states, it's all handled properly.
- **Interactive Events**: Get callbacks for success or failure, build smarter, more resilient apps.
- **Visual Candy**: Blur-up placeholders, fallback images, quality settings, gorgeous UX without extra work.

## 🔥 Quick Yew Setup

Getting started? It's a breeze.

Add `image-rs` to your project:

```sh
cargo add image-rs --features=yew
```

Import it in your app:

```rust
use yew::prelude::*;
use image_rs::yew::Image;
use image_rs::Layout;
```

Use it:

```rust
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Image
            src="/images/photo.jpg"
            alt="A beautiful view"
            width="800"
            height="600"
            layout={Layout::Responsive}
        />
    }
}
```

**That's it.** Blazing fast images, minimal setup.

## 🧩 Features That Set It Apart

| Feature            | What It Means for You                         |
| ------------------ | --------------------------------------------- |
| Lazy Loading       | Only loads images when needed, faster pages   |
| Blur Placeholder   | Smooth loading experience with visual cues    |
| ARIA Accessibility | Inclusive, accessible apps without extra code |
| Layouts            | Responsive, Fixed, Stretch, and more          |
| Event Hooks        | Handle load success, failure, and retries     |
| Fallback Images    | No more broken image icons                    |
| Custom Styling     | Tailor it exactly to your needs               |

## ⚙️ Full Control with Props

Need more fine-grained control?  
**Image RS** exposes a full suite of props, including:

- `src`, `alt`, `width`, `height`
- `fallback_src`, `placeholder`, `priority`
- `layout`, `object_fit`, `object_position`
- `on_loading_complete`, `on_error`
- Accessibility props like `aria_labelledby`, `aria_hidden`, `aria_current`, etc.

**Customization?** ✅

**Flexibility?** ✅

**Developer happiness?** ✅

## 🎯 Real-World Examples

Want to see it in action? Here are some real snippets you can find on [the live demo](https://image-rs.netlify.app):

```rust
// Simple image
<Image src="https://placehold.co/300x200" alt="Basic Image" />

// Responsive layout
<Image src="https://placehold.co/600x400" alt="Responsive Layout" layout={Layout::Responsive} width="600" height="400" />

// Blur-up placeholder
<Image src="https://placehold.co/600x400" alt="Blurred Image" layout={Layout::Responsive} width="600" height="400" placeholder="blur" blur_data_url="https://placehold.co/10x10" />

// Priority loading
<Image src="https://placehold.co/400x300" alt="Priority Image" priority=true />

// Custom object fit and position
<Image
    src="https://placehold.co/600x400"
    alt="Cover Fit"
    layout={Layout::Responsive}
    width="600"
    height="400"
    object_fit={ObjectFit::Cover}
    object_position={Position::TopRight}
/>
```

Yes, you can do **a lot** with it, and still keep your codebase super clean.

## 💡 Final Thoughts

When you're building modern, performant WASM apps, you need your images to keep up, not slow you down.

**Image RS** is lightweight, battle-tested, accessibility-first, and packed with features that make real-world web development easier, smoother, and more scalable.

If you're ready to stop fighting your image components and start shipping polished, professional apps, **Image RS** is here 🖼️.

> **We are Open SASS, babe!**.

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋!
