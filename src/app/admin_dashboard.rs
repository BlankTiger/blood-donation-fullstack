use crate::app::stations_table::*;
use crate::app::AuthGuard;
use crate::app::AuthGuardProps;
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
