> Bien le bonjour 👋!

Every WASM app eventually needs an avatar. Profile pictures, user cards, comment threads, they're everywhere. And every time, developers end up rolling the same `<img>` tag with some fallback logic, hardcoded border-radius, and a prayer that the CDN doesn't return a 404.

That stops today. We're shipping [**Avatar RS**](https://github.com/opensass/avatar).

## What Is Avatar RS?

**Avatar RS** is a fully-featured avatar component for **Yew**, **Dioxus**, and **Leptos**. It handles image loading, fallback text/icons, overflow groups, color themes, size variants, accessible markup, the whole thing. Built on top of [**Image RS**](https://github.com/opensass/image-rs), which means you get smart lazy loading, blur-up placeholders, and responsive layouts for free.

![one does not simply roll their own avatar component](assets/images/meme-1.jpeg)

Think of it as the avatar component you'd build if you had infinite time and zero deadlines. We had neither, but here we are anyway.

## The Stack Under the Hood

**Avatar RS** wraps **Image RS**, our WASM-native image primitive, for the actual `<img>` element. This is not a trivial detail.

Image RS uses [Intersection Observer API](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API) for lazy loading, supports `srcset`/`sizes` for responsive images, and fires clean `on_load` / `on_error` callbacks. All of that comes for free inside every `Image` you render. The avatar layer only adds **context management** and **state coordination** on top.

## Core Components

### `Avatar`

The container. Wraps `Image` and `Fallback`, provides the shared loading context, and inherits size/color/variant from a surrounding `Group` if present.

```rust
use avatar::yew::{Avatar, Image, Fallback};
use avatar::Color;
use yew::prelude::*;

#[function_component(MyProfile)]
pub fn my_profile() -> Html {
    html! {
        <Avatar aria_label="Ferris Prophet">
            <Image
                src="https://i.pravatar.cc/300"
                alt="Ferris Prophet"
            />
            <Fallback color={Some(Color::Accent)}>{"JD"}</Fallback>
        </Avatar>
    }
}
```

The fallback shows while the image loads, hides once it lands.

### `Fallback`

Text initials, an icon, or anything you want rendered when the image hasn't loaded yet (or doesn't exist at all). Supports a `delay_ms` prop to intentionally delay reveal, useful when your image usually loads fast and you don't want the fallback to flash briefly then disappear.

```rust
// Fallback shows immediately:
<Fallback color={Some(Color::Success)}>{"MH"}</Fallback>

// Fallback waits 200ms before appearing (prevents flicker on fast connections):
<Fallback delay_ms={200} color={Some(Color::Accent)}>{"JD"}</Fallback>
```

Small prop, big UX difference.

### `Group`

Stack multiple avatars with automatic overlap, configurable max count, and an overflow badge showing how many more there are.

```rust
use avatar::yew::{Avatar, Group, Count, Image, Fallback};
use avatar::{Color, Size, Variant};
use yew::prelude::*;

#[function_component(TeamStack)]
pub fn team_stack() -> Html {
    html! {
        <Group
            size={Size::Lg}
            max={Some(3)}
            total={Some(6)}
            aria_label="Project team"
        >
            <Avatar aria_label="Member 1">
                <Image src="https://i.pravatar.cc/150?u=m1" alt="Member 1" />
                <Fallback color={Some(Color::Accent)}>{"M1"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 2">
                <Image src="https://i.pravatar.cc/150?u=m2" alt="Member 2" />
                <Fallback color={Some(Color::Success)}>{"M2"}</Fallback>
            </Avatar>
            <Avatar aria_label="Member 3">
                <Image src="https://i.pravatar.cc/150?u=m3" alt="Member 3" />
                <Fallback color={Some(Color::Warning)}>{"M3"}</Fallback>
            </Avatar>
            <Count
                color={Some(Color::Danger)}
                variant={Some(Variant::Soft)}
                aria_label="3 more members"
            />
        </Group>
    }
}
```

Pass `max={Some(3)}` and `total={Some(6)}`, and `Count` automatically shows `+3`. The math is done for you. The CSS overlap is done for you. You just write components.

![math lady](assets/images/meme-2.jpeg)

## The `overflow_count` Context

This is the part that required actual thought.

`Group` computes `overflow_count = total - max` and pushes it into context. `Count` reads it from context when its own `count` prop is zero (the default). This means the badge doesn't need to know anything explicit, it just reads the group state.

