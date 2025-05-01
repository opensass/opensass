> **Welcome 👋!**

If you've ever built a frontend app in **Rust**, you've probably realized one thing real quick: **making a responsive, polished, and customizable navbar is surprisingly annoying**. Between layout quirks, dropdown handling, mega menu hacks, and mobile toggles... things get out of hand fast.

That's why we're excited to introduce [**Navbar**](https://crates.io/crates/navbar), a **fully-featured**, totally plug-and-play navigation component for **WASM** frameworks like Yew, Dioxus, and Leptos.

It's sleek, smart, responsive, and **just works**, whether you're building a startup dashboard or the next social media empire.

Let's take a bite into everything **Navbar** has to offer 🍟.

## 🚀 What Is Navbar?

**Navbar** is a fully-configurable component built for the **Yew** framework that gives you:

- A responsive layout with a mobile hamburger menu 🍔.
- Support for **mega menus**, **dropdowns**, and **call-to-action buttons**.
- Full **accessibility**, **custom styling**, and **state management** baked in.
- Easy integration with `state`, `Callback`, and other hooks.

No JavaScript duct tape. No style hacks. Just clean, idiomatic Rust and WebAssembly.

## ⚡️ Why You'll Love It

Most navbars feel like a chore. You paste in 200 lines of spaghetti HTML, try to wire up `onclick` logic, and still end up with a weird dropdown. **Navbar** takes care of **all that**. Out of the box, you get:

- ✅ Mobile-first responsiveness.
- ✅ Easy-to-add search bar.
- ✅ Mega menu support for large content.
- ✅ Profile dropdown menus.
- ✅ Fully styleable with `class` and `style` props.
- ✅ Event handling for resize, clicks, and toggles.
- ✅ 100% declarative API.

## 🧰 Quick Setup in Yew

Getting started is as easy as eating fries:

```sh
cargo add navbar --features=yew
```

Then import and use it in your app:

```rust
use yew::prelude::*;
use navbar::yew::{Navbar, Menu};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Navbar
            logo_src="/assets/logo.svg"
            logo_alt="My App"
            menus={vec![
                Menu { id: 1, link: "/", name: "Dashboard", icon_start: None, icon_end: None },
                Menu { id: 2, link: "/reports", name: "Reports", icon_start: None, icon_end: None }
            ]}
            button_text="Sign Up"
            button_href="/signup"
            show_search=true
            show_mega_menu=true
            show_profile_menu=true
        />
    }
}
```

Boom 💥. You've got a fully interactive, responsive navbar.

## 🧩 Feature Breakdown

Let's talk toppings. Here's what comes with the Navbar combo meal:

### 🧭 Logo & Menu

Display your logo with alt text and optional link, and set up top-level navigation with `Menu` items.

### 🔍 Search Input

Need a search bar? Just flip `show_search = true` and you're done. You can even manage the input state with `use_state`.

### 🍕 Mega Menu

Have a ton of links? Turn on `show_mega_menu` and pass in `mega_menu_items`. It works with hover and auto-closes cleanly.

### 🙋 Profile Menu

Want a profile dropdown? Set `show_profile_menu = true` and provide `dropdown_items` + a profile image URL.

### 📱 Mobile Support

Below 768px, the navbar switches to mobile mode with a hamburger toggle. Menu opens as a vertical drawer, no extra config needed.

### 🎨 Full Styling Control

Every single part of the navbar, logo, menu, button, dropdown, etc., can be styled via `class` and `style` props. Go wild.

## 💻 Developer Features

Here's where it gets extra juicy:

| Feature                  | What It Means for You                                   |
| ------------------------ | ------------------------------------------------------- |
| `use_state` & `Callback` | Fully interactive toggles without boilerplate           |
| Auto-close dropdowns     | Click outside? Menus close automatically 🙌             |
| Resize listener          | Reacts to screen size changes, real-time responsiveness |
| Conditional rendering    | Don't need a feature? Just don't enable the prop!       |
| Accessibility            | ARIA-compliant, screen-reader friendly                  |
| Custom events            | Hook into `oninput`, `onclick`, and more                |

All of this wrapped in a clean component API that's easy to use.

## 🎛️ Full Props Reference

Too many props to list here, but here's a taste:

- `logo_src`, `logo_alt`, `logo_link`
- `menus`, `show_search`, `search_placeholder`
- `button_text`, `button_href`, `button_target`
- `show_mega_menu`, `mega_menu_items`
- `show_profile_menu`, `dropdown_items`, `profile_image_url`
- `navbar_style`, `navbar_class`, `container_style`, `menu_item_class`, etc.

Check the [docs](https://docs.rs/navbar) or [source](https://github.com/opensass/navbar/blob/ea46fa78290a311b16a91525a9fad2f88fc05e5f/src/yew.rs#L66) for all props and default values.

## 🧪 Real-World Scenarios

Let's say you want:

### A clean marketing navbar:

```rust
<Navbar
    logo_src="/logo.svg"
    logo_alt="My Brand"
    menus={vec![
        Menu { id: 1, link: "/", name: "Home", icon_start: None, icon_end: None },
        Menu { id: 2, link: "/features", name: "Features", icon_start: None, icon_end: None },
    ]}
    button_text="Start Free Trial"
    button_href="/signup"
    show_search=false
/>
```

### A complex app dashboard navbar:

```rust
<Navbar
    logo_src="/logo.svg"
    logo_alt="Dashboard"
    menus={...}
    show_search=true
    show_profile_menu=true
    dropdown_items={vec![
        DropdownItem { text: "Settings", link: "/settings" },
        DropdownItem { text: "Logout", link: "/logout" }
    ]}
    profile_image_url="/user.png"
/>
```

### A styled, custom-themed navbar:

```rust
<Navbar
    navbar_style="background: #0d1117; color: white;"
    button_style="background: #238636; border-radius: 4px;"
    search_input_style="border-radius: 4px; padding: 8px;"
    ...
/>
```

## 💡 Final Thoughts

Your navbar is your app's **first impression**, don't settle for a janky, fragile mess of divs. With **Navbar**, you can build a beautiful, responsive, fully accessible header in **minutes**, not hours. Whether it's a static marketing site or a complex SPA, it scales with you. Fast, customizable, idiomatic, and built **for the modern WASM dev**.

> **We are Open SASS, babe!**.

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋!
