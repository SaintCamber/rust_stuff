use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::Simple_Counter;
use crate::components::Skybox;
#[component]
pub fn App() -> impl IntoView {

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-web-app.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

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
                <Routes>
                    <Route path="" view=HomePageTwo/>
                    <Route path="/test" view=Skybox::Skybox_background/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePageTwo() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);


    view! {
        <h1>"Testing component creations"</h1>
        <h3>"here goes a smaller heading"</h3>
        <div>
            <h1>"nested h1 with a counter"</h1>
            <Skybox::Skybox_background/>

            </div>
    }
}