```rust
// In Group:
let overflow_count = total.saturating_sub(max);
ctx_provider.provide(&GroupContext { overflow_count, .. });

// In Count:
let effective_count = if count == 0 { ctx_overflow } else { count };
// Renders: "+{effective_count}"
```

One source of truth.

## Framework Parity

All three frameworks ship the same API surface:

| Feature                  | Yew | Dioxus | Leptos |
| ------------------------ | --- | ------ | ------ |
| `Avatar`                 | ✅  | ✅     | ✅     |
| `Image`            | ✅  | ✅     | ✅     |
| `Fallback`         | ✅  | ✅     | ✅     |
| `Group`            | ✅  | ✅     | ✅     |
| `Count`       | ✅  | ✅     | ✅     |
| `total` + `max` overflow | ✅  | ✅     | ✅     |
| Context propagation      | ✅  | ✅     | ✅     |
| `delay_ms` on fallback   | ✅  | ✅     | ✅     |
| Image RS integration     | ✅  | ✅     | ✅     |

The Dioxus and Leptos implementation uses `use_context_provider` / `use_context` idioms native to each framework, but the behavior is identical. If you've used it in Yew, you already know how to use it in Leptos.

## Soft Refresh & The Browser Cache Fix

Here's one that caught us off guard. In Yew, images rendered via `Image` would sometimes fail to appear after a soft page refresh. Hard refresh? Fine. Soft? Gone.

The culprit: when a browser serves an image from cache, the `load` event fires _before_ Yew has had a chance to attach the listener. The effect sees a "loading" state, the image is already done, and the component never flips to "loaded".

The fix was straightforward once we knew what to look for. We inject a `node_ref` into the underlying `<img>` element and check `img.complete` on mount:

```rust
use_effect_with(props.src, move |src| {
    if !src.is_empty() {
        let already_loaded = node_ref
            .cast::<web_sys::HtmlImageElement>()
            .map(|img| img.complete())
            .unwrap_or(false);

        if already_loaded {
            status.set(ImageLoadingStatus::Loaded);
        } else {
            status.set(ImageLoadingStatus::Loading);
        }
    }
    || ()
});
```

This required adding `web-sys` with the `HtmlImageElement` feature to the `yew` feature gate in `Cargo.toml`. A small addition, but it closes a genuinely annoying browser quirk.

![it works on my machine](assets/images/meme-6.jpeg)

Dioxus and Leptos did not have this issue, their reactive systems handle DOM updates differently and don't suffer from this ordering problem.

## Colors, Variants, Sizes

Everything inherits downward through context. Set defaults at the group level, override them per-avatar if needed.

```rust
// Colors
Color::Default | Color::Accent | Color::Success | Color::Warning | Color::Danger | Color::Custom("...")

// Variants
Variant::Default  // solid fill
Variant::Soft     // muted, lower-contrast

// Sizes
Size::Xs | Size::Sm | Size::Md | Size::Lg | Size::Xl | Size::Xxl | Size::Custom("...")
```

`Color::Custom("...")` and `Size::Custom("...")` accept raw CSS strings, so if your design system uses `hsl(270 60% 50%)` or `3.5rem`, those work too.

## Quick Setup

```sh
# Yew
cargo add avatar --features=yew

# Dioxus
cargo add avatar --features=dio

# Leptos
cargo add avatar --features=lep
```

That's it. The styling is computed inline from the prop values.

## Accessibility

Every component exposes its ARIA props explicitly:

- `Avatar` renders `role="img"` with a required `aria_label`.
- `Image` manages `aria-hidden` based on loading state.
- `Fallback` is hidden from assistive tech once the image loads.
- `Group` gets an `aria_label` for the whole group.
- `Count` gets its own `aria_label` (defaults to `"Additional members"`).

Screen readers get a consistent, clear picture regardless of whether images loaded or not.

## What We Didn't Do

No auto-injection of count badges. No magic child-slicing that differs between frameworks. No runtime diffing of the children list to figure out how many to show. These patterns break in different ways across Yew, Dioxus, and Leptos, and they make the component harder to reason about.

Instead: **explicit `total` and `max` props**, **explicit `Count` placement**, **one context object** that everything reads from. It's a little more to type. It's a lot easier to debug.

## Final Thoughts

**Avatar RS** is exactly what it says on the tin: a production-ready avatar component for WASM apps.

It's backed by Image RS, which means you get all the image loading sophistication without reinventing it. The overflow logic is explicit and predictable. Framework parity is real, not aspirational.

> **We are Open SASS, babe!**

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Till next time 👋!
