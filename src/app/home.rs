mod overview;

use crate::app::stations_table::*;
use self::overview::*;
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <title>"Where can I donate blood?"</title>
        <div class="w-full mt-96">
            <Overview/>
            <StationsTable/>
        </div>
    }
}
