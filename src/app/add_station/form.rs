use crate::actions::station::AddStation;
use crate::app::notification::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Form(cx: Scope) -> impl IntoView {
    let add_station = create_server_action::<AddStation>(cx);
    let last_result = add_station.value();

    view! { cx,
        <ActionForm action=add_station class="space-y-4">
            <div>
                <label class="sr-only" for="name">
                    "Nazwa"
                </label>
                <input
                    class="w-full rounded-lg border-gray-200 p-3 text-sm"
                    placeholder="Nazwa RCKiK"
                    type="text"
                    name="name"
                />
            </div>
            <div>
                <label class="sr-only" for="address">
                    "Adres"
                </label>
                <input
                    class="w-full rounded-lg border-gray-200 p-3 text-sm"
                    placeholder="Adres"
                    type="text"
                    name="address"
                />
            </div>
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                <div>
                    <label class="sr-only" for="city">
                        "Miasto"
                    </label>
                    <input
                        class="w-full rounded-lg border-gray-200 p-3 text-sm"
                        placeholder="Miejscowość"
                        type="text"
                        name="city"
                    />
                </div>
                <div>
                    <label class="sr-only" for="phone">
                        "Telefon"
                    </label>
                    <input
                        class="w-full rounded-lg border-gray-200 p-3 text-sm"
                        placeholder="Numer telefonu"
                        type="tel"
                        name="phone"
                    />
                </div>
            </div>
            {move || match last_result() {
                Some(Ok(_)) => {
                    view! { cx, <Notification msg="Stacja dodana!".into() notification_type=NotificationType::Info/> }
                        .into_view(cx)
                }
                Some(Err(e)) => {
                    let e_msg = e.to_string();
                    let msg = e_msg.split(": ").last().unwrap_or("");
                    view! { cx, <Notification msg=format!("Oops! {msg}") notification_type=NotificationType::Error/> }
                        .into_view(cx)
                }
                None => {
                    view! { cx, <></> }
                        .into_view(cx)
                }
            }}
            <div class="mt-4 flex justify-center">
                <button
                    type="submit"
                    class="inline-block w-full rounded-lg bg-black px-5 py-3 font-medium text-white sm:w-auto"
                >
                    "Dodaj stację"
                </button>
            </div>
        </ActionForm>
    }
}
