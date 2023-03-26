use crate::{app::logout::*, auth::User};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar(
    cx: Scope,
    logged_in_user: Signal<Option<User>>,
    logout_action: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <nav
            id="header"
            class="w-full z-30 py-1 bg-white shadow-lg border-b border-blue-400 fixed top-0"
        >
            <Transition
                fallback=move || view! {cx, <span>"Loading..."</span>}
            >
            {
            move || {
                if let Some(user) = logged_in_user() {
                    view! {
                        cx,
                        <div class="w-full flex items-center justify-between mt-0 px-6 py-2">
                            <div class="hidden md:flex md:items-center md:w-auto w-full" id="menu">
                                <nav>
                                    <ul class="md:flex items-center justify-between text-base text-blue-600 pt-4 md:pt-0">
                                        <li>
                                            <A class="inline-block no-underline hover:text-black font-medium text-lg py-2 px-4 lg:-ml-2"
                                                href="/add_station">
                                                "Dodaj stację"
                                            </A>
                                        </li>
                                        <li>
                                            <A class="inline-block no-underline hover:text-black font-medium text-lg py-2 px-4 lg:-ml-2" href="/settings">"Ustawienia"</A>
                                        </li>
                                    </ul>
                                </nav>
                            </div>


                            <div class="order-2 md:order-3 flex flex-wrap items-center justify-end mr-0 md:mr-4" id="nav-content">
                                <div class="auth flex items-center w-full md:w-full">
                                    <span class="mx-10">{format!("Logged in as: {}", user.email)}</span>
                                    <Logout action=logout_action />
                                </div>
                            </div>
                        </div>
                    }.into_view(cx)
                } else {
                        view! {
                        cx,
                        <div class="w-full flex items-center justify-between mt-0 px-6 py-2">
                            <div class="hidden md:flex md:items-center md:w-auto w-full order-3 md:order-1" id="menu">
                                <nav>
                                    <ul class="md:flex items-center justify-between text-base text-blue-600 pt-4 md:pt-0">
                                        <li>
                                        </li>
                                    </ul>
                                </nav>
                            </div>
                            <div class="order-2 md:order-3 flex flex-wrap items-center justify-end mr-0 md:mr-4" id="nav-content">
                                <div class="auth flex items-center w-full md:w-full">
                                    <A href="/login"
                            class="bg-blue-600 text-gray-200 p-2 rounded hover:bg-blue-500 hover:text-gray-100">"Zaloguj"</A>
                                </div>
                            </div>
                        </div>
                    }.into_view(cx)
                }
            }}
            </Transition>
        </nav>
    }
}
