use axum::{
    extract::DefaultBodyLimit,
    middleware,
    routing::{get, post},
    Router,
};

use super::handlers;
use crate::config::AppState;

/// 创建路由
pub fn create_routes(state: AppState) -> Router {
    // 公开路由（无需认证）
    let public = Router::<AppState>::new()
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/auth/refresh", post(handlers::auth::refresh_token))
        .route("/clients/register", post(handlers::client::register))
        .route("/clients/heartbeat", post(handlers::client::heartbeat));

    // 需要认证的路由
    let protected = Router::<AppState>::new()
        .route("/auth/user-info", get(handlers::auth::get_user_info))
        .nest("/users", user_routes())
        .nest("/roles", role_routes())
        .nest("/softwares", software_routes())
        .nest("/blacklists", blacklist_routes())
        .nest("/clients", Router::new().route("/", get(handlers::client::list)))
        .nest("/reports", report_routes())
        .nest("/categories", category_routes())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            super::middleware::auth::auth_middleware,
        ));

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state)
}

/// 用户路由（需要认证 + 管理员权限）
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::user::list).post(handlers::user::create))
        .route(
            "/:id",
            get(handlers::user::get)
                .put(handlers::user::update)
                .delete(handlers::user::delete),
        )
        .route("/:id/reset-password", post(handlers::user::reset_password))
        .layer(middleware::from_fn(
            super::middleware::authz::admin_required,
        ))
}

/// 角色路由（需要认证 + 管理员权限）
fn role_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::role::list))
        .layer(middleware::from_fn(
            super::middleware::authz::admin_required,
        ))
}

/// 软件路由（需要认证）
fn software_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::software::list).post(handlers::software::create),
        )
        .route(
            "/:id",
            get(handlers::software::get)
                .put(handlers::software::update)
                .delete(handlers::software::delete),
        )
        .route("/:id/download", get(handlers::software::download))
        .layer(DefaultBodyLimit::max(500 * 1024 * 1024)) // 500MB
}

/// 黑名单路由
fn blacklist_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::blacklist::list).post(handlers::blacklist::create),
        )
        .route(
            "/:id",
            get(handlers::blacklist::get)
                .put(handlers::blacklist::update)
                .delete(handlers::blacklist::delete),
        )
        .route("/client", get(handlers::blacklist::get_client_blacklist))
}

/// 客户端路由
fn client_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::client::register))
        .route("/heartbeat", post(handlers::client::heartbeat))
        .route("/", get(handlers::client::list))
}

/// 报告路由
fn report_routes() -> Router<AppState> {
    Router::new()
        .route("/process-scans", post(handlers::report::report_scan))
        .route("/scan-records", get(handlers::report::get_scan_records))
        .route("/statistics", get(handlers::report::get_statistics))
}

/// 分类路由
fn category_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::category::list).post(handlers::category::create),
        )
        .route(
            "/:id",
            get(handlers::category::get)
                .put(handlers::category::update)
                .delete(handlers::category::delete),
        )
}
