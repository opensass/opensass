> Welcome 👋!

So I came across the [**Frontend Challenge: Comfort Food Edition**](https://dev.to/devteam/join-our-latest-frontend-challenge-comfort-food-edition-28a0) on [dev.to](https://dev.to), and I thought: _"Hey, what if I build a Rust crate that lets you render SVG sushi directly in your WASM frontend? How hard can it be?"_

_(Famous last words. The borrow checker immediately disagreed.)_

![me starting this project](https://preview.redd.it/i-just-need-to-finish-this-project-v0-tsxtxwf1q01b1.jpg?width=640&crop=smart&auto=webp&s=8d898d29a82af3fd38a0647ae56f5ac5aa43c8e1)

But spoiler: _we made it_. And so, **Sushi RS** was born.

## 🍣 Why Sushi, Specifically?

Now, you might be wondering: _"Of all comfort foods: pizza, ramen, mac and cheese, a warm bowl of existential dread, why sushi?"_

Fair question.

The answer is personal, and a little dark, in the way that good sushi always has a little bit of wasabi hiding underneath.

I wrote a long-form post called [**Who Am I?**](https://wiseai.dev/blogs/who-am-i) that goes deep into my life, my mental health journey, and the concept of what I called **sushicide 🍣**, a metaphor I use for sushicidal ideation that I've carried for years. It's a real thing I struggle with. It's also my way of talking about it without the conversation immediately shutting down.

> 🚨 _"Sushicide 🍣 is a serious problem that many among us struggle with. Software engineering is a heartless and lonely profession, and we should be more open about this topic so that we don't lose another innocent human soul."_

So when the challenge said **"comfort food"**, I didn't just reach for sushi because it's delicious. I reached for it because it _means_ something to me, wrapped up in layers, held together at the seams, sometimes a little raw in the middle.

Building **Sushi RS** was, in a deeply weird developer-brain kind of way, a form of therapy. Instead of drowning in the darkness, I made the darkness into something beautiful, accessible, and open-source. I rendered it in SVG. I wrapped it in nori. I gave it ARIA labels so screen readers could describe it.

_If that's not turning pain into art, I don't know what is._

![this is fine](https://c.tenor.com/MYZgsN2TDJAAAAAC/tenor.gif)

If you're going through something heavy right now, I see you. Go read [that post](https://wiseai.dev/blogs/who-am-i). And then come back here, because we're about to have fun with Rust.

## 🍣 What Is Sushi RS?

**Sushi RS** is a cross-framework SVG sushi rendering component library for Rust + WASM frontends. It supports **Yew**, **Dioxus**, and **Leptos** out of the box, and lets you render beautifully detailed, fully accessible sushi SVGs with a single component.

Yes, we said _beautifully detailed_. Salmon gradients. Nori weave patterns. Red caviar rim decorations. Rice grain textures. This is not a placeholder circle with a fish emoji slapped on it.

![gordon ramsay approves](https://media2.giphy.com/media/v1.Y2lkPTc5MGI3NjExcDA0OHhuOTd3Ym92Z25jYWdiejJzcXN0dmt5Z244bzA1eXFqbXRzdCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/3oFzlX9khlRIev1E2Y/giphy.gif)

## 🎯 The Challenge

The dev.to [**Comfort Food Challenge**](https://dev.to/devteam/join-our-latest-frontend-challenge-comfort-food-edition-28a0) asked us to build something related to comfort food. We could have built a pizza component. Or a burger one. Or just flexboxed some taco emojis and called it a day.

Instead, we chose _sushi_.

Because nothing says "comfort" like watching the Rust borrow checker scream at you for 3 hours while you try to interpolate SVG hex colors inside a `format!()` macro.

> _Fun fact_: you can't put `#3a6a20` (nori green) inside a `r#"..."#` raw string literal, because `"#` terminates the raw string. So our entire SVG rendering engine uses **single-quote string templates** with a `.replace('\'', '"')` pass at the end. Comfort food? Yes. Comfortable code? _Debatable._

![typing furiously](https://c.tenor.com/tOoW-TVnlhUAAAAC/tenor.gif)

## 🧬 Under the Hood

The core idea is a **framework-agnostic SVG string engine** living in `src/svg.rs`. Each shape renderer returns a plain `String` of SVG markup, which every framework embeds via its "inner HTML" escape hatch:

| Framework | Escape hatch                     |
| --------- | -------------------------------- |
| Yew       | `Html::from_html_unchecked(...)` |
| Dioxus    | `dangerous_inner_html: "{svg}"`  |
| Leptos    | `inner_html={svg}`               |

The word **"dangerous"** is right there in the Dioxus API name and we used it anyway. No fear.

Here's how the shared `SushiData` struct that drives it all looks:

```rust
pub struct SushiData {
    pub id: String,
    pub name: String,
    pub shape: SushiShape,      // Circular, Square, Triangular, Oval
    pub state: SushiState,      // Rolled or Exploded 💥
    pub view: SushiView,        // Top (bird's-eye) or Front (isometric)
    pub ingredients: Vec<Ingredient>,
    pub outer_sheet: SushiOuterSheet,  // nori color + thickness
    pub size: SushiSize,
    pub rice_color: Option<String>,
    pub top_edge: Option<TopEdgeDecoration>,
    pub description: Option<String>,
}
```

Write once. Render everywhere. The Rust dream. 🦀

## 🍱 The Shapes

Because one sushi shape would never be enough:

```rust
pub enum SushiShape {
    Circular,    // 🟤 Classic maki roll
    Square,      // 🟫 Tamago / oshi-style
    Triangular,  // 🔺 Onigiri vibes
    Oval,        // 🥚 Nigiri hand-pressed
}
```

And because we're overachievers who clearly have no regard for weekends:

```rust
pub enum SushiView {
    Top,    // 🐦‍⬛ Bird's-eye: you're looking DOWN at the roll
    Front,  // 👀 Isometric: you're eyeballing the cross-section
}
```

And when a roll just wants to be free:

```rust
pub enum SushiState {
    Rolled,    // 🍣 Perfectly assembled
    Exploded,  // 💥 Deconstructed (every ingredient side-by-side)
}
```

`SushiState::Exploded` ignores shape and view entirely and renders all layers laid out next to each other like a crime scene forensics exhibit. Very artsy.

![explosion gif](https://i.giphy.com/2rqEdFfkMzXmo.webp)

## 🐟 The Ingredients

Twelve ingredients, each with SVG gradients that actually look like the real thing:

```rust
pub enum Ingredient {
    Salmon,      // 🐟 Pinkish-orange gradient
    Tuna,        // 🐠 Deep red
    Avocado,     // 🥑 Green gradient
    Egg,         // 🥚 Yellow-gold (Tamago)
    RedCaviar,   // 🔴 Tiny red circles rendered individually
    BlackCaviar, // ⚫ Same but dark and fancy
    Cucumber,    // 🥒 Fresh green
    Crab,        // 🦀 Coral + white layers
    Shrimp,      // 🍤 Coral crescent
    Mango,       // 🥭 Warm orange tropical
    CreamCheese, // 🧀 Creamy white (Philadelphia roll represent)
    Wasabi,      // 🌿 That sharp green that regrets you immediately
}
```

Multiple ingredients? They get rendered as **equal pie-slice arcs** with individual `<title>` elements for screen readers. Accessible sushi. Because we believe in inclusive dining.

## ⚙️ Getting Started (Yew)

```sh
cargo add sushi-rs --features=yew
```

```rust
use yew::prelude::*;
use sushi_rs::yew::Sushi;
use sushi_rs::common::{
    Ingredient, SushiOuterSheet, SushiShape, SushiSize, NORI_COLOR,
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Sushi
            id="salmon-maki"
            name="Salmon Maki"
            shape={SushiShape::Circular}
            ingredients={vec![Ingredient::Salmon]}
            outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }}
            size={SushiSize { width: 140.0, height: 140.0 }}
        />
    }
}
```

**That's it.** You now have a fully accessible SVG salmon maki roll rendered by the fastest systems language on the planet. Ferris the crab has never been more culinarily accomplished. 🦀🍣

## 🎨 Top-Edge Decorations

Rolls can have rim decorations:

```rust
pub enum TopEdgeType {
    None,
    RedCaviar,   // 🔴 Ring of tiny red roe dots
    BlackCaviar, // ⚫ Dark premium roe ring
    Sesame,      // ✳️  Sesame seeds scattered on top
    Herbs,       // 🌿 Herb speckles
    Tobiko,      // 🟠 Bright orange flying fish roe
}
```

You can even set a custom color for the herb speckles. Yes, we went that far.

```rust
<Sushi
    id="fancy-roll"
    name="Red Caviar Roll"
    shape={SushiShape::Circular}
    ingredients={vec![Ingredient::RedCaviar]}
    outer_sheet={SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }}
    size={SushiSize { width: 140.0, height: 140.0 }}
    top_edge={Some(TopEdgeDecoration {
        edge_type: TopEdgeType::RedCaviar,
        color: None,
    })}
    rice_color={Some("#FFF9E8".to_string())}
    scale={1.1}
/>
```

The fact that `rice_color` caused a type inference ambiguity with Yew's prop system because of `Option<_>: IntoPropValue<Option<String>>` having two possible implementations is both hilarious and deeply unhinged. We fixed it. Don't worry about it. 😇

## 🖼️ SushiGallery

```rust
use sushi_rs::yew::SushiGallery;
use sushi_rs::common::default_sushi_gallery;

html! {
    <SushiGallery items={default_sushi_gallery()} />
}
```

One function call. Twelve sushi rolls. A whole ikebana-level arrangement on your screen. The `default_sushi_gallery()` function returns a `Vec<SushiData>` with pre-built sushi configurations so you can drop an entire sushi restaurant into your WASM app with one line of Rust.

Michelin star pending. 🌟

## 🧰 For the Dioxus People

We see you. RSX syntax incoming:

```rust
rsx! {
    Sushi {
        id: "tuna-nigiri",
        name: "Tuna Nigiri",
        shape: SushiShape::Oval,
        ingredients: vec![Ingredient::Tuna],
        outer_sheet: SushiOuterSheet {
            color: "transparent".to_string(),
            thickness: 0.0,
        },
        size: SushiSize { width: 150.0, height: 100.0 },
    }
}
```

`dangerous_inner_html` has been used. Fingers crossed. Tests pass.

## 🌱 For the Leptos Enjoyers

Leptos gets `#[prop(into)]` on `id` and `name` so you can pass string literals directly without `.to_string()` everywhere. You're welcome:

```rust
view! {
    <Sushi
        id="avocado-maki"
        name="Avocado Maki"
        shape=SushiShape::Circular
        ingredients=vec![Ingredient::Avocado]
        outer_sheet=SushiOuterSheet { color: NORI_COLOR.into(), thickness: 10.0 }
        size=SushiSize { width: 140.0, height: 140.0 }
    />
}
```

Thin props API, zero drama. Just good wholesome sushi.

## 🛠️ For Judges

Want to try it locally? Each example folder has full instructions:

- [Yew example](https://github.com/opensass/sushi-rs/tree/main/examples/yew): run with `trunk serve --port 3000`
- [Dioxus example](https://github.com/opensass/sushi-rs/tree/main/examples/dioxus): run with `dx serve --port 3000`
- [Leptos example](https://github.com/opensass/sushi-rs/tree/main/examples/leptos): run with `trunk serve --port 3000`

Each landing page shows a 3-column grid of 12 interactive example cards: rendered sushi on the bottom, raw source code on the top.

## 💡 What's Next?

- 🕹️ **Interactive sushi builder** with reactive props
- 🌍 **Sushi Gallery themes** (dark plating, minimalist kaiseki)
- 🏎️ **WASM streaming rendering** for galleries with 100+ pieces

## 💬 Final Thoughts

When the challenge said "comfort food," we didn't build a recipe card. We built a **multi-framework, fully accessible, SVG sushi rendering engine** in Rust.

- ✅ Built with Rust and zero JavaScript
- ✅ Works in Yew, Dioxus, and Leptos
- ✅ 4 shapes × 2 views × 12 ingredients = _a lot_ of sushi
- ✅ ARIA labels on every roll so your screen reader knows it's salmon
- ✅ Ferris the crab is now also a sushi chef 🦀🍣

> Add it. Ship it. Eat it (figuratively). 🍣

If you made it to the end of this post without getting hungry, you are built different, and we want you on our team. Come say hi on [Discord](https://discord.gg/b5JbvHW5nv) and star [the repo](https://github.com/opensass/sushi-rs). We have sushi. (SVG sushi. But still.)

![ferris smoking with the boys](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTSV1ZX7WKTTNgRRdmcYhw5RDIfk0y2tUE4L5m9mlMczQ&s=10)

Till next time: _Keep Rustin', keep rollin'_ 🦀🍣
