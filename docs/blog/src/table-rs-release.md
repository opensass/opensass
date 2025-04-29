> 👋 Welcome

When you're building **lightning-fast**, **interactive**, and **accessible** WASM frontends with **Yew**, the little things can make a big difference, and **tables** are no exception.

Until now, building a rich data table in Rust meant reinventing the wheel, stitching together sorting logic, pagination UI, and URL state sync, all while wrestling with accessibility concerns and styling constraints.

That's why we're excited to introduce [**Table RS**](https://github.com/opensass/table-rs) 📊, a **powerful**, **lightweight**, and **accessibility-first** table component purpose-built for **Yew**, **Dioxus**, and **Leptos**.

Let's explore why **Table RS** isn't just another table, it's the one your Rust frontend deserves.

## 📦 What Is Table RS?

**Table RS** is a modular, fully-featured data table component that brings together **real-time search**, **column sorting**, **pagination**, and **style customization**, all with built-in support for **semantic markup**, **ARIA attributes**, and **URL synchronization**.

Built specifically for **WASM-based Rust frameworks**, Table RS was designed with three key priorities:

- **Performance** (don't block your app or over-render).
- **Accessibility** (keyboard users and screen readers welcome).
- **Developer Ergonomics** (composable, prop-driven API).

It's time to stop wrestling with HTML tables and start focusing on your app logic.

## 🚀 Why You'll Love Table RS

Table RS isn't just a helper, it's a solution. Here's what sets it apart:

- **🔍 Built-in Search**: Add a global search bar in one prop, no extra logic required. It even syncs with the URL (`?search=query`) so users can share filtered views.
- **⬆️ Sorting Support**: Enable column-based sorting with accessible `aria-sort` indicators for screen readers.
- **📄 Pagination**: Easily split data into pages, customize page size, and add intuitive nav controls.
- **🧹 Debounced Inputs**: Reduce unnecessary re-renders for better user experience and performance.
- **🎨 Full Customization**: Override class names and inline styles with ease.
- **♿ Accessibility First**: Proper roles, ARIA attributes, and semantic tags built-in by default.
- **🛠 Zero Boilerplate**: Focus on your data, not on wiring up handlers or rebuilding UI from scratch.

## ⚙️ Quick Setup for Yew

Setting up **Table RS** in your Yew app is easy and intuitive. Let's walk through it.

### 1️⃣ Add the Dependency

```sh
cargo add table-rs --features=yew
```

### 2️⃣ Import the Component

```rust
use yew::prelude::*;
use table_rs::yew::table::Table;
use table_rs::yew::types::Column;
use maplit::hashmap;
```

### 3️⃣ Use It in Your Component

```rust
#[function_component(App)]
pub fn app() -> Html {
    let data = vec![
        hashmap! { "name" => "Ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
        hashmap! { "name" => "Ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
        hashmap! { "name" => "Crab".to_string(), "email" => "crab@opensass.org".to_string() },
    ];

    let columns = vec![
        Column {
            id: "name",
            header: "Name",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "email",
            header: "Email",
            sortable: false,
            ..Default::default()
        },
    ];

    html! {
        <Table data={data} columns={columns} />
    }
}
```

✅ That's it, your table is searchable, paginated, sortable, and accessible out of the box.

## 🧩 Table RS Props Overview

### 🔑 Core Props

| Prop        | Type                                  | Description                                     | Default  |
| ----------- | ------------------------------------- | ----------------------------------------------- | -------- |
| `data`      | `Vec<HashMap<&'static str, String>>`  | Row data, each row is a hashmap of column IDs.  | `[]`     |
| `columns`   | `Vec<Column>`                         | Describes headers and behavior for each column. | `[]`     |
| `paginate`  | `bool`                                | Enables pagination controls.                    | `false`  |
| `page_size` | `usize`                               | How many rows per page.                         | `10`     |
| `search`    | `bool`                                | Enables global search input.                    | `false`  |
| `loading`   | `bool`                                | Show a loading indicator.                       | `false`  |
| `classes`   | `TableClasses`                        | Customize class names.                          | Defaults |
| `styles`    | `HashMap<&'static str, &'static str>` | Add inline styles to table parts.               | `{}`     |

### 🧱 Column Definition

| Field      | Type                   | Description                      | Default |
| ---------- | ---------------------- | -------------------------------- | ------- |
| `id`       | `&'static str`         | Matches the key in the row data. | `""`    |
| `header`   | `&'static str`         | Display name in the header.      | `""`    |
| `sortable` | `bool`                 | Enables sorting for this column. | `false` |
| `class`    | `Option<&'static str>` | Optional class name.             | `None`  |
| `style`    | `Option<&'static str>` | Optional inline styles.          | `None`  |

### 🎨 Style/Classes Reference

| Section        | CSS Class (default)     | Description              |
| -------------- | ----------------------- | ------------------------ |
| `container`    | `"table-container"`     | Outer wrapper            |
| `search_input` | `"search-input"`        | The input box for search |
| `table`        | `"table"`               | The `<table>` element    |
| `thead`        | `"thead"`               | Header row section       |
| `tbody`        | `"tbody"`               | Body rows section        |
| `pagination`   | `"pagination-controls"` | Pagination UI wrapper    |

## 🤝 Built With Open SASS

Table RS is proudly part of the [Open SASS](https://github.com/opensass/cli) ecosystem, where we build battle-tested UI primitives for Rust-powered frontends.

This project is **open source**, **community-driven**, and ready for contributions. Whether it's bug reports, feature ideas, or PRs, we'd love to hear from you.

👉 [GitHub Repo](https://github.com/opensass/table-rs)

👉 [Live Demo](https://table-rs.netlify.app)

## 🎯 Final Thoughts

If you've struggled with hand-rolling tables in Yew or wanted something more flexible than HTML templates, **Table RS** is built for you.

It's fast, lightweight, thoughtfully designed, and extensible, ready for production use and just as happy in your side projects.

> **We are Open SASS, babe!**.

> We're working tirelessly on making Rust web development extremely easy for everyone.

> If you made it this far, it would be nice if you could [join us on Discord](https://discord.gg/b5JbvHW5nv).

> Together, let's move the web beyond JavaScript, and into something that actually compiles.

> Feel free to share this. Fork it. Turn it into a meme. Tattoo it on your CI pipeline. Tell your manager Rust is your spirit animal now.

> Till next time 👋
