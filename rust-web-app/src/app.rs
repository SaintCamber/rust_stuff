use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::navbar;
#[component]
pub fn App() -> impl IntoView {

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-web-app.css"/>

        // sets the document title
        <Title text="Infinibnb"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
        <main>
        <navbar::Nav/>
        <Routes>
        <Route path="/" view=HomePage ssr=SsrMode::Async/>
        </Routes>
        <navbar::Foot/>
        </main>
                </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    // let (_count, _set_count) = create_signal(0);


    view! {
        <div class="home">
            <h1>"HomePage"</h1>
            <div class="space">"Content"
                <div>
                    <h1>"stuff"</h1>
                </div>
            </div>
        </div>
    }
}

