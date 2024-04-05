use leptos::{component, view, IntoView};
use leptos_router::*;
use crate::components::profileButton;

#[component]
pub fn Foot() -> impl IntoView {
    view! {
        <footer>
        <a class="github" href="http://github.com/saintCamber" target="_blank" rel="noreferrer">
                    "Built by Sarah Nodwell"
                </a>
                </footer>
    }
}
#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <header class="navbar_base">
        <nav class="links_container">
        <A href="/">
        <strong>"stays"</strong>
        </A>
        <A href="/">
        <strong>"experiences"</strong>
        </A>
        <A href="/">
        <strong>"Online experiences"</strong>
        </A>
        </nav>
        <profileButton::Profile_button/>
        </header>
    }
}



