<div align="center">

# 🌐 Open SASS

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.79%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[![Open SASS Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

<video src="https://github.com/user-attachments/assets/b376b575-69e9-4a67-a6ac-ad5104a9e3aa"></video>

</div>

## 🛠️ Pre-requisites:

1. Install [`rustup`](https://www.rust-lang.org/tools/install):

 ```bash
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
 ```

1. Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.5/getting_started):

 ```bash
 cargo install dioxus-cli@0.5.6
 ```

## 🚀 Building and Running

1. Fork/Clone the GitHub repository.

 ```bash
 git clone https://github.com/opensass/opensass
 ```

## Downgrade Crates:

   ```bash
   cargo update half --precise 2.4.0
   cargo update litemap --precise 0.7.4
   cargo update zerofrom --precise 0.1.5
   cargo update base64ct --precise 1.6.0
   ```

1. Run the client:

 ```sh
 dx serve --port 3000
 ```

Navigate to http://localhost:3000 to explore the landing page.
