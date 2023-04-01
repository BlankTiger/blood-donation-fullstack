mod admin_dashboard;
mod home;
mod login;
mod logout;
mod navbar;
mod overview;
mod stations_table;
mod add_station;
mod update_available_blood;
mod station_page;
use crate::app::admin_dashboard::*;
use crate::app::home::*;
use crate::app::login::*;
use crate::app::logout::Logout;
use crate::app::navbar::*;
use crate::app::add_station::*;
use crate::app::update_available_blood::*;
use crate::app::station_page::*;
use crate::auth::*;
use crate::error_template::{ErrorTemplate, ErrorTemplateProps};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;
        use crate::auth::AuthSession;
        use crate::app::stations_table::*;

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
            _ = AddStation::register();
            _ = DeleteStation::register();
            _ = UpdateAvailableBlood::register();
        }
    }
}

pub type UserResource = Resource<(usize, usize, usize), Option<User>>;

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
        <Stylesheet id="leptos" href="/pkg/session_auth_axum.css"/>
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
                        path="signup"
                        view=move |cx| {
                            view! { cx, <Signup action=signup/> }
                        }
                    />
                    <Route
                        path="logout"
                        view=move |cx| {
                            view! { cx, <h1>"Logging out..."</h1> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Signup(cx: Scope, action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! { cx,
        <ActionForm action=action>
            <h1>"Sign Up"</h1>
            <label>
                "User ID:"
                <input
                    type="text"
                    placeholder="User ID"
                    maxlength="32"
                    name="email"
                    class="auth-input"
                />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input"/>
            </label>
            <br/>
            <label>
                "Confirm Password:"
                <input
                    type="password"
                    placeholder="Password again"
                    name="password_confirmation"
                    class="auth-input"
                />
            </label>
            <br/>
            <label>
                "Remember me?" <input type="checkbox" name="remember" class="auth-input"/>
            </label>
            <br/>
            <button type="submit" class="button">
                "Sign Up"
            </button>
        </ActionForm>
    }
}
