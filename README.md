<div align="center">

# 🌐 Open SASS

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[![Open SASS Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

</div>

## 🛠️ Pre-requisites:

1. Install [`rustup`](https://www.rust-lang.org/tools/install):

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.6/getting_started):

   ```sh
   cargo install dioxus-cli
   ```

## 🚀 Building and Running

1. Fork/Clone the GitHub repository.

   ```sh
   git clone https://github.com/opensass/opensass
   ```

1. SSG The Blogs.

   ```sh
   PORT=3000 dx build --ssg
   ```

1. Run The Client.

   ```sh
   dx serve --port 3000
   ```

Navigate to http://localhost:3000 to explore the landing page.
