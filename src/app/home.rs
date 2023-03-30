use crate::app::overview::*;
use crate::app::stations_table::*;
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-full mt-96">
            <Overview/>
            <StationsTable/>
        </div>
    }
}
