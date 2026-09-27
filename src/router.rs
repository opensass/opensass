use crate::blog::router_blog;
use crate::components::navbar::dropdown::Dropdown;
use crate::components::navbar::NavBar;
use crate::pages::admin::AdminPanel;
use crate::pages::aibook::AIBook;
use crate::pages::blog::Blog;
use crate::pages::blogs::Blogs;
use crate::pages::donate::Donate;
use crate::pages::eldflow::ELDFlow;
use crate::pages::home::Home;
use crate::pages::kit::Kit;
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
    // #[route("/:..route")]
    // PageNotFound { route: Vec<String> },
    // #[route("/blog/:id")]
    // ,
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
    // #[layout(Blog)]
    #[redirect("/", || Route::BlogPost { child: router_blog::BookRoute::AnnouncingOpensass {} })]
    #[layout(Blog)]
    #[child("/blogs")]
    BlogPost { child: router_blog::BookRoute },
    #[end_layout]
    // #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-black flex flex-col items-center justify-center font-sans",
            div {
                class: "flex flex-col items-center text-center max-w-md px-4",
                i { class: "fa-solid fa-triangle-exclamation text-[#facc15] text-5xl mb-6" } // text-yellow-400
                h1 { class: "text-white text-3xl md:text-4xl font-bold mb-4", "404 - Page not found" }
                p { class: "text-gray-400 text-sm md:text-base mb-8", "We are terribly sorry, but the page you requested doesn't exist." }
                Link {
                    to: Route::Home {},
                    class: "bg-[#16a34a] hover:bg-[#15803d] text-white font-semibold py-2 px-6 rounded-md transition-colors", // bg-green-600
                    "← Back to Home"
                }
            }
        }
    }
}
#[component]
fn DropdownItem(name: String) -> Element {
    rsx! {}
}

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>())
}
