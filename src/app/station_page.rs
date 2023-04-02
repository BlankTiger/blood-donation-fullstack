use crate::app::stations_table::get_station_data_option;
use crate::auth::AuthGuardTwoViews;
use crate::auth::AuthGuardTwoViewsProps;
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
                            let station_cpy = station.clone();
                            view! { cx,
                                <title>{&station.name}</title>
                                <h1 class="text-black text-4xl mt-36 px-20 text-center">
                                    <AuthGuardTwoViews
                                        view_authed=move || {
                                            view! { cx, {format!("Stan krwi w {}, id={}", station.name, station.id)} }
                                                .into_view(cx)
                                        }
                                        view_unauthed=move || {
                                            view! { cx, {format!("Stan krwi w {}", station_cpy.name)} }
                                                .into_view(cx)
                                        }
                                    />
                                </h1>
                                <section class="flex justify-center">
                                    <div class="flex flex-col justify-center items-center w-1/2 m-20 p-5">
                                        <h2 class="text-left font-semibold text-dark text-xl sm:text-[22px] md:text-xl lg:text-[22px] xl:text-xl 2xl:text-[22px] mb-4 block hover:text-primary">
                                            "
                                            Krew RhD (+) plus"
                                        </h2>
                                        <div class="flex flex-row w-full space-x-4">
                                            <BloodCard
                                                title="A".to_string()
                                                value=station.available_blood.amount_a_plus
                                            />
                                            <BloodCard
                                                title="B".to_string()
                                                value=station.available_blood.amount_b_plus
                                            />
                                            <BloodCard
                                                title="AB".to_string()
                                                value=station.available_blood.amount_ab_plus
                                            />
                                            <BloodCard
                                                title="O".to_string()
                                                value=station.available_blood.amount_o_plus
                                            />
                                        </div>
                                    </div>
                                    <div class="flex flex-col justify-center items-center w-1/2 m-20 p-5">
                                        <h2 class="text-left font-semibold text-dark text-xl sm:text-[22px] md:text-xl lg:text-[22px] xl:text-xl 2xl:text-[22px] mb-4 block hover:text-primary">
                                            "
                                            Krew RhD (-) minus"
                                        </h2>
                                        <div class="flex flex-row w-full space-x-4">
                                            <BloodCard
                                                title="A".to_string()
                                                value=station.available_blood.amount_a_minus
                                            />
                                            <BloodCard
                                                title="B".to_string()
                                                value=station.available_blood.amount_b_minus
                                            />
                                            <BloodCard
                                                title="AB".to_string()
                                                value=station.available_blood.amount_ab_minus
                                            />
                                            <BloodCard
                                                title="O".to_string()
                                                value=station.available_blood.amount_o_minus
                                            />
                                        </div>
                                    </div>
                                </section>
                            }
                                .into_view(cx)
                        }
                        None => {
                            {
                                view! { cx, <div>"Station not found"</div> }
                            }
                                .into_view(cx)
                        }
                    })}
            </Transition>
        </div>
    }
}

#[component]
fn BloodCard(cx: Scope, title: String, value: f32) -> impl IntoView {
    let percent = f32::min(100.0, value) as i32;
    let css_selector_left = format!("level-left{}{}", title, percent);
    let css_selector_right = format!("level-right{}{}", title, percent);
    view! { cx,
        <div class="w-full md:w-1/4 xl:w-1/8">
            <div class="rounded-2xl bg-white overflow-hidden hover:shadow-lg transition-all delay-75">
                <div class="pt-1 text-center bg-gray-200">
                    <h3>
                        <p class="font-semibold text-dark text-xl sm:text-[22px] md:text-xl lg:text-[22px] xl:text-xl 2xl:text-[22px] mb-4 block hover:text-primary ">
                            {format!("{} / 100 l", value as i32)}
                        </p>
                    </h3>
                </div>
                <svg
                    class="py-10"
                    version="1.1"
                    id="Layer_1"
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
                    viewBox="0 0 512 512"
                    xml:space="preserve"
                >
                    <g>
                        <defs>
                            <linearGradient id=&css_selector_left x1="0" y1="1" x2="0" y2="0">
                                <stop
                                    id="stop1"
                                    offset=format!("{}%", percent)
                                    stop-color="#D80027"
                                ></stop>
                                <stop
                                    id="stop2"
                                    offset=format!("{}%", percent)
                                    stop-color="transparent"
                                ></stop>
                            </linearGradient>
                            <linearGradient id=&css_selector_right x1="0" y1="1" x2="0" y2="0">
                                <stop
                                    id="stop1"
                                    offset=format!("{}%", percent)
                                    stop-color="#A2001D"
                                ></stop>
                                <stop
                                    id="stop2"
                                    offset=format!("{}%", percent)
                                    stop-color="transparent"
                                ></stop>
                            </linearGradient>
                        </defs>
                        <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                        <g
                            id="SVGRepo_tracerCarrier"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        ></g>
                        <g id="SVGRepo_iconCarrier">
                            <path
                                fill=format!("url(#{})", & css_selector_left)
                                d="M450.207,317.793C450.207,425.05,363.256,512,256,512c-107.257,0-194.207-86.95-194.207-194.207 C61.793,176.552,256,0,256,0S450.207,176.552,450.207,317.793z"
                            ></path>
                            <path
                                fill=format!("url(#{})", & css_selector_right)
                                d="M450.207,317.793C450.207,425.05,363.256,512,256,512c0-211.862,0-512,0-512 S450.207,176.552,450.207,317.793z"
                            ></path>
                            <polygon
                                style="fill:#dddddd;"
                                points="336.92,285.425 288.368,285.425 288.368,236.873 223.632,236.873 223.632,285.425 175.08,285.425 175.08,350.161 223.632,350.161 223.632,398.712 288.368,398.712 288.368,350.161 336.92,350.161 "
                            ></polygon>
                        </g>
                    </g>
                </svg>
                <div class="p-4 text-center bg-gray-200">
                    <h3>
                        <p class="font-semibold text-dark text-xl sm:text-[22px] md:text-xl lg:text-[22px] xl:text-xl 2xl:text-[22px] mb-4 block hover:text-primary ">
                            {title}
                        </p>
                    </h3>
                </div>
            </div>
        </div>
    }
}
