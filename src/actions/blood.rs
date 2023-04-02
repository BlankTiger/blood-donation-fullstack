use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::app::{pool, auth};
}}

#[server(UpdateAvailableBlood, "/api")]
pub async fn update_available_blood(
    cx: Scope,
    station_id: i32,
    amount_a_plus: f32,
    amount_a_minus: f32,
    amount_b_plus: f32,
    amount_b_minus: f32,
    amount_ab_plus: f32,
    amount_ab_minus: f32,
    amount_o_plus: f32,
    amount_o_minus: f32,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    if amount_a_plus < 0.0
        || amount_a_minus < 0.0
        || amount_b_plus < 0.0
        || amount_b_minus < 0.0
        || amount_ab_plus < 0.0
        || amount_ab_minus < 0.0
        || amount_o_plus < 0.0
        || amount_o_minus < 0.0
    {
        return Err(ServerFnError::ServerError("Invalid amount.".to_string()));
    }

    auth.current_user.ok_or(ServerFnError::ServerError(
        "User not logged in.".to_string(),
    ))?;

    sqlx::query(
        "update stations set amount_a_plus = ?, amount_a_minus = ?, amount_b_plus = ?, amount_b_minus = ?, amount_ab_plus = ?, amount_ab_minus = ?, amount_o_plus = ?, amount_o_minus = ? where id = ?")
        .bind(amount_a_plus)
        .bind(amount_a_minus)
        .bind(amount_b_plus)
        .bind(amount_b_minus)
        .bind(amount_ab_plus)
        .bind(amount_ab_minus)
        .bind(amount_o_plus)
        .bind(amount_o_minus)
        .bind(station_id)
        .execute(&pool)
        .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
}
