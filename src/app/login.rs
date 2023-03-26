use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use axum_sessions_auth::{SessionSqlitePool, Authentication, HasPermission};
    use bcrypt::{hash, verify, DEFAULT_COST};
    use crate::app::{pool, auth};
    use crate::auth::User;
    pub type AuthSession = axum_sessions_auth::AuthSession<User, i64, SessionSqlitePool, SqlitePool>;
}}

#[server(Login, "/api")]
pub async fn login(
    cx: Scope,
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or("User does not exist.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match verify(password, &user.password).map_err(|e| ServerFnError::ServerError(e.to_string()))? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect(cx, "/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[component]
pub fn Login(cx: Scope, action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    view! {
        cx,
        <div class="h-full w-full overflow-hidden">
            <div class="min-h-screen bg-purple-400 flex justify-center items-center">
                <div
                    class="absolute w-60 h-60 rounded-xl bg-purple-300 -top-42 -left-16 z-0 transform rotate-45 hidden md:block">
                </div>
                <div
                    class="absolute w-48 h-48 rounded-xl bg-purple-300 -bottom-30 right-6 transform rotate-12 hidden md:block">
                </div>
                <div class="py-12 px-12 bg-white rounded-2xl shadow-xl z-20">
                    <ActionForm action=action>
                        <div>
                            <h1 class="text-3xl font-bold text-center mb-4 cursor-pointer">"Stacja krwiodawstwa"</h1>
                        </div>
                        <div class="space-y-4">
                            <input type="text" placeholder="Email Addres" name="username"
                                class="block text-sm py-3 px-4 rounded-lg w-full border outline-none" />
                            <input type="password" placeholder="Password" name="password"
                                class="block text-sm py-3 px-4 rounded-lg w-full border outline-none appearance-none" />
                        </div>
                        <label>
                            <input type="checkbox" name="remember" class="auth-input" />
                            " Zapamiętaj mnie"
                        </label>
                        <br/>
                        <div class="text-center mt-6">
                            <button type="submit" class="py-3 w-64 text-xl text-white bg-purple-400 rounded-2xl">"Zaloguj"</button>
                        </div>
                    </ActionForm>
                </div>
            </div>
        </div>
    }
}
