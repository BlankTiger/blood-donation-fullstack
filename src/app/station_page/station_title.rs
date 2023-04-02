use crate::app::auth::auth_guard::*;
use crate::model::station::Station;
use leptos::*;

#[component]
pub fn StationTitle(cx: Scope, station: Station) -> impl IntoView {
    let station_copy = station.clone();

    view! { cx,
        <AuthGuardTwoViews
            view_authed=move || {
                view! { cx, {format!("Stan krwi w {}, id={}", station.name, station.id)} }
                    .into_view(cx)
            }
            view_unauthed=move || {
                view! { cx, {format!("Stan krwi w {}", station_copy.name)} }
                    .into_view(cx)
            }
        />
    }
}
