use leptos::*;
use leptos::html::Div;
use leptos_use::on_click_outside;
use icondata as i;

#[component]
pub fn Profile_button() -> impl IntoView {
    let show_menu: RwSignal<bool> = create_rw_signal(false);
   let menu_ref = create_node_ref::<Div>();

    let handle_click = move || show_menu.set(!show_menu.get());
    let _ = on_click_outside(menu_ref, move |_| show_menu.set(false));
    
    view! {
        <div node_ref=menu_ref>
            <div class="profButtonContainer" on:click=move |_| { handle_click() }><leptos_icons::Icon icon=i::FaBarsSolid /></div>
            <Show when=move || show_menu.get() fallback=|| ()>
            <ul >
                <li>
                    <a href="#">Bookings</a>
                </li>
                <li>
                    <a href="#">Your Spots</a>
                </li>
                <li>
                    <a href="#">Manage Reviews</a>
                </li>
                <li>
                    <a href="#">Logout</a>
                </li>
            </ul>
            </Show>
        </div>
    }
}
