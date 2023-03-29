use std::collections::HashSet;

use cfg_if::cfg_if;
use leptos::*;

use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::MySqlPool;
    use axum_sessions_auth::{SessionMySqlPool, Authentication, HasPermission};
    use bcrypt::{hash, verify, DEFAULT_COST};
    use crate::app::{pool, auth};
    pub type AuthSession = axum_sessions_auth::AuthSession<User, i64, SessionMySqlPool, MySqlPool>;
}}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub permissions: HashSet<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: -1,
            email: "Guest".into(),
            password: "".into(),
            permissions: HashSet::new(),
        }
    }
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use async_trait::async_trait;

    impl User {
        pub async fn get(id: i64, pool: &MySqlPool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissions>(
                "SELECT permission FROM user_permissions WHERE user_id = ?;",
            )
            .bind(id)
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }

        pub async fn get_from_email(email: String, pool: &MySqlPool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE email = ?")
                .bind(email)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissions>(
                "SELECT permission FROM user_permissions WHERE user_id = ?;",
            )
            .bind(sqluser.id)
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissions {
        pub permission: String,
    }

    #[async_trait]
    impl Authentication<User, i64, MySqlPool> for User {
        async fn load_user(userid: i64, pool: Option<&MySqlPool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();

            User::get(userid, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            self.username == "Guest"
        }
    }

    #[async_trait]
    impl HasPermission<MySqlPool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&MySqlPool>) -> bool {
            self.permissions.contains(perm)
        }
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlUser {
        pub id: i64,
        pub email: String,
        pub password: String,
    }

    impl SqlUser {
        pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissions>>) -> User {
            User {
                id: self.id,
                email: self.email,
                password: self.password,
                permissions: if let Some(user_perms) = sql_user_perms {
                    user_perms
                        .into_iter()
                        .map(|x| x.permission)
                        .collect::<HashSet<String>>()
                } else {
                    HashSet::<String>::new()
                },
            }
        }
    }
}
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = auth(cx)?;

    Ok(auth.current_user)
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

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    sqlx::query("INSERT INTO users (email, password) VALUES (?,?)")
        .bind(email.clone())
        .bind(password_hashed)
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
