use crate::auth::AuthGuardTwoViews;
use crate::auth::AuthGuardTwoViewsProps;
use cfg_if::cfg_if;
use leptos::{server_fn, *};
use leptos_router::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::{auth, pool};
        use crate::auth::User;
        use sqlx::FromRow;

        #[derive(FromRow, Clone, Debug)]
        pub struct SqlStation {
            id: i32,
            name: String,
            address: String,
            city: String,
            phone: String,
            #[sqlx(flatten)]
            available_blood: SqlAvailableBlood,
        }

        #[derive(FromRow, Clone, Debug)]
        pub struct SqlAvailableBlood {
            amount_a_plus: f32,
            amount_a_minus: f32,
            amount_b_plus: f32,
            amount_b_minus: f32,
            amount_ab_plus: f32,
            amount_ab_minus: f32,
            amount_o_plus: f32,
            amount_o_minus: f32,
        }

        impl From<SqlAvailableBlood> for AvailableBlood {
            fn from(sql_available_blood: SqlAvailableBlood) -> Self {
                AvailableBlood {
                    amount_a_plus: sql_available_blood.amount_a_plus,
                    amount_a_minus: sql_available_blood.amount_a_minus,
                    amount_b_plus: sql_available_blood.amount_b_plus,
                    amount_b_minus: sql_available_blood.amount_b_minus,
                    amount_ab_plus: sql_available_blood.amount_ab_plus,
                    amount_ab_minus: sql_available_blood.amount_ab_minus,
                    amount_o_plus: sql_available_blood.amount_o_plus,
                    amount_o_minus: sql_available_blood.amount_o_minus,
                }
            }
        }

        impl From<SqlStation> for Station {
            fn from(sql_station: SqlStation) -> Self {
                Station {
                    id: sql_station.id,
                    name: sql_station.name,
                    address: sql_station.address,
                    city: sql_station.city,
                    phone: sql_station.phone,
                    available_blood: sql_station.available_blood.into(),
                }
            }
        }
    }

}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AvailableBlood {
    pub amount_a_plus: f32,
    pub amount_a_minus: f32,
    pub amount_b_plus: f32,
    pub amount_b_minus: f32,
    pub amount_ab_plus: f32,
    pub amount_ab_minus: f32,
    pub amount_o_plus: f32,
    pub amount_o_minus: f32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Station {
    id: i32,
    pub name: String,
    pub address: String,
    pub city: String,
    pub phone: String,
    pub available_blood: AvailableBlood,
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

pub type StationsResource = Resource<(), Option<Vec<Station>>>;

async fn stations_option(cx: Scope) -> Option<Vec<Station>> {
    get_stations(cx).await.ok()
}

#[component]
pub fn StationsTable(cx: Scope) -> impl IntoView {
    view! { cx,
        <AuthGuardTwoViews
            view_authed=move || {
                view! { cx, <StationsTableAuthed/> }
                    .into_view(cx)
            }
            view_unauthed=move || {
                view! { cx, <StationsTableUnauthed/> }
                    .into_view(cx)
            }
        />
    }
}

fn stations_to_rows_authed(cx: Scope, stations: Vec<Station>) -> impl IntoView {
    let delete_station = create_server_action::<DeleteStation>(cx);
    let stations = move || stations.clone();

    view! { cx,
        <tbody class="divide-y divide-gray-200">
            <For
                each=stations
                key=|station| station.id
                view=move |cx, station: Station| {
                    view! { cx,
                        <tr>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-900">{station.id}</td>
                            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{station.name}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{station.address}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{station.city}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{station.phone}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                <A
                                    class="bg-blue-600 text-gray-200 p-2 rounded hover:bg-blue-500 hover:text-gray-100"
                                    href=format!("/station/{}", station.id)
                                >
                                    "Zobacz"
                                </A>
                            </td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                <ActionForm action=delete_station>
                                    <input type="hidden" name="station_id" value=station.id/>
                                    <button
                                        class="bg-blue-600 text-gray-200 p-2 rounded hover:bg-blue-500 hover:text-gray-100"
                                        type="submit"
                                    >
                                        "Usuń"
                                    </button>
                                </ActionForm>
                            </td>
                        </tr>
                    }
                }
            />
        </tbody>
    }
}

#[component]
fn StationsTableAuthed(cx: Scope) -> impl IntoView {
    let stations: StationsResource = create_resource(cx, move || (), move |_| stations_option(cx));

    view! { cx,
        <h1 class="text-black text-4xl pt-20 px-20 text-center">
            "Zarejestrowane stacje krwiodawstwa"
        </h1>
        <div class="flex justify-center">
            <div class="overflow-x-auto flex justify-center w-full border-gray-400 border-2 rounded-2xl mx-40 my-10">
                <table class="w-full mx-20 my-10 divide-y-2 divide-gray-200 text-sm">
                    <thead>
                        <tr>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "id"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "nazwa"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "adres"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "miejscowość"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "telefon"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "link"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "usuń"
                            </th>
                            <th class="px-4 py-2"></th>
                        </tr>
                    </thead>
                    <Suspense fallback=move || {
                        view! { cx, <></> }
                    }>
                        {stations
                            .read(cx)
                            .map(|stations| match stations {
                                Some(stations) => stations_to_rows_authed(cx, stations).into_view(cx),
                                None => {
                                    view! { cx, <h1 class="text-center">"Loading.."</h1> }
                                        .into_view(cx)
                                }
                            })
                            .into_view(cx)}
                    </Suspense>
                </table>
            </div>
        </div>
    }
}

fn stations_to_rows_unauthed(cx: Scope, stations: Vec<Station>) -> impl IntoView {
    let stations = move || stations.clone();

    view! { cx,
        <tbody class="divide-y divide-gray-200">
            <For
                each=stations
                key=|station| station.id
                view=move |cx, station: Station| {
                    view! { cx,
                        <tr>
                            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{station.name}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{station.address}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{station.city}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{station.phone}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                <A
                                    class="bg-blue-600 text-gray-200 p-2 rounded hover:bg-blue-500 hover:text-gray-100"
                                    href=format!("/station/{}", station.id)
                                >
                                    "Zobacz"
                                </A>
                            </td>
                        </tr>
                    }
                }
            />
        </tbody>
    }
}

#[component]
fn StationsTableUnauthed(cx: Scope) -> impl IntoView {
    let stations: StationsResource = create_resource(cx, move || (), move |_| stations_option(cx));

    view! { cx,
        <h1 class="text-black text-4xl pt-20 px-20 text-center">
            "Zarejestrowane stacje krwiodawstwa"
        </h1>
        <div class="flex justify-center">
            <div class="overflow-x-auto flex justify-center w-full border-gray-400 border-2 rounded-2xl mx-40 my-10">
                <table class="w-full mx-20 my-10 divide-y-2 divide-gray-200 text-sm">
                    <thead>
                        <tr>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "nazwa"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "adres"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "miejscowość"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "telefon"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "link"
                            </th>
                            <th class="px-4 py-2"></th>
                        </tr>
                    </thead>
                    <Suspense fallback=move || {
                        view! { cx, <></> }
                    }>
                        {stations
                            .read(cx)
                            .map(|stations| match stations {
                                Some(stations) => stations_to_rows_unauthed(cx, stations).into_view(cx),
                                None => {
                                    view! { cx, <h1 class="text-center">"Loading.."</h1> }
                                        .into_view(cx)
                                }
                            })
                            .into_view(cx)}
                    </Suspense>
                </table>
            </div>
        </div>
    }
}
