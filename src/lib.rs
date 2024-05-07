use std::env;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::BreakpointsTailwind::*;
use leptos_use::{breakpoints_tailwind, use_breakpoints};

// Modules
mod commons;
mod components;
mod features;
mod utilities;

use crate::features::dashboard::page::PageManager;
// Top-Level pages
// use crate::pages::home::Home;
// use crate::pages::not_found::NotFound;
use crate::components::not_found::NotFound;
use crate::utilities::configuration::get_environment;
use crate::utilities::cookies::check_server_cookie;

#[derive(Copy, Clone)]
pub struct Refetcher(pub RwSignal<bool>);

#[derive(Copy, Clone)]
pub struct HasError(pub RwSignal<bool>);

#[derive(Copy, Clone)]
pub struct CheckCookie(pub Resource<bool, Result<bool, ServerFnError>>);

/// An app router which renders the homepage and handles 404's
#[component]
#[allow(non_snake_case)]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // let environment_resource = Resource::once(move || get_environment());
    let screen_width = use_breakpoints(breakpoints_tailwind());

    let xs = screen_width.le(Sm);
    let sm = screen_width.between(Sm, Md);
    let md = screen_width.between(Md, Lg);
    let lg = screen_width.ge(Lg);

    let environment_resource =
        create_local_resource(|| (), move |_| async move { get_environment().await });

    let refetcher = create_rw_signal(false);
    let has_error = create_rw_signal(false);

    let auth_resource: Resource<bool, Result<bool, ServerFnError>> =
        create_local_resource(refetcher, move |_| async move {
            check_server_cookie("admin_portal_csr".to_string()).await
        });

    let environment = move || match environment_resource.get() {
        Some(env) => env,
        None => "".to_string(),
    };

    let responsive_stamp = move || match (xs.get(), sm.get(), md.get(), lg.get()) {
        (true, _, _, _) => view! { <h1 class="dev-stamp-sm">{environment()}</h1> },
        (_, true, _, _) => view! { <h1 class="dev-stamp-sm">{environment()}</h1> },
        (_, _, true, _) => view! { <h1 class="dev-stamp-md">{environment()}</h1> },
        (_, _, _, true) => view! { <h1 class="dev-stamp-lg">{environment()}</h1> },
        _ => view! { <h1 class="dev-stamp-lg">{environment()}</h1> },
    };

    provide_context(Refetcher(refetcher));
    provide_context(HasError(has_error));
    provide_context(CheckCookie(auth_resource));

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="darkpurple"/>

        // sets the document title
        <Title text="Jabra Admin Portal"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <main class="font-poppins">

                {move || { responsive_stamp() }} <div class="min-h-screen">

                    <Routes>
                        <Route path="/" view=PageManager/>
                        <Route path="/login" view=PageManager/>
                        <Route path="/home" view=PageManager/>
                        <Route path="/trades/deals" view=PageManager/>
                        <Route path="/quotes/active" view=PageManager/>
                        <Route path="/trades/recents" view=PageManager/>
                        <Route path="/trades/expiring" view=PageManager/>
                        <Route path="/trades/termsheets" view=PageManager/>
                        <Route path="/quotes/builder" view=PageManager/>
                        <Route path="/counterparties" view=PageManager/>
                        <Route path="/trades/positions" view=PageManager/>
                        <Route path="/trades/history" view=PageManager/>
                        <Route path="/riskslide" view=PageManager/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}
