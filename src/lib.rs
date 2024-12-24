use hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos::{config::LeptosOptions, *};
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes};
use leptos_router::path;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[derive(Clone)]
struct ExpensivelyComputed;

#[component]
pub fn Parent() -> impl IntoView {
    provide_context(ExpensivelyComputed);

    view! { <Outlet /> }
}

#[component]
pub fn Child() -> impl IntoView {
    use_context::<ExpensivelyComputed>().expect("ExpensivelyComputed context not provided.");
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| ()>
                <ParentRoute
                    path=path!("/")
                    view=move || Parent
                >
                    <Route
                        path=path!("")
                        view=Child
                    />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
