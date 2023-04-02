mod blood_card;
mod blood_cards;
mod station_title;

use self::blood_cards::*;
use self::station_title::*;
use crate::actions::station::get_station_data_option;
use leptos::*;
use leptos_router::*;

#[component]
pub fn StationPage(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let station = create_resource(
        cx,
        move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
        move |id| get_station_data_option(cx, id),
    );

    view! { cx,
        <div class="h-full w-full bg-gray-100">
            <Transition fallback=move || {
                view! { cx, <div>"Loading..."</div> }
            }>
                {station
                    .read(cx)
                    .map(|station| match station {
                        Some(station) => {
                            view! { cx,
                                <title>{&station.name}</title>
                                <h1 class="text-black text-4xl mt-36 px-20 text-center">
                                    <StationTitle station=station.clone()/>
                                </h1>
                                <section class="flex justify-center">
                                    <BloodCards station=station.clone() rhd_plus=true/>
                                    <BloodCards station=station rhd_plus=false/>
                                </section>
                            }
                                .into_view(cx)
                        }
                        None => {
                            {
                                view! { cx, <div>"Couldn't load info for this station."</div> }
                            }
                                .into_view(cx)
                        }
                    })}
            </Transition>
        </div>
    }
}
