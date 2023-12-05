use leptos::*;

use crate::molecules::menu::Menu;

#[component]
pub fn DashboardPage() -> impl IntoView{
    view! {
        <div class="page">
            <Menu />
            <div> "Welcome aboard captain"</div>
        </div>
    }
}


