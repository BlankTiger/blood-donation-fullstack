use cfg_if::cfg_if;

// TODO: add searching
// TODO: change logging strategy to eyre
// TODO: maybe add map
// TODO: refactor

cfg_if! {
if #[cfg(feature = "ssr")] {
    use krwiodawstwo_fullstack::app::*;
    use krwiodawstwo_fullstack::auth::*;
    use krwiodawstwo_fullstack::*;
    use krwiodawstwo_fullstack::fallback::file_and_error_handler;
    use axum::{
        response::{Response, IntoResponse},
        routing::{post, get},
        extract::{Path, Extension},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
        Router,
    };
    use axum_sessions_auth::{AuthSessionLayer, AuthConfig, SessionMySqlPool};
    use axum_database_sessions::{SessionConfig, SessionLayer, SessionStore};
    use leptos::{log, view, provide_context, LeptosOptions, get_configuration};
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
    use dotenvy::dotenv;
    use std::sync::Arc;
    use std::env;

    async fn server_fn_handler(Extension(pool): Extension<MySqlPool>, auth_session: AuthSession, path: Path<String>, headers: HeaderMap, request: Request<AxumBody>) -> impl IntoResponse {

        log!("{:?}", path);

        handle_server_fns_with_context(path, headers, move |cx| {
            provide_context(cx, auth_session.clone());
            provide_context(cx, pool.clone());
        }, request).await
    }

    async fn leptos_routes_handler(
        Extension(pool): Extension<MySqlPool>,
        auth_session: AuthSession,
        Extension(options): Extension<Arc<LeptosOptions>>,
        req: Request<AxumBody>
    ) -> Response{
            let handler = leptos_axum::render_app_to_stream_with_context((*options).clone(),
            move |cx| {
                provide_context(cx, auth_session.clone());
                provide_context(cx, pool.clone());
            },
            |cx| view! { cx, <App/> }
        );
        handler(req).await.into_response()
    }

    #[tokio::main]
    async fn main() {
        simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = sqlx::MySqlPool::connect(&database_url).await.expect("pool to be made");

        // Auth section
        let session_config = SessionConfig::default().with_table_name("axum_sessions");
        let auth_config = AuthConfig::<i64>::default();
        let session_store = SessionStore::<SessionMySqlPool>::new(Some(pool.clone().into()), session_config);
        session_store.initiate().await.unwrap();

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("could not run SQLx migrations");

        crate::app::register_server_functions();

        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

        // build our application with a route
        let app = Router::new()
            .route("/api/*fn_name", post(server_fn_handler))
            .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
            .fallback(file_and_error_handler)
            .layer(AuthSessionLayer::<User, i64, SessionMySqlPool, MySqlPool>::new(Some(pool.clone()))
                        .with_config(auth_config))
            .layer(SessionLayer::new(session_store))
            .layer(Extension(Arc::new(leptos_options)))
            .layer(Extension(pool));

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

    // client-only stuff for Trunk
    else {
        pub fn main() {
            // This example cannot be built as a trunk standalone CSR-only app.
            // Only the server may directly connect to the database.
        }
    }
}
