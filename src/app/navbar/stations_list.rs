use crate::actions::station::get_stations_query_option;
use crate::model::station::Station;
use leptos::*;
use leptos_router::*;

#[component]
pub fn StationsList(cx: Scope, query_signal: ReadSignal<String>) -> impl IntoView {
    let stations = create_resource(
        cx,
        move || query_signal(),
        move |query| get_stations_query_option(cx, query),
    );

    view! { cx,
        <Transition fallback=move || {
            view! { cx, <></> }
        }>
            {stations
                .read(cx)
                .map(|stations| match stations {
                    Some(stations) => {
                        log!("{}", query_signal());
                        stations_to_li(cx, stations).into_view(cx)
                    }
                    None => {
                        view! { cx, <></> }
                            .into_view(cx)
                    }
                })}
        </Transition>
    }
}

fn stations_to_li(cx: Scope, stations: Vec<Station>) -> impl IntoView {
    let stations = move || stations.clone();

    if stations().is_empty() {
        return view! { cx,
                   <div class="text-center text-gray-500 rounded py-2 border-gray-200 bg-white">
                       "No stations found.."
                   </div>
               }
        .into_view(cx);
    }

    view! { cx,
        <ul class="border border-gray-200 rounded overflow-hidden shadow-md">
            <For
                each=stations
                key=|station| station.id
                view=move |cx, station: Station| {
                    view! { cx,
                        <A href=format!("/station/{}", station.id)>
                            <li class="px-4 py-2 bg-white hover:bg-sky-100 hover:text-sky-900 border-b last:border-none border-gray-200 transition-all duration-300 ease-in-out">
                                {format!("{} - {}, {}, {}", station.name, station.city, station.address, station.phone)
                                    .to_string()}
                            </li>
                        </A>
                    }
                }
            />
        </ul>
    }.into_view(cx)
}
