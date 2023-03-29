use leptos::*;
use leptos_router::*;

#[component]
pub fn UpdateAvailableBlood(cx: Scope) -> impl IntoView {
    view! { cx,

    }
}

#[server(UpdateAvailableBlood, "/api")]
pub async fn update_available_blood(
    cx: Scope,
    station_Id: i32,
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
        .execute(&pool)
        .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
}
