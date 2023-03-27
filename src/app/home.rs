use leptos::*;
use crate::app::overview::*;
use crate::app::stations_table::*;

use crate::auth::User;

#[component]
pub fn Home(cx: Scope, logged_in_user: Signal<Option<User>>) -> impl IntoView {
    if let Some(user) = logged_in_user() {
        view! {
            cx,
            <div class="w-full mt-32">
                <Overview />
                <StationsTable />
            </div>
        }
        .into_view(cx)
    } else {
        view! {
            cx,
            <div class="w-full flex items-center justify-center">
                <h1 class="">"Login"</h1>
            </div>
        }
        .into_view(cx)
    }
}
