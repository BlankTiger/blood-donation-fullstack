pub mod stations_table;
use leptos::*;
use crate::auth::User;
use crate::app::admin_dashboard::stations_table::*;


#[component]
pub fn AdminDashboard(cx: Scope, user: Signal<Option<User>>) -> impl IntoView {
    if user().is_none() {
        return view! {
            cx,
            <div class="flex w-full items-center justify-center">
                <h1 class="text-black text-4xl pt-20 px-20 text-center">
                    "You are not authorized to view this page."
                </h1>
            </div>
        };
    };

    view! {
        cx,
        <div class="w-full">
            <StationsTable />
        </div>
    }
}
