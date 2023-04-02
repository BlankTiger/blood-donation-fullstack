mod authorized_add_station;
mod form;

use self::authorized_add_station::*;
use crate::app::auth::auth_guard::*;
use leptos::*;

#[component]
pub fn AddStation(cx: Scope) -> impl IntoView {
    view! { cx,
        <AuthGuard view=move || {
            view! { cx, <Authorized/> }
        }/>
    }
}
