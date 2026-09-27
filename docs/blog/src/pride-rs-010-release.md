> Welcome back 👋, brave soul!

So, version 0.0.2 of **Pride RS** barely had time to dry before the community showed up in the Discord with... _opinions_. Big ones. Ferris the crab 🦀 got some letters. The Open SASS council convened. Arguments were had. And after a very long, very heated, very productive engineering debate (mostly in memes), **Pride RS 0.1.0** is here.

The headline? **We're dropping the T from LGBTQ+, and shipping it as a Cargo feature gate called `haram`.**

Before you type in that issue, yes, you can still opt-in. It's Rust. Everything is opt-in. That's kind of the whole point. 😌

![Ferris the crab slowly backing away from a gender studies textbook](https://media2.giphy.com/media/v1.Y2lkPTc5MGI3NjExc2w4OTQ3dnBsdjR2bXVzdTc5MWo3ZjY4eTdwNjZuczR3NTE2ZTd4ayZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/QssZ0uIbZbW8M5VVoB/giphy.gif)

## 🧠 What Even Is `haram`?

In Arabic, _haram_ (حرام) means "forbidden". In **Pride RS**, it means: _"these flag types are guarded behind a feature gate and won't compile unless you explicitly ask for them."_

Specifically, the following four flag types are now gated:

| Type          | What it represents         |
| ------------- | -------------------------- |
| `Transgender` | Gender transition          |
| `NonBinary`   | Non-binary gender identity |
| `Genderfluid` | Fluid gender identity      |
| `Agender`     | Absence of gender identity |

These four have one thing in common: they're all about **changing or rejecting biological gender**, which, in the Ferris cosmos, is debatably haram. The crab has spoken. Or at least, the Cargo feature flag has.

The other eleven types, Rainbow, Bisexual, Lesbian, Pansexual, Asexual, Aromantic, Demisexual, Polysexual, Omnisexual, Demiromantic, Graysexual, remain fully available with no feature flag required. Those are the **halal** ones. Rainbow stays. Obviously. Ferris loves rainbows.

![Rainbow appearing dramatically](https://c.tenor.com/4ktLqPXQ0DcAAAAC/tenor.gif)

## ⚙️ Under the Hood

You'd think adding a Cargo feature gate is simple. You'd be wrong. Here's why:

[`phf`](https://docs.rs/phf), our compile-time perfect hash map, does **not** support `#[cfg(...)]` inside a single `phf_map!` invocation. You can't do this:

```rust
pub static FLAG_CONFIGURATIONS: phf::Map<&'static str, FlagConfig> = phf_map! {
    "Rainbow" => FlagConfig { ... },
    #[cfg(feature = "haram")]  // <-- NOPE. Compiler says no.
    "Transgender" => FlagConfig { ... },
};
```

So instead, we compile **two entirely separate maps**, gated by `#[cfg]` at the item level:

```rust
#[cfg(not(feature = "haram"))]
pub static FLAG_CONFIGURATIONS: phf::Map<&'static str, FlagConfig> = phf_map! {
    // 11 halal entries
};

#[cfg(feature = "haram")]
pub static FLAG_CONFIGURATIONS: phf::Map<&'static str, FlagConfig> = phf_map! {
    // 15 entries (all flags)
};
```

Two statics. Same name. Mutually exclusive. Zero runtime overhead. **Perfectly legal Rust.** Ferris approves. 🦀✅

![Two identical doors](https://i.imgflip.com/43ijfs.png)

## 🚩 The New `Type` Enum

Here's what `Type` looks like now, straight from the codebase:

```rust
pub enum Type {
    Rainbow,
    Bisexual,
    Lesbian,
    Pansexual,
    Asexual,
    Aromantic,
    Demisexual,
    Polysexual,
    Omnisexual,
    Demiromantic,
    Graysexual,

    #[cfg(feature = "haram")]
    Transgender,

    #[cfg(feature = "haram")]
    NonBinary,

    #[cfg(feature = "haram")]
    Genderfluid,

    #[cfg(feature = "haram")]
    Agender,
}
```

Without `--features haram`, the four guarded variants don't exist. At all. Not a dead code warning. Not a `None`. They literally **do not compile into the binary**. Zero bytes. Zero overhead. Four fewer existential crises in your type system.

![Something cute disappearing](https://gifdb.com/images/high/poof-cute-magic-disappear-1qsy2ek9t31kcqt2.webp)

## 🔍 The `is_haram()` Method

New in 0.1.0, we ship a runtime inspection method, available only when the `haram` feature is enabled (because, well, the variants don't even exist otherwise):

```rust
#[cfg(feature = "haram")]
pub fn is_haram(self) -> bool {
    matches!(
        self,
        Type::Transgender | Type::NonBinary | Type::Genderfluid | Type::Agender
    )
}
```

O(1). No heap. No drama. Just a match arm and a boolean.

```rust
assert!(Type::Transgender.is_haram());
assert!(!Type::Rainbow.is_haram());
```

Ferris the crab, checking IDs at the door like a bouncer in a tiny crab hat. 🦀🎩

![Checking list](https://i.giphy.com/nf9OAG4MUPbsOUDtu4.webp)

## 🏷️ The `haram` Field on `FlagConfig`

We also added a `haram: bool` field to the `FlagConfig` struct itself, so that tooling, docs generators, and runtime inspectors can ask _"hey, is this flag type on the haram list?"_ without needing `#[cfg]` gymnastics:

```rust
pub struct FlagConfig {
    pub colors: &'static [&'static str],
    pub direction: Direction,
    pub name: &'static str,
    pub description: &'static str,
    pub haram: bool,  // <-- new!
}
```

Rainbow? `haram: false`. Transgender (when enabled)? `haram: true`. Useful if you want to render a little ⚠️ badge or log a warning before someone deploys a fully featured pride flag to a government app in Riyadh.

## 🛠️ Using the `haram` Feature

### Default (Halal) Edition

Nothing changes. Just use Pride RS as before:

```toml
[dependencies]
pride-rs = { version = "0.1.0", features = ["yew"] }
```

You get 11 flags. Go wild. Ferris blesses you.

### Full Edition (The `haram` Opt-in)

Add the feature flag to unlock all 15 types:

```toml
[dependencies]
pride-rs = { version = "0.1.0", features = ["yew", "haram"] }
```

Now the full quartet is available:

```rust
use pride_rs::yew::FlagSection;
use pride_rs::Type;

<FlagSection
    id="questionable-choices"
    title="The Haram Four"
    flags={vec![
        Type::Transgender,
        Type::NonBinary,
        Type::Genderfluid,
        Type::Agender,
    ]}
/>
```

No judgment. Cargo features are additive. Ship what you need.

![Person sneaking through a door labeled "haram"](https://media.tenor.com/KXNeRGQuSNMAAAAM/talan-talon.gif)

## 🧪 Tests: Now Cfg-Conditional

The test suite was updated to reflect the new reality:

```rust
#[test]
fn test_enum_iter_default() {
    let variants: Vec<Type> = Type::iter().collect();
    #[cfg(not(feature = "haram"))]
    assert_eq!(variants.len(), 11);
    #[cfg(feature = "haram")]
    assert_eq!(variants.len(), 15);
}
```

And a full `haram_tests` module:

```rust
#[cfg(feature = "haram")]
mod haram_tests {
    #[test]
    fn test_is_haram_true_for_transgender() {
        assert!(Type::Transgender.is_haram());
    }
    // ... and more
}
```

Run the halal suite:

```sh
cargo test
```

Run the full suite:

```sh
cargo test --features haram
```

## 📦 0.1.0 Changelog Summary

| Change                       | Details                                                      |
| ---------------------------- | ------------------------------------------------------------ |
| ✨ New feature gate          | `haram`, compile-time toggle for gender-identity flags       |
| 🚩 New `Type` variants gated | `Transgender`, `NonBinary`, `Genderfluid`, `Agender`         |
| 🔍 New method                | `Type::is_haram()` (available with `haram` feature)          |
| 🏷️ New struct field          | `FlagConfig::haram: bool`                                    |
| 📖 Docs                      | Full rustdoc on every public item with time/space complexity |
| 🧪 Tests                     | Cfg-conditional variant counts, full `haram_tests` module    |
| 🔒 License                   | MIT license banner on all source files                       |

## 💬 Final Thoughts

Look, we're not here to debate theology or gender theory. We're here to write **fast, correct, zero-overhead Rust**. And with 0.1.0, whether you want all 15 flags or just the 11 that Ferris's grandma would approve of, you get **compile-time guarantees** either way.

That's the Rust way. Strong types. Explicit opt-ins. No runtime surprises.

- ✅ Zero overhead when `haram` is off (variants don't exist in the binary)
- ✅ Full access when `haram` is on (explicit opt-in)
- ✅ `is_haram()` for runtime inspection
- ✅ `FlagConfig::haram` for tooling
- ✅ Ferris the crab, canonically confuzled 🦀❓

![Confused crab](https://media3.giphy.com/media/v1.Y2lkPTc5MGI3NjExem83OGU2OWZoNjBheWdlaXZpMmw2bzF4dXRyeWlkaGkybDF5azJzeiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/whLZjJ14pOWuW8eNRz/giphy.gif)

> Compile it. Gate it. Ship it. Let the borrow checker sort it out 🏳️‍🌈🦀.

And as always, if you have thoughts, flags (the physical kind OR the code kind), or strong opinions about Cargo feature semantics, swing by [our Discord](https://discord.gg/b5JbvHW5nv). Ferris is there. He's a little confused but he's trying his best.

Till next time: _Keep Rustin', stay halal._ 🦀💚
