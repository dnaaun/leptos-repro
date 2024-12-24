use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes};
use leptos_router::path;

#[derive(Clone)]
struct TheContext;

#[component]
pub fn Parent() -> impl IntoView {
    provide_context(TheContext);
    view! { <Outlet /> }
}

#[component]
pub fn Child() -> impl IntoView {
    use_context::<TheContext>().expect("TheContext not provided.");
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| ()>
                <ParentRoute path=path!("/") view=|| Parent >
                    <Route path=path!("") view=Child />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
