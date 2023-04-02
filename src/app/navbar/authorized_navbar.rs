use crate::app::auth::logout::*;
use crate::auth::*;
use crate::model::user::User;
use leptos::ev::MouseEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn AuthorizedNavbar(
    cx: Scope,
    user: User,
    on_click: Box<dyn FnMut(MouseEvent)>,
    logout_action: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
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
                    <span class="mx-10">{format!("Logged in as: {}", & user.email)}</span>
                    <Logout action=logout_action/>
                </div>
            </div>
        </div>
    }
}
