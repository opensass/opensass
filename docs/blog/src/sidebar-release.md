> **Welcome 👋!**

If you've ever built a **Rust-powered frontend**, you've probably felt this pain: _"How the heck do I make a sidebar that doesn't look like it crawled out of the year 2001?"_

We get it. Collapsibles, user profiles, nested menus, custom styles... you need a PhD in CSS gymnastics and a minor in JavaScript contortionism just to get a halfway-decent result.

**That's so over, and we are so back!**

Introducing [**Sidebar**](https://crates.io/crates/sidebar), the **ultimate plug-and-play sidebar component** for **Yew**, **Dioxus**, and **Leptos**. It's clean, fast, accessible, and stupidly customizable. And yep, **it just works**.

Let's open it up 🧰.

![vibin](https://github.com/user-attachments/assets/c583f235-17c3-496e-9689-35db043ebff3)

## 🗃️ What Is Sidebar?

**Sidebar** is a **fully modular**, **highly customizable** sidebar component built specifically for **WASM UI frameworks**. It's designed to make your app feel like a well-polished dashboard without you rage-quitting halfway through.

Here's what it gives you out of the box:

- ✅ Collapsible layout with smooth toggle.
- ✅ Nested submenus (with disclosure arrows!).
- ✅ Badges, icons, and dynamic styling.
- ✅ Built-in profile section with avatar + logout.
- ✅ Full control via `props`, `class`, and `style`.
- ✅ Idiomatic Rust state management with `UseStateHandle`.
- ✅ Designed with **accessibility** in mind.

Whether you're building a SaaS dashboard, admin panel, or a hobby side project, **Sidebar** will save you hours of UI pain.

## ⚡️ Why You'll Love It

Let's break it down:

| Feature                     | Why it's awesome                                                                       |
| --------------------------- | -------------------------------------------------------------------------------------- |
| **Modular Components**      | Use just what you need: `Sidebar`, `Menu`, `MenuItem`, `Submenu`, `Profile`, or `Logo` |
| **Composability**           | Nest menus, add icons, toggle visibility—anything goes                                 |
| **Customization**           | Every pixel is yours to style with `props`, `class`, or `style`                        |
| **Built for Rust**          | Idiomatic use of `Callback`, `UseStateHandle`, and `Html`                              |
| **Responsive + Accessible** | Auto-collapses on mobile; plays nice with screen readers                               |

And the cherry on top? **It's built by Open SASS**, a community on a mission to make Rust web development suck less.

## 🚀 Quick Start with Yew

Getting up and running is easier than convincing Ferris to wear a bow tie.

```sh
cargo add sidebar --features=yew
```

Then plug it into your app:

```rust
use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::menu::Menu;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::submenu::Submenu;

#[function_component(App)]
pub fn app() -> Html {
    let selected = use_state(|| String::from("Dashboard"));

    html! {
        <Sidebar
            user_name="Ferris"
            designation="Admin"
            user_img="/assets/logo.webp"
            on_logout={Callback::from(|_| log::info!("Logout!"))}
            logo_img_url="/logo.svg"
            logo_href="/"
        >
            <Menu sub_heading="Main">
                <MenuItem
                    label="Dashboard"
                    href="/dashboard"
                    icon_html={html! {<span>{ "📊" }</span>}}
                    selected={selected.clone()}
                />
                <MenuItem
                    label="Settings"
                    href="/settings"
                    icon_html={html! {<span>{ "⚙️" }</span>}}
                    selected={selected.clone()}
                />
                <Submenu title="More" icon_html={html! {<span>{ "➕" }</span>}}>
                    <MenuItem
                        label="Reports"
                        href="/reports"
                        icon_html={html! {<span>{ "📁" }</span>}}
                        selected={selected.clone()}
                    />
                </Submenu>
            </Menu>
        </Sidebar>
    }
}
```

You now have a sleek, collapsible sidebar with dynamic state and profile logout handling 🎉.

## 🔍 What You Can Customize (Spoiler: Everything)

Here's a taste of what you can tweak:

### 🧱 `Sidebar` Props

- `style`, `class`: Tweak the outer container to match your brand.
- `user_name`, `designation`, `user_img`: Show off your user's profile.
- `logo_img_url`, `logo_href`: Your logo, your link.
- `on_logout`: Handle logout like a pro (or a script kiddie, no judgment).

### 📋 `Menu` & `MenuItem`

- `sub_heading`: Optional section labels.
- `href`: Navigation links.
- `icon_html`: Drop in custom SVGs or emojis (we love emojis, btw).
- `selected`: Use shared `UseStateHandle<String>` for tracking.

### 🔽 `Submenu`

- Supports deep nesting.
- Expand/collapse toggle.
- Styled with arrows because UI polish.

### 🙋 `Profile`

- Automatically hides when sidebar is collapsed.
- Includes a logout button.
- Fully styleable.

All of this controlled with easy props like `style`, `class`, and `Callback`.

Below 768px? **Sidebar auto-collapses** into a sleek mobile-friendly mode. No JavaScript hacks. Just Rust and vibes.

## 💡 Final Thoughts

**Stop wasting time rebuilding sidebars from scratch**. Sidebar gives you everything you need, and nothing you don't, to ship polished, responsive, and composable UI components in your Rust WASM apps.

So go ahead. Drop it in your app. Customize it. Hack it. Make it yours.

> **We are Open SASS, babe!**.

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋!
