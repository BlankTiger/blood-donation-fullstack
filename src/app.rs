mod add_station;
mod admin_dashboard;
pub mod auth;
mod home;
mod navbar;
pub mod notification;
mod station_page;
mod stations_table;
mod update_available_blood;
use crate::{
    app::{
        add_station::*, admin_dashboard::*, auth::login::*, auth::logout::*, auth::signup::*,
        home::*, navbar::*, station_page::*, update_available_blood::*,
    },
    auth::*,
    error_template::{ErrorTemplate, ErrorTemplateProps},
    model::UserResource,
};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;
        use crate::auth::AuthSession;
        use crate::actions::station::*;
        use crate::actions::blood::*;

        pub fn pool(cx: Scope) -> Result<MySqlPool, ServerFnError> {
            Ok(use_context::<MySqlPool>(cx)
                .ok_or("Pool missing.")
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?)
        }

        pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
            Ok(use_context::<AuthSession>(cx)
                .ok_or("Auth session missing.")
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?)
        }

        pub fn register_server_functions() {
            _ = Login::register();
            _ = Logout::register();
            _ = Signup::register();
            _ = GetUser::register();
            _ = StationData::register();
            _ = StationsTable::register();
            _ = StationsTableQuery::register();
            _ = AddStation::register();
            _ = DeleteStation::register();
            _ = UpdateAvailableBlood::register();
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let login = create_server_action::<Login>(cx);
    let logout = create_server_action::<Logout>(cx);
    let signup = create_server_action::<Signup>(cx);

    let get_curr_user = move |cx| async move { get_user(cx).await.ok().unwrap_or(None) };

    let user: UserResource = create_resource(
        cx,
        move || {
            (
                login.version().get(),
                logout.version().get(),
                signup.version().get(),
            )
        },
        move |_| get_curr_user(cx),
    );

    provide_meta_context(cx);
    provide_context(cx, user);

    view! { cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/blood-donation-fullstack.css"/>
        <Router>
            <Navbar logout_action=logout/>
            <main class="w-screen h-screen flex items-center">
                <Routes>
                    <Route
                        path=""
                        view=move |cx| {
                            view! { cx,
                                <ErrorBoundary fallback=move |cx, errors| {
                                    view! { cx, <ErrorTemplate errors=errors/> }
                                }>
                                    <Home/>
                                </ErrorBoundary>
                            }
                        }
                    />
                    <Route
                        path="admin"
                        view=move |cx| {
                            view! { cx, <AdminDashboard/> }
                        }
                    />
                    <Route
                        path="add-station"
                        view=move |cx| {
                            view! { cx, <AddStation/> }
                        }
                    />
                    <Route
                        path="update-available-blood"
                        view=move |cx| {
                            view! { cx, <UpdateAvailableBlood/> }
                        }
                    />
                    <Route
                        path="station/:id"
                        view=move |cx| {
                            view! { cx, <StationPage/> }
                        }
                    />
                    <Route
                        path="login"
                        view=move |cx| {
                            view! { cx, <Login action=login/> }
                        }
                    />
                    <Route
                        path="logout"
                        view=move |cx| {
                            view! { cx, <Logout action=logout/> }
                        }
                    />
                    <Route
                        path="signup"
                        view=move |cx| {
                            view! { cx, <Signup action=signup/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
