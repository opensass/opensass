use crate::components::navbar::dropdown::Dropdown;
use crate::components::navbar::NavBar;
use crate::pages::blog::Blog;
use crate::pages::blogs::Blogs;
use crate::pages::create_blog::CreateBlog;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::register::Register;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[nest("/about")]
    #[layout(Dropdown)]
    #[route("/:name")]
    DropdownItem { name: String },
    #[end_layout]
    #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/signup")]
    Register {},
    #[route("/login")]
    Login {},
    #[route("/blogs")]
    Blogs {},
    #[route("/blogs/create")]
    CreateBlog {},
}
#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
#[component]
fn DropdownItem(name: String) -> Element {
    rsx! {}
}
