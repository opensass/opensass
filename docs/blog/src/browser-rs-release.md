> Welcome 👋!

If you've ever tried building a polished, production-ready WASM UI in **Rust**, you know the drill: getting your app to _feel_ native often involves duct tape, a prayer to the compiler gods, and a suspicious amount of copy-pasted CSS. And when it comes to wrapping content in a clean, browser-like frame? Forget it, until now.

That's why we're excited to announce [**Browser RS**](https://crates.io/crates/browser-rs), the **drop-in browser frame component** built natively for Rust. It's sleek, it's accessible, it's got style for days, and yes, it even plays nice with Tailwind. Whether you're embedding an iframe, a widget, or just want your app to look like it belongs inside a miniature Chrome tab, **Browser RS** delivers **maximum polish with minimum fuss**.

Let's dive in and see why Browser RS is the browser _inside_ your browser you didn't know you needed.

![bro do be vibin](https://media.tenor.com/dW17uHQlonMAAAAi/dance-anime.gif)

## 🌐 What Is Browser RS?

**Browser RS** is a **fully customizable browser-frame component** built specifically for **WASM Frameworks**, Rust's powerhouse frontend frameworks. It lets you **wrap any HTML content** inside a mock browser UI, complete with a header bar, address field, window controls, and optional custom buttons, all while giving you fine-grained control over styling, behavior, and accessibility. Basically, it's like giving your content a tuxedo and sending it to prom.

## 🚀 Why You'll Love Browser RS

Not all browser frames are created equal. Some are rigid. Others are pure CSS gimmicks. But Browser RS? It's got **substance _and_ style**. Here's what sets it apart:

- **Plug-and-Play Simplicity**: Add it to your project in seconds. No dark magic or manual wiring required.
- **Full Control**: Customize _everything_, from the close button to the ARIA labels.
- **Event-Driven**: Hook into user actions like close, maximize, and minimize. It's like window management, but without the OS.
- **Accessible by Default**: Screen reader support, keyboard nav, ARIA labels, your app's users (and auditors) will thank you.
- **Dark Mode Friendly**: Browser RS fits in with your Tailwind-based design system, including that moody, stylish dark mode.

Even Ferris the crab gave it claws up 🦀👍.

## 🔥 Getting Started with Yew

If you're already in the Yew ecosystem, integrating **Browser RS** is smoother than a fresh `cargo build`.

### Add the Crate

```bash
cargo add browser-rs --features=yew
```

### Import the Component

```rust
use yew::prelude::*;
use browser_rs::yew::BrowserFrame;
```

### Wrap Your Content

```rust
#[function_component(App)]
fn app() -> Html {
    let on_close = Callback::from(|_| log::info!("Closed like it's 2003 and your mom needs the phone line"));

    html! {
        <BrowserFrame url={"https://opensass.org".to_string()} on_close={on_close}>
            <p>{ "Here's some magical Yew-powered content." }</p>
        </BrowserFrame>
    }
}
```

## 🧩 Features That Matter

| Feature                                    | Why It's Awesome                                              |
| ------------------------------------------ | ------------------------------------------------------------- |
| `url` & `placeholder`                      | Looks real, works seamlessly, no `<iframe>` nightmares needed |
| `on_close` / `on_minimize` / `on_maximize` | Build dynamic, responsive UIs like a pro                      |
| `custom_buttons`                           | Add fun buttons, even a "Launch Ferris" rocket icon 🚀🦀      |
| `class`, `style`                           | Tailor every pixel to your liking                             |
| ARIA & keyboard support                    | Accessible by default, no extra effort required               |
| Size & Variant                             | Choose from small to full screen, minimal to rich             |

And let's not forget: it **works everywhere**. Editors, sandboxes, dashboards, iframes, whatever you're building, it makes it look like it belongs on a browser within a browser, which is _almost_ as cool as Inception.

## 🎨 Styled to Match

Want that clean, modern look without writing a single line of CSS? Just pass in your Tailwind classes:

```rust
<BrowserFrame
    url={"https://opensass.org".to_string()}
    class={"rounded-xl shadow-xl"}
    input_class={"bg-gray-200 text-gray-900"}
    container_class={"flex-1 mx-4"}
>
    <p>{ "Styled browser frame!" }</p>
</BrowserFrame>
```

Your designers will think you've leveled up overnight.

## 🎛️ Full Control via Props

Browser RS comes with **a buffet of props**, you can style, control, and wire up everything:

- `url`, `placeholder`, `read_only`
- `show_controls`, `show_address_bar`
- `on_url_change`, `on_close`, `on_minimize`, `on_maximize`
- `custom_buttons`
- Styling: `class`, `style`, `container_class`, `input_class`
- Accessibility: `aria_label`, `aria_describedby`
- Visual size & variant controls: `size`, `variant`
- Full control of internal buttons: `close_*`, `maximize_*`, `share_*`, etc.

If you want _boring_, you've come to the wrong crate.

## 🛠️ Real-World Use Cases

Here's where **Browser RS** really shines:

- **Interactive Demos**: Want to showcase your app with embedded code or tools? Wrap it in a `BrowserFrame`.
- **Developer Tools**: Give your in-app dev tools a visual shell.
- **Mini-browser Widgets**: Use it for previews, sandboxed environments, or even static content.
- **Figma-like Interfaces**: Great for app builders and drag-n-drop UIs.

And yes, you can make it look like Safari, Firefox, or even Netscape Navigator (if you're _that_ nostalgic).

## 💬 Final Thoughts

If you're serious about building delightful web apps with **Yew** and **WASM**, Browser RS is a no-brainer.

- ✅ It makes your app look polished without extra design work.

- ✅ It provides real interactivity through events and dynamic props.

- ✅ It keeps your app accessible, responsive, and customizable.

- ✅ It's built by Rust devs, _for_ Rust devs.

Ferris didn't have a browser frame when Rust was born, but if he did, **he'd use this one**, and probably paint it red 🦀.

> At Open SASS, we're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋!
