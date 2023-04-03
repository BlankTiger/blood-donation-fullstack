use crate::actions::station::{get_stations_option, DeleteStation};
use crate::model::station::Station;
use crate::model::StationsResource;
use leptos::*;
use leptos_router::*;

#[component]
pub fn StationsTableAuthed(cx: Scope) -> impl IntoView {
    let stations: StationsResource =
        create_resource(cx, move || (), move |_| get_stations_option(cx));

    view! { cx,
        <h1 class="text-black text-4xl pt-20 px-20 text-center">
            "Registered blood donation stations"
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
                                "name"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "address"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "city"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                "phone"
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                ""
                            </th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">
                                ""
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
                                    "View"
                                </A>
                            </td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">
                                <ActionForm action=delete_station>
                                    <input type="hidden" name="station_id" value=station.id/>
                                    <button
                                        class="bg-blue-600 text-gray-200 p-2 rounded hover:bg-blue-500 hover:text-gray-100"
                                        type="submit"
                                    >
                                        "Delete"
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
