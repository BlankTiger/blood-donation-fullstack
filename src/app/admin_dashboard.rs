use crate::app::auth::auth_guard::*;
use crate::app::stations_table::*;
use leptos::*;

#[component]
pub fn AdminDashboard(cx: Scope) -> impl IntoView {
    view! { cx,
        <AuthGuard view=move || {
            view! { cx,
                <title>"Dashboard"</title>
                <div class="w-full">
                    <StationsTable/>
                </div>
            }
        }/>
    }
}
