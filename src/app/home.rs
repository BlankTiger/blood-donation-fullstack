use cfg_if::cfg_if;
use leptos::*;
use crate::app::overview::*;
use crate::app::stations_table::*;

use crate::auth::User;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::SqlitePool;
        use crate::app::{pool, get_user};

        #[derive(sqlx::FromRow, Clone)]
        pub struct SqlTodo {
            id: u32,
            user_id: i64,
            title: String,
            created_at: String,
            completed: bool,
        }
    }
}

#[component]
pub fn Home(cx: Scope, logged_in_user: Signal<Option<User>>) -> impl IntoView {
    if let Some(user) = logged_in_user() {
        view! {
            cx,
            <Overview />
            <StationsTable />
        }
        .into_view(cx)
    } else {
        view! {
            cx,
            <h1>"Login"</h1>
        }
        .into_view(cx)
    }
}
