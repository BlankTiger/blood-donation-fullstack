use cfg_if::cfg_if;
use leptos::{server_fn, *};
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
    amount_a_plus: f32,
    amount_a_minus: f32,
    amount_b_plus: f32,
    amount_b_minus: f32,
    amount_ab_plus: f32,
    amount_ab_minus: f32,
    amount_o_plus: f32,
    amount_o_minus: f32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Station {
    id: i32,
    name: String,
    address: String,
    city: String,
    phone: String,
    available_blood: AvailableBlood,
}

fn stations_to_rows(cx: Scope, stations: Vec<Station>) -> impl IntoView {
    let stations = move || stations.clone();

    view! {
        cx,
        <tbody class="divide-y divide-gray-200">
            <For
                each=stations
                key=|station| station.id
                view=move |cx, station: Station| {
                    view! {
                        cx,
                        <tr>
                            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">{station.name}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{station.address}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{station.city}</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">{station.phone}</td>
                        </tr>
                    }
            }/>
        </tbody>
    }
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
    let stations: StationsResource = create_resource(cx, || (), move |_| stations_option(cx));

    view! {
        cx,
        <h1 class="text-black text-4xl pt-20 px-20 text-center">"Zarejestrowane stacje krwiodawstwa"</h1>
        <div class="flex justify-center">
            <div class="overflow-x-auto flex justify-center w-full border-gray-400 border-2 rounded-2xl mx-40 my-10">
                <table class="w-full mx-20 my-10 divide-y-2 divide-gray-200 text-sm">
                    <thead>
                        <tr>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">"RCKiK"</th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">"adres"</th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">"miejscowość"</th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">"telefon"</th>
                            <th class="px-4 py-2"></th>
                        </tr>
                    </thead>
                <Suspense fallback=move || {view! {cx, <></>}}>
                    {stations.read(cx).map(|stations| match stations {
                            Some(stations) => stations_to_rows(cx, stations).into_view(cx),
                            None => view! {cx, <h1 class="text-center">"Loading.."</h1>}.into_view(cx)
                    }).into_view(cx)}
                </Suspense>
                </table>
            </div>
        </div>
    }
}
