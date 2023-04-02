mod stations_table_authed;
mod stations_table_unauthed;

use self::stations_table_authed::*;
use self::stations_table_unauthed::*;
use crate::app::auth::auth_guard::*;
use leptos::*;

#[component]
pub fn StationsTable(cx: Scope) -> impl IntoView {
    view! { cx,
        <AuthGuardTwoViews
            view_authed=move || {
                view! { cx, <StationsTableAuthed/> }
                    .into_view(cx)
            }
            view_unauthed=move || {
                view! { cx, <StationsTableUnauthed/> }
                    .into_view(cx)
            }
        />
    }
}
