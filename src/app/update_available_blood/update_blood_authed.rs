use crate::actions::blood::*;
use crate::app::notification::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Authorized(cx: Scope) -> impl IntoView {
    let update_available_blood = create_server_action::<UpdateAvailableBlood>(cx);
    let last_result = update_available_blood.value();

    view! { cx,
        <title>"Zaktualizuj stan krwi"</title>
        <section class="w-full bg-gray-100">
            <div class="mx-auto max-w-screen-xl px-4 py-16 sm:px-6 lg:px-8">
                <div class="flex justify-center">
                    <div class="mt-16 w-1/2 rounded-lg bg-white p-8 shadow-lg lg:p-12">
                        <h1 class="text-black text-4xl text-center">
                            "Aktualizuj stan krwi w stacji"
                        </h1>
                        <img class="my-10 rounded-2xl" src="blood_donation_3.jpg"/>
                        <ActionForm action=update_available_blood>
                            <div class="flex flex-row justify-center w-full space-x-4">
                                <div class="flex justify-center w-1/2">
                                    <label class="sr-only" for="station_id">
                                        "ID stacji"
                                    </label>
                                    <input
                                        class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                        placeholder="ID stacji"
                                        type="text"
                                        name="station_id"
                                    />
                                </div>
                                <div class="flex flex-col space-y-4 w-1/2">
                                    <div>
                                        <label class="sr-only" for="A RhD+">
                                            "A RhD+"
                                        </label>
                                        <input
                                            class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                            placeholder="A RhD+"
                                            type="text"
                                            name="amount_a_plus"
                                        />
                                    </div>
                                    <div>
                                        <label class="sr-only" for="B RhD+">
                                            "B RhD+"
                                        </label>
                                        <input
                                            class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                            placeholder="B RhD+"
                                            type="text"
                                            name="amount_b_plus"
                                        />
                                    </div>
                                    <div>
                                        <label class="sr-only" for="AB RhD+">
                                            "AB RhD+"
                                        </label>
                                        <input
                                            class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                            placeholder="AB RhD+"
                                            type="text"
                                            name="amount_ab_plus"
                                        />
                                    </div>
                                    <div>
                                        <label class="sr-only" for="O RhD+">
                                            "O RhD+"
                                        </label>
                                        <input
                                            class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                            placeholder="O RhD+"
                                            type="text"
                                            name="amount_o_plus"
                                        />
                                    </div>
                                </div>
                                <div class="flex flex-col space-y-4 w-1/2">
                                    <div>
                                        <label class="sr-only" for="A RhD-">
                                            "A RhD-"
                                        </label>
                                        <input
                                            class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                            placeholder="A RhD-"
                                            type="text"
                                            name="amount_a_minus"
                                        />
                                    </div>
                                    <div>
                                        <label class="sr-only" for="B RhD-">
                                            "B RhD-"
                                        </label>
                                        <input
                                            class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                            placeholder="B RhD-"
                                            type="text"
                                            name="amount_b_minus"
                                        />
                                    </div>
                                    <div>
                                        <label class="sr-only" for="AB RhD-">
                                            "AB RhD-"
                                        </label>
                                        <input
                                            class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                            placeholder="AB RhD-"
                                            type="text"
                                            name="amount_ab_minus"
                                        />
                                    </div>
                                    <div>
                                        <label class="sr-only" for="O RhD-">
                                            "O RhD-"
                                        </label>
                                        <input
                                            class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                            placeholder="O RhD-"
                                            type="text"
                                            name="amount_o_minus"
                                        />
                                    </div>
                                </div>
                            </div>
                            {move || match last_result() {
                                Some(Err(_)) => {
                                    view! { cx,
                                        <Notification
                                            msg="Podano niewłaściwe dane.".into()
                                            notification_type=NotificationType::Error
                                        />
                                    }
                                        .into_view(cx)
                                }
                                Some(Ok(_)) => {
                                    view! { cx,
                                        <Notification
                                            msg="Stan krwi został zaktualizowany.".into()
                                            notification_type=NotificationType::Info
                                        />
                                    }
                                        .into_view(cx)
                                }
                                _ => {
                                    view! { cx, <></> }
                                        .into_view(cx)
                                }
                            }}
                            <div class="mt-4 flex justify-center">
                                <button
                                    type="submit"
                                    class="inline-block w-full rounded-lg bg-black px-5 py-3 font-medium text-white sm:w-auto"
                                >
                                    "Aktualizuj"
                                </button>
                            </div>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </section>
    }
}
