use crate::model::user::User;
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;
        use axum_sessions_auth::SessionMySqlPool;
        use argon2::{
            password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
            Argon2,
        };
        use crate::app::{pool, auth};
        pub type AuthSession = axum_sessions_auth::AuthSession<User, i64, SessionMySqlPool, MySqlPool>;

        static PEPPER: &str = "very secure pepper";

        fn get_argon2_instance() -> Result<Argon2<'static>, ServerFnError> {
            Argon2::new_with_secret(
                PEPPER.as_bytes(),
                Default::default(),
                Default::default(),
                Default::default(),
            )
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
        }
    }
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = auth(cx)?;

    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    cx: Scope,
    email: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    let user: User = User::get_from_email(email, &pool)
        .await
        .ok_or("User does not exist.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    dbg!(&user.permissions);

    let argon2 = get_argon2_instance().map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let parsed_hash =
        PasswordHash::new(&user.password).map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect(cx, "/admin");
            Ok(())
        }
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    use crate::app::auth;
    let auth = auth(cx)?;

    auth.logout_user();
    leptos_axum::redirect(cx, "/");

    Ok(())
}

#[server(Signup, "/api")]
pub async fn signup(
    cx: Scope,
    email: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = get_argon2_instance().map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .to_string();

    sqlx::query("INSERT INTO users (email, password) VALUES (?,?)")
        .bind(email.clone())
        .bind(password_hash)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let user = User::get_from_email(email, &pool)
        .await
        .ok_or("Signup failed: User does not exist.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect(cx, "/");

    Ok(())
}
