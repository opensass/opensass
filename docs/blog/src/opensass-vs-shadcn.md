> Welcome 👋!

When evaluating modern UI libraries, the market is flooded with impressive choices; But not all are built equal. **Shadcn UI** has earned a following among developers working primarily within the React + Tailwind + Radix ecosystem. However, its design philosophy is inherently narrow. It forces teams into a predefined stack, leaving little room for freedom, modularity, or framework independence. That's where **Open SASS** comes in, not just as an alternative, but as a next-generation solution built for a truly agnostic, performant, and future-ready development experience. Let's explore why **Open SASS** doesn't just compete; It **obliterates** Shadcn UI on every front.

## Framework and Styling Agnosticism

One of the most obvious limitations of **Shadcn UI** is that it is _hardwired_ into the React ecosystem. You can't use it without installing and depending on **React**, **Radix UI**, **Lucide React**, and **Tailwind CSS**. This is fine if you're working on a Next.js project and never plan to leave that bubble. But what happens when you're building with **Vue**, **Svelte** or even a native desktop interface? You're out of luck. Although there are unofficial ShadCN components available for each of these frameworks, they are maintained by the community rather than the core team. Shadcn simply does not operate outside of React.

To see how tightly Shadcn is bound to its stack, try installing a simple Accordion component with the following command:

```sh
npx shadcn@latest add accordion
```

The generated component looks like this:

```js
"use client";

import * as React from "react";
import * as AccordionPrimitive from "@radix-ui/react-accordion";
import { ChevronDownIcon } from "lucide-react";

import { cn } from "@/lib/utils";

function Accordion({
  ...props
}: React.ComponentProps<typeof AccordionPrimitive.Root>) {
  return <AccordionPrimitive.Root data-slot="accordion" {...props} />;
}

function AccordionItem({
  className,
  ...props
}: React.ComponentProps<typeof AccordionPrimitive.Item>) {
  return (
    <AccordionPrimitive.Item
      data-slot="accordion-item"
      className={cn("border-b last:border-b-0", className)}
      {...props}
    />
  );
}

function AccordionTrigger({
  className,
  children,
  ...props
}: React.ComponentProps<typeof AccordionPrimitive.Trigger>) {
  return (
    <AccordionPrimitive.Header className="flex">
      <AccordionPrimitive.Trigger
        data-slot="accordion-trigger"
        className={cn(
          "focus-visible:border-ring focus-visible:ring-ring/50 flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 [&[data-state=open]>svg]:rotate-180",
          className
        )}
        {...props}
      >
        {children}
        <ChevronDownIcon className="text-muted-foreground pointer-events-none size-4 shrink-0 translate-y-0.5 transition-transform duration-200" />
      </AccordionPrimitive.Trigger>
    </AccordionPrimitive.Header>
  );
}

function AccordionContent({
  className,
  children,
  ...props
}: React.ComponentProps<typeof AccordionPrimitive.Content>) {
  return (
    <AccordionPrimitive.Content
      data-slot="accordion-content"
      className="data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down overflow-hidden text-sm"
      {...props}
    >
      <div className={cn("pt-0 pb-4", className)}>{children}</div>
    </AccordionPrimitive.Content>
  );
}

export { Accordion, AccordionItem, AccordionTrigger, AccordionContent };
```

This code is riddled with dependencies: `@radix-ui/react-accordion`, `lucide-react`, Tailwind utilities, and React's own component system. Want to use it in a Vue app? Too bad. Trying to reuse it in a Yew project written in Rust? Forget about it. You're locked in, hard.

In contrast, **Open SASS** components are the very definition of _framework-agnostic_. Each component in Open SASS is shipped in multiple flavors, tailored to the platform you're using. Want to use the Accordion in **Yew**? Run, after installing the [Open SASS CLI](https://crates.io/crates/opensass):

```sh
os add accordion-rs yew
```

This fetches only the Rust/Yew-compatible source code with no extra fluff, no unnecessary dependencies, and no styling assumptions. Here's an excerpt of what you get:

