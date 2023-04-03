use crate::model::station::Station;
use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::{pool, auth};
        use crate::model::station::SqlStation;
    }
}

#[server(AddStation, "/api")]
pub async fn add_station(
    cx: Scope,
    name: String,
    address: String,
    city: String,
    phone: String,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    if name.is_empty() {
        return Err(ServerFnError::ServerError(
            "Name field is empty.".to_string(),
        ));
    }

    if address.is_empty() {
        return Err(ServerFnError::ServerError(
            "Address field is empty.".to_string(),
        ));
    }

    if city.is_empty() {
        return Err(ServerFnError::ServerError(
            "City field is empty.".to_string(),
        ));
    }

    if phone.is_empty() {
        return Err(ServerFnError::ServerError(
            "Phone field is empty.".to_string(),
        ));
    }

    auth.current_user.ok_or(ServerFnError::ServerError(
        "You are not logged in".to_string(),
    ))?;

    sqlx::query(
        "INSERT INTO stations (name, address, city, phone, amount_a_plus, amount_a_minus, amount_b_plus, amount_b_minus, amount_ab_plus, amount_ab_minus, amount_o_plus, amount_o_minus)
        VALUES (?, ?, ?, ?, 0, 0, 0, 0, 0, 0, 0, 0)")
        .bind(name)
        .bind(address)
        .bind(city)
        .bind(phone)
        .execute(&pool)
        .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
}

#[server(DeleteStation, "/api")]
pub async fn delete_station(cx: Scope, station_id: i32) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    auth.current_user.ok_or(ServerFnError::ServerError(
        "User not logged in.".to_string(),
    ))?;

    sqlx::query("DELETE FROM stations WHERE id = ?")
        .bind(station_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
}

#[server(StationData, "/api")]
pub async fn get_station_data(cx: Scope, id: String) -> Result<Station, ServerFnError> {
    let pool = pool(cx)?;

    let station = sqlx::query_as::<_, SqlStation>("SELECT * FROM stations WHERE id = ? limit 1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(station.into())
}

pub async fn get_station_data_option(cx: Scope, id: String) -> Option<Station> {
    get_station_data(cx, id).await.ok()
}

#[server(StationsTable, "/api")]
pub async fn get_stations(cx: Scope) -> Result<Vec<Station>, ServerFnError> {
    let pool = pool(cx)?;

    let stations = sqlx::query_as::<_, SqlStation>("SELECT * FROM stations")
        .fetch_all(&pool)
        .await
        .ok()
        .ok_or(ServerFnError::ServerError("No stations found.".to_string()))?;

    let stations: Vec<Station> = stations.into_iter().map(|station| station.into()).collect();
    Ok(stations)
}

pub async fn get_stations_option(cx: Scope) -> Option<Vec<Station>> {
    get_stations(cx).await.ok()
}

#[server(StationsTableQuery, "/api")]
pub async fn get_stations_query(cx: Scope, query: String) -> Result<Vec<Station>, ServerFnError> {
    let pool = pool(cx)?;

    let stations = sqlx::query_as::<_, SqlStation>("SELECT * FROM stations WHERE match(name, address, city, phone) against (? in natural language mode)")
        .bind(query)
        .fetch_all(&pool)
        .await
        .ok()
        .ok_or(ServerFnError::ServerError("No stations found.".to_string()))?;

    let stations: Vec<Station> = stations.into_iter().map(|station| station.into()).collect();
    Ok(stations)
}

pub async fn get_stations_query_option(cx: Scope, query: String) -> Option<Vec<Station>> {
    get_stations_query(cx, query).await.ok()
}
