mod update_blood_authed;

use self::update_blood_authed::*;
use crate::app::auth::auth_guard::*;
use leptos::*;

#[component]
pub fn UpdateAvailableBlood(cx: Scope) -> impl IntoView {
    view! { cx,
        <AuthGuard view=move || {
            view! { cx, <Authorized/> }
        }/>
    }
}
