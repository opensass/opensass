> Welcome 👋!

So I came across the [**Frontend Challenge: June Celebrations (CSS Art)**](https://dev.to/challenges/frontend-2025-06-04) on [dev.to](https://dev.to), and I thought: _"Hey, what if I build a handy dandy crate for our gay friends that they can slap onto their rusty websites?"_ This way, I learn a bit more about CSS, make something useful, and give Ferris the crab 🦀 a chance to finally come out of the shell.

And so, **Pride RS** was born.

## 🏳️‍🌈 What Is Pride RS?

**Pride RS** is a drop-in, customizable rusty component for rendering LGBTQ+ pride flags directly in your Rust frontend. From celebrating Pride Month, to adding some rainbow love to your app, or just want a vertical NonBinary flag for reasons between you and your browser, **Pride RS** makes it extremely easy.

Flags are rendered using [`flex-direction`](https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction), composed of CSS-powered color stripes that flex in all the right directions: **horizontally** or **vertically**. No SVGs. No dependencies on assets. Just pure HTML and CSS, generated at runtime.

And yes, Ferris is now canonically queer. You're welcome 🦀🏳️‍🌈.

![queer ferris](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/c5d9zu5u9igr5aal5vy2.webp)

## ⚙️ Under the Hood

Each flag is defined in a build-time-generated hashmap using the magic of [`phf`](https://docs.rs/phf), which means **zero runtime overhead**, and fast constant lookups. The following is how one of the configurations looks:

```rust
pub static FLAG_CONFIGURATIONS: phf::Map<&'static str, FlagConfig> = phf_map! {
    "Rainbow" => FlagConfig {
        colors: &["#e40303", "#ff8c00", "#ffed00", "#008018", "#0066ff", "#732982"],
        direction: Direction::Horizontal,
        name: "Pride Rainbow Flag",
        description: "The original rainbow pride flag representing LGBTQ+ community",
    },
    "Transgender" => FlagConfig {
        colors: &["#5bcffa", "#f5abb9", "#ffffff", "#f5abb9", "#5bcffa"],
        direction: Direction::Horizontal,
        name: "Transgender Flag",
        description: "Flag representing transgender community with light blue, pink, and white stripes",
    },
    // ... more flags!
};
```

Each flag can be rendered either **horizontally** (`flex-direction: column`) or **vertically** (`flex-direction: row`). You control the direction, size, and style directly via props.

We've got a solid collection of flags (sourced from [Wikipedia](https://en.wikipedia.org/wiki/Pride_flag) with love and hex codes):

```rust
#[derive(
    EnumString, EnumIter, AsRefStr, Display, Debug, Eq, PartialEq, Hash, Clone, Copy, Default,
)]
pub enum Type {
    #[default]
    Rainbow,
    Transgender,
    Bisexual,
    Lesbian,
    Pansexual,
    Asexual,
    NonBinary,
    Aromantic,
    Demisexual,
    Genderfluid,
    Agender,
    Polysexual,
    Omnisexual,
    Demiromantic,
    Graysexual,
}
```

Got a flag not listed? PRs welcome. Future updates will support more **complex shapes** too: think chevrons, triangles. Geometry is gay now.

You can tweak everything:

### Flag Sizes

```rust
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}
```

### Direction (Layout)

```rust
#[derive(EnumString, EnumIter, AsRefStr, Display, Debug, Clone, Copy, Default, PartialEq)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}
```

### Style Everything

The `Flag` and `FlagSection` components give you full control over:

- Stripe styling
- Container layout
- ARIA accessibility
- Tooltip behavior
- Custom CSS classes

Everything is built with **accessibility-first** principles: screen-reader labels, keyboard operability, and polite announcements for empty sections.

## 🧰 Getting Started with Yew

If you're already cozy with **Yew**, using Pride RS is pretty straightforward, i mean gayforward ;-):

### Step 1: Add the Crate

```sh
cargo add pride-rs --features=yew
```

### Step 2: Use the Component

```rust
use yew::prelude::*;
use pride_rs::yew::{FlagSection, Flag};
use pride_rs::{Size, Type};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Flag r#type={Type::Bisexual} />
            <Flag size={Size::Large} />
            <FlagSection
                title={"Pride Flags".to_string()}
                flags={vec![Type::Rainbow, Type::Transgender, Type::NonBinary]}
                id="pride"
            />
        </>
    }
}
```

That's it. You're rendering flags like a boss.

## 🧑‍🎨 Real Use Cases

- 🎉 **Celebrate Pride Month (e.g. This Dev Challenge)** without hand-rolling rainbow divs.
- 🧪 **Build inclusive UI demos** for Rust-based component libraries.
- 📚 **Educate with pride**: show flag tooltips and screen-reader descriptions.
- 🏳️‍⚧️ **Trans visibility?** Add the Transgender flag to your footer.

Want a whole grid of flags grouped under a section title?

```rust
<FlagSection
    title={"Non-Cis Energy"}
    flags={vec![Type::Agender, Type::Genderfluid, Type::Transgender]}
    id="non-cis"
/>
```

Yes, even your flag containers can slay.

Just imagine Ferris the crab walking across a NonBinary flag, wearing a tiny hat and waving a trans-colored claw. That's the vibe we're channeling.

```rust
// Rust + CSS + Queer joy = Pride RS 🏳️‍🌈🦀
```

## 🛠️ What's Next?

- 🌀 More complex flags (with shapes!)
- 🌍 International pride flags
- 🧩 SSR compatibility (if that's your thing)

## 🧑‍⚖️ For Judges

If you'd like to test this project locally, you can do so using either [**Dioxus**](http://github.com/opensass/pride-rs/tree/main/examples/dioxus) or [**Yew**](https://github.com/opensass/pride-rs/tree/main/examples/yew). Please refer to the README files for detailed instructions on how to run it locally.

## 💬 Final Thoughts

If you're building a Rust-based web UI and want to include a splash of **queer pride**, accessibility, and joy, **Pride RS** is your new best friend.

- ✅ Built with Rust

- ✅ Powered by CSS flex

- ✅ Fully accessible

- ✅ Entirely customizable

- ✅ Backed by Ferris's rainbow crab energy

> Add it. Ship it. Celebrate it 🏳️‍🌈🦀.

And hey, if you made it this far, consider [joining the Open SASS Discord](https://discord.gg/b5JbvHW5nv). We've got a dedicated channel for the queer rusty web, one div at a time.

![discord](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/8a7r5iibqyvsz4pf1tql.png)

Till next time: _Keep Rustin', keep Pride'n_ 💖