```rust
use crate::common::{Align, Size};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AccordionProps {
    pub expand: UseStateHandle<bool>,

    #[prop_or_default]
    pub expanded: Html,

    #[prop_or_default]
    pub collapsed: Html,

    #[prop_or_default]
    pub children: ChildrenWithProps<List>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub aria_controls: &'static str,

    #[prop_or_default]
    pub style: &'static str,

    #[prop_or_default]
    pub expanded_style: &'static str,

    #[prop_or_default]
    pub collapsed_style: &'static str,

    #[prop_or_default]
    pub content_style: &'static str,

    #[prop_or_default]
    pub class: &'static str,

    #[prop_or_default]
    pub expanded_class: &'static str,

    #[prop_or_default]
    pub collapsed_class: &'static str,

    #[prop_or_default]
    pub content_class: &'static str,

    #[prop_or(true)]
    pub aria_enabled: bool,

    #[prop_or(600)]
    pub duration: u64,

    #[prop_or_default]
    pub will_open: Callback<()>,

    #[prop_or_default]
    pub did_open: Callback<()>,

    #[prop_or_default]
    pub will_close: Callback<()>,

    #[prop_or_default]
    pub did_close: Callback<()>,
}

#[function_component]
pub fn Accordion(props: &AccordionProps) -> Html {
    let is_expanded = &props.expand;
    let is_expanded_value = **is_expanded;

    let toggle_expansion = {
        let is_expanded = is_expanded.clone();
        let props = props.clone();

        move |e: MouseEvent| {
            e.prevent_default();

            if is_expanded_value {
                props.will_close.emit(());
                is_expanded.set(false);
                props.did_close.emit(());
            } else {
                props.will_open.emit(());
                is_expanded.set(true);
                props.did_open.emit(());
            }
        }
    };

    html! {
        <div
            style={format!(
                "{} {}",
                props.size.to_style(),
                props.style
            )}
            class={props.class}
        >
            <div
                aria-expanded={if props.aria_enabled { Some(is_expanded_value.to_string()) } else { None }}
                aria-controls={if props.aria_enabled { Some(props.aria_controls) } else { None }}
                onclick={toggle_expansion.clone()}
                class={if is_expanded_value {
                        props.expanded_class
                    } else {
                        props.collapsed_class
                    }}
                style={format!(
                    "cursor: pointer; transition: all {}ms; {}",
                    props.duration,
                    if is_expanded_value {
                        props.expanded_style
                    } else {
                        props.collapsed_style
                    }
                )}
            >
                { if is_expanded_value { props.expanded.clone() } else { props.collapsed.clone() } }
            </div>
            { if is_expanded_value {
                html! {
                    <div
                        id={props.aria_controls}
                        class={props.content_class}
                        style={format!(
                            "overflow: hidden; transition: all {}ms; {}",
                            props.duration,
                            props.content_style
                        )}
                    >
                    { for props.children.iter() }
                    </div>
                }
            } else {
                html! {}
            } }
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ItemProps {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub style: &'static str,

    #[prop_or_default]
    pub class: &'static str,

    #[prop_or_default]
    pub align: Align,

    #[prop_or_default]
    pub title: &'static str,

    #[prop_or_default]
    pub icon: &'static str,
}

#[function_component]
pub fn Item(props: &ItemProps) -> Html {
    html! {
        <li
            class={props.class}
            style={format!(
                "{} {}",
                props.align.to_style(),
                props.style
            )}
        >
            { if !props.icon.is_empty() {
                    html! { <span class="mr-2">{ props.icon }</span> }
                } else {
                    html! {}
                } }
            { if !props.title.is_empty() {
                    html! { <strong>{ props.title }</strong> }
                } else {
                    html! {}
                } }
            { props.children.clone() }
        </li>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub style: &'static str,

    #[prop_or_default]
    pub class: &'static str,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html! {
        <button class={props.class} style={props.style}>
            { props.children.clone() }
        </button>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub style: &'static str,

    #[prop_or_default]
    pub class: &'static str,
}

#[function_component]
pub fn List(props: &ListProps) -> Html {
    html! {
        <ul class={props.class} style={props.style}>
            { for props.children.iter() }
        </ul>
    }
}
```

