use crate::app::logout::*;
use leptos::*;
use leptos_router::*;

use super::UserResource;

#[component]
pub fn Navbar(
    cx: Scope,
    logout_action: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    let user = use_context::<UserResource>(cx).expect("User resource to be present");

    view! { cx,
        <nav
            id="header"
            class="w-full z-30 py-1 bg-white shadow-lg border-b border-blue-400 fixed top-0"
        >
            <Transition fallback=move || {
                view! { cx, <span>"Loading..."</span> }
            }>
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
                                                <li></li>
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
