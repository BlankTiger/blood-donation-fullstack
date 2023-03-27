mod home;
mod login;
mod logout;
mod navbar;
mod overview;
mod stations_table;
mod admin_dashboard;
use crate::app::home::*;
use crate::app::login::*;
use crate::app::logout::Logout;
use crate::app::navbar::*;
use crate::app::admin_dashboard::*;
use crate::auth::*;
use crate::error_template::{ErrorTemplate, ErrorTemplateProps};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::SqlitePool;
        use crate::app::login::AuthSession;
        use crate::app::admin_dashboard::stations_table::StationsTable;

        pub fn pool(cx: Scope) -> Result<SqlitePool, ServerFnError> {
            Ok(use_context::<SqlitePool>(cx)
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
            _ = StationsTable::register();
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let login = create_server_action::<Login>(cx);
    let logout = create_server_action::<Logout>(cx);

    let user = create_resource(
        cx,
        move || (login.version().get(), logout.version().get()),
        move |_| get_user(cx),
    );

    let user_signal = Signal::derive(cx, move || {
        if let Some(Ok(user)) = user.read(cx) {
            user
        } else {
            None
        }
    });

    provide_meta_context(cx);

    view! {
        cx,
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/session_auth_axum.css"/>
        <Suspense fallback=move || view!{cx, <h1>{"Loading..."}</h1>}>
            <Router>
                <Navbar logged_in_user={user_signal} logout_action={logout} />
                <main class="w-screen h-screen flex items-center">
                    <Routes>
                        <Route path="" view=move |cx| view! {
                                cx,
                                <ErrorBoundary fallback=move |cx, errors| view!{cx, <ErrorTemplate errors=errors/>}>
                                    <Home logged_in_user={user_signal} />
                                </ErrorBoundary>
                        }/> //Route
                        <Route path="admin" view=move |cx| {
                            view! {
                                cx,
                                <AdminDashboard user={user_signal} />
                            }
                        }/>
                        <Route path="login" view=move |cx| {
                            view! {
                                cx,
                                <Login action=login />
                            }
                        }/>
                        <Route path="logout" view=move |cx| {
                            view! {
                                cx,
                                <h1>"Logging out..."</h1>
                            }
                        }/>
                    </Routes>
                </main>
            </Router>
        </Suspense>
    }
}

#[component]
pub fn Signup(cx: Scope, action: Action<Signup, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        cx,
        <ActionForm action=action>
            <h1>"Sign Up"</h1>
            <label>
                "User ID:"
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <label>
                "Confirm Password:"
                <input type="password" placeholder="Password again" name="password_confirmation" class="auth-input" />
            </label>
            <br/>
            <label>
                "Remember me?"
                <input type="checkbox" name="remember" class="auth-input" />
            </label>

            <br/>
            <button type="submit" class="button">"Sign Up"</button>
        </ActionForm>
    }
}