No framework or CSS library is forced on you. Use **Tailwind**, **Bootstrap**, **Vanilla CSS**, or **inline styles**; Open SASS doesn't care. The component is just logic and structure, not opinionated about how you present it. Want to switch from Yew to Dioxus? Easy:

```sh
os add accordion-rs dio
```

This modularity empowers teams to build components once and reuse them across frontend frameworks, native apps, and even WebAssembly runtimes. This kind of adaptability is simply **unachievable** with Shadcn UI.

## Performance

In software, speed isn't everything; But when tooling begins to slow down your workflow, it becomes a bottleneck. Shadcn UI relies on a Node.js-based CLI to fetch and generate components. That means it needs to spawn a Node process, resolve dependencies, and scaffold code in a relatively bloated JavaScript environment.

Here's what happens when you run Shadcn's CLI:

```sh
❯ time npx shadcn@latest add accordion
5.93s user 2.22s system 93% cpu
```

Almost **6 seconds** to add a single component. That may not sound like much on its own, but in large projects or CI/CD pipelines, those seconds **add up**.

Now compare that to **Open SASS**:

```sh
❯ time os add accordion-rs yew
0.17s user 0.02s system 6% cpu
```

**Under 0.2 seconds** of actual user time. That's roughly **60x faster**. The reason? Open SASS is built using **native Rust**, compiled ahead-of-time to a single binary, and it makes blazing fast HTTP requests directly to `crates.io`. There's no JavaScript runtime bloat, no npm resolution latency, and no unnecessary complexity.

This level of performance doesn't just make your dev life smoother; It scales beautifully in automated systems, remote dev containers, CI/CD pipelines, and embedded developer tooling. It's the kind of raw speed you feel instantly, and once you've experienced it, you'll never want to go back to a sluggish Node-based CLI.

## Customization and Extensibility

Shadcn UI gives you a component that looks nice; **If** you stay inside its carefully crafted box. It assumes you want Tailwind. It assumes you want React. It assumes you want Radix. If you try to diverge, maybe to add some accessibility behavior that Radix doesn't support out-of-the-box; You'll find yourself fighting the abstraction more than working with it.

Let's say you want to change the accordion animation or use a different icon system in Shadcn. You're diving into a web of dependencies like `@radix-ui/react-accordion`, needing to override Radix's animations, swap Lucide icons, and massage Tailwind classes around React's className hell.

Open SASS, on the other hand, is **fully declarative and extensible**. You can:

- Customize animation speed with a `duration` prop
- Inject callbacks (`will_open`, `did_open`, `will_close`, `did_close`)
- Apply your own ARIA logic
- Use class-based or style-based theming
- Extend components like `Item`, `Button`, or `List` with your own elements

No wrappers. No monkey patching. No awkward re-exports. Just clean, readable components, documented in detail, with no styling or framework assumptions. You're the boss.

Want a bootstrap-themed accordion with your own icons and custom transition logic? You don't have to hack around someone else's component; You build directly with Open SASS:

```rust
<Accordion
    expand={expand}
    expanded={html! { <div>{"Details visible"}</div> }}
    collapsed={html! { <div>{"Click to expand"}</div> }}
    class="accordion-container"
    style="background: #fff; border: 1px solid #ccc;"
    expanded_style="background: #eef;"
    collapsed_style="background: #fee;"
    duration={300}
/>
```

No layers of indirection. No theme providers. Just clean code.

## Conclusion

In every measurable way, **portability**, **performance**, **customization**, **extensibility**, and **cross-platform support**, **Open SASS** obliterates **Shadcn UI**. Shadcn is a slick choice if you're building _yet another_ Tailwind/React site. But the moment you need to step outside that bubble, whether it's for performance, flexibility, native development, or Rust integration, Shadcn breaks. Open SASS thrives.

Open SASS isn't just an alternative, it's the **future** of component libraries. Cross-framework. Cross-language (TODO). Ultra-fast. Infinitely extensible.

If you want freedom, speed, and true composability; Open SASS isn't just better.

It's on a whole other level.

> **We are Open SASS, babe!**.

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Together, let's move the web beyond JavaScript, and into something that actually compiles.

> Feel free to share this. Fork it. Turn it into a meme. Tattoo it on your CI pipeline. Tell your manager Rust is your spirit animal now.
