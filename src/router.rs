use crate::components::navbar::dropdown::Dropdown;
use crate::components::navbar::NavBar;
use crate::pages::admin::AdminPanel;
use crate::pages::aibook::AIBook;
use crate::pages::blog::Blog;
use crate::pages::blogs::Blogs;
use crate::pages::donate::Donate;
use crate::pages::eldflow::ELDFlow;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::nanoog::NanoOG;
use crate::pages::register::Register;
use crate::pages::soulchain::SoulChain;
use crate::pages::tripper::Tripper;
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
    Blog { id: String },
    #[route("/admin/signup")]
    Register {},
    #[route("/admin/login")]
    Login {},
    // #[layout(NavBar)]
    #[route("/blogs")]
    Blogs {},
    // TODO: use protected routes
    // #[end_layout]
    // #[guard("/admin", |_| {
    //     let mut tok = "".to_string();
    //     spawn(async move {
    //         let token: String =
    //             SessionStorage::get("jwt").expect("JWT not found in session storage");

    //         match about_me(token.clone()).await {
    //             Ok(data) => {
    //                 let user = data.data.user;
    //                 if user.role == "admin" {
    //                     tok = token;
    //                 }
    //             }
    //             Err(e) => {
    //             }
    //         }
    //     });
    //     if tok.is_empty() {
    //         Some(Login {})
    //     } else {
    //         Some(AdminPanel {})
    //     }
    // })]
    #[route("/admin")]
    AdminPanel {},
    // #[end_guard]
    #[route("/donate")]
    Donate {},
    #[route("/aibook")]
    AIBook {},
    #[route("/tripper")]
    Tripper {},
    #[route("/nanoog")]
    NanoOG {},
    #[route("/eldflow")]
    ELDFlow {},
    #[route("/soulchain.pdf")]
    SoulChain {},
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
