use crate::app::logout::*;
use leptos::ev::MouseEvent;
use leptos::html::{Dialog, Input, Div};
use leptos::*;
use leptos_router::*;

use super::stations_table::{get_stations_query_option, Station};
use super::UserResource;

#[component]
pub fn Navbar(
    cx: Scope,
    logout_action: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    let user = use_context::<UserResource>(cx).expect("User resource to be present");
    let my_dialog = create_node_ref::<Dialog>(cx);
    let on_click = move |_| {
        if let Some(dialog) = my_dialog.get() {
            let _ = dialog.show_modal();
        }
    };

    view! { cx,
        <nav
            id="header"
            class="w-full z-30 py-1 bg-white shadow-lg border-b border-blue-400 fixed top-0"
        >
            <Transition fallback=move || {
                view! { cx, <span>"Loading..."</span> }
            }>
                <SearchDialog reference=my_dialog/>
                {user
                    .read(cx)
                    .map(|user| match user {
                        Some(user) => {
                            view! { cx,
                                <div class="w-full flex items-center justify-between mt-0 px-6 py-2">
                                    <div class="hidden md:flex md:items-center md:w-auto w-full" id="menu">
                                        <nav>
                                            <ul class="md:flex items-center justify-between text-base text-blue-600 pt-4 md:pt-0">
                                                <li>
                                                    <button
                                                        on:click=on_click
                                                        class="inline-block no-underline hover:text-black font-medium text-lg py-2 px-4 lg:-ml-2"
                                                    >
                                                        <svg
                                                            aria-hidden="true"
                                                            class="w-5 h-5 text-gray-500 dark:text-gray-400"
                                                            fill="currentColor"
                                                            viewBox="0 0 20 20"
                                                            xmlns="http://www.w3.org/2000/svg"
                                                        >
                                                            <path
                                                                fill-rule="evenodd"
                                                                d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
                                                                clip-rule="evenodd"
                                                            ></path>
                                                        </svg>
                                                    </button>
                                                </li>
                                                <li>
                                                    <A
                                                        class="inline-block no-underline hover:text-black font-medium text-lg py-2 px-4 lg:-ml-2"
                                                        href="/"
                                                    >
                                                        "Home"
                                                    </A>
                                                </li>
                                                <li>
                                                    <A
                                                        class="inline-block no-underline hover:text-black font-medium text-lg py-2 px-4 lg:-ml-2"
                                                        href="/admin"
                                                    >
                                                        "Dashboard"
                                                    </A>
                                                </li>
                                                <li>
                                                    <A
                                                        class="inline-block no-underline hover:text-black font-medium text-lg py-2 px-4 lg:-ml-2"
                                                        href="/add-station"
                                                    >
                                                        "Add station"
                                                    </A>
                                                </li>
                                                <li>
                                                    <A
                                                        class="inline-block no-underline hover:text-black font-medium text-lg py-2 px-4 lg:-ml-2"
                                                        href="/update-available-blood"
                                                    >
                                                        "Update available blood"
                                                    </A>
                                                </li>
                                            </ul>
                                        </nav>
                                    </div>
                                    <div
                                        class="order-2 md:order-3 flex flex-wrap items-center justify-end mr-0 md:mr-4"
                                        id="nav-content"
                                    >
                                        <div class="auth flex items-center w-full md:w-full">
                                            <span class="mx-10">{format!("Logged in as: {}", user.email)}</span>
                                            <Logout action=logout_action/>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                        None => {
                            view! { cx,
                                <div class="w-full flex items-center justify-between mt-0 px-6 py-2">
                                    <div
                                        class="hidden md:flex md:items-center md:w-auto w-full order-3 md:order-1"
                                        id="menu"
                                    >
                                        <nav>
                                            <ul class="md:flex items-center justify-between text-base text-blue-600 pt-4 md:pt-0">
                                                <li>
                                                    <button
                                                        on:click=on_click
                                                        class="inline-block no-underline hover:text-black font-medium text-lg py-2 px-4 lg:-ml-2"
                                                    >
                                                        <svg
                                                            aria-hidden="true"
                                                            class="w-5 h-5 text-gray-500 dark:text-gray-400"
                                                            fill="currentColor"
                                                            viewBox="0 0 20 20"
                                                            xmlns="http://www.w3.org/2000/svg"
                                                        >
                                                            <path
                                                                fill-rule="evenodd"
                                                                d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
                                                                clip-rule="evenodd"
                                                            ></path>
                                                        </svg>
                                                    </button>
                                                </li>
                                                <li>
                                                    <A
                                                        class="inline-block no-underline hover:text-black font-medium text-lg py-2 px-4 lg:-ml-2"
                                                        href="/"
                                                    >
                                                        "Home"
                                                    </A>
                                                </li>
                                            </ul>
                                        </nav>
                                    </div>
                                    <div
                                        class="order-2 md:order-3 flex flex-wrap items-center justify-end mr-0 md:mr-4"
                                        id="nav-content"
                                    >
                                        <div class="auth flex items-center w-full md:w-full">
                                            <A
                                                href="/login"
                                                class="bg-blue-600 text-gray-200 p-2 rounded hover:bg-blue-500 hover:text-gray-100"
                                            >
                                                "Zaloguj"
                                            </A>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    })
                    .into_view(cx)}
            </Transition>
        </nav>
    }
}

#[component]
fn SearchDialog(cx: Scope, reference: NodeRef<Dialog>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    let div_ref = create_node_ref::<Div>(cx);
    let on_dialog_click = move |_| {
        if let Some(dialog) = reference() {
            dialog.close();
        }
    };
    let on_div_click = move |e: MouseEvent| {
        if div_ref().is_some() {
            e.stop_propagation();
        }
    };
    let (query, set_query) = create_signal(cx, "".to_string());
    let on_change = move |_| {
        set_query(input_ref().unwrap().value());
    };

    view! { cx,
        <dialog _ref=reference on:click=on_dialog_click class="bg-transparent w-fit">
            <div class="w-lg mx-auto" on:click=on_div_click _ref=div_ref>
                <div class="relative flex mb-2 items-center w-full h-12 rounded-lg bg-white focus-within:shadow-lg overflow-hidden">
                    <div class="grid place-items-center h-full w-12 text-gray-300">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                            ></path>
                        </svg>
                    </div>
                    <input
                        on:change=on_change
                        _ref=input_ref
                        class="peer h-full w-full outline-none text-sm text-gray-700 pr-2"
                        type="text"
                        id="search"
                        placeholder="Search something.."
                    />
                </div>
                <div class="h-50 w-full bg-black-100">
                    <StationsList query_signal=query/>
                </div>
            </div>
        </dialog>
    }
}

#[component]
fn StationsList(cx: Scope, query_signal: ReadSignal<String>) -> impl IntoView {
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
                       "Brak wyników"
                   </div>
               }.into_view(cx);
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
