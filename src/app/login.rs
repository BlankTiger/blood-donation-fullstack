use crate::auth::Login;
use leptos::*;
use leptos_router::*;
use crate::app::notification::*;

#[component]
pub fn Login(cx: Scope, action: Action<Login, Result<(), ServerFnError>>) -> impl IntoView {
    let result_of_login = action.value();

    view! { cx,
        <title>"Zaloguj się"</title>
        <div class="h-full w-full overflow-hidden">
            <div class="min-h-screen bg-purple-400 flex justify-center items-center">
                <div class="absolute w-60 h-60 rounded-xl bg-purple-300 -top-42 -left-16 z-0 transform rotate-45 hidden md:block"></div>
                <div class="absolute w-48 h-48 rounded-xl bg-purple-300 -bottom-30 right-6 transform rotate-12 hidden md:block"></div>
                <div class="py-12 px-12 bg-white rounded-2xl shadow-xl z-20">
                    <ActionForm action=action>
                        <div>
                            <h1 class="text-3xl font-bold text-center mb-4 cursor-pointer">
                                "Stacja krwiodawstwa"
                            </h1>
                        </div>
                        <div class="space-y-4">
                            <input
                                type="text"
                                placeholder="Email Addres"
                                name="email"
                                class="block text-sm py-3 px-4 rounded-lg w-full border outline-none"
                            />
                            <input
                                type="password"
                                placeholder="Password"
                                name="password"
                                class="block text-sm py-3 px-4 rounded-lg w-full border outline-none appearance-none"
                            />
                        </div>
                        {move || match result_of_login() {
                            Some(Err(_)) => {
                                view! { cx,
                                    <div class="mt-2">
                                        <Notification
                                            msg="Niepoprawny login lub hasło".into()
                                            notification_type=NotificationType::Error
                                        />
                                    </div>
                                }
                                    .into_view(cx)
                            }
                            _ => {
                                view! { cx, <></> }
                                    .into_view(cx)
                            }
                        }}
                        <label>
                            <input type="checkbox" name="remember" class="auth-input"/>
                            " Zapamiętaj mnie"
                        </label>
                        <br/>
                        <div class="text-center mt-6">
                            <button
                                type="submit"
                                class="py-3 w-64 text-xl text-white bg-purple-400 rounded-2xl"
                            >
                                "Zaloguj"
                            </button>
                        </div>
                    </ActionForm>
                </div>
            </div>
        </div>
    }
}
