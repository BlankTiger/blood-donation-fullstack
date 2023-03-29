use leptos::*;
use cfg_if::cfg_if;
use crate::auth::AuthGuard;
use crate::auth::AuthGuardProps;
use leptos_router::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::app::{pool, auth};
    use crate::auth::User;
}}

#[component]
pub fn AddStation(cx: Scope) -> impl IntoView {
    view! { cx, <AuthGuard view=move || view! {cx, <Authorized /> } /> }
}

#[component]
fn Authorized(cx: Scope) -> impl IntoView {
    let add_station = create_server_action::<AddStation>(cx);

    view! { cx,
        <section class="w-full h-full bg-gray-100">
            <div class="mx-auto max-w-screen-xl bg-gray-100">
                <div class="flex justify-center bg-gray-100">

                    <div class="w-1/2 mt-24 rounded-lg bg-white p-8 shadow-lg lg:p-12">
                        <h1 class="text-black text-4xl text-center">"Dodaj stację RCKiK"</h1>
                        <img class="my-10 rounded-2xl" src="blood_donation.jpg" />
                        <ActionForm action=add_station class="space-y-4">
                            <div>
                                <label class="sr-only" for="name">"Nazwa"</label>
                                <input class="w-full rounded-lg border-gray-200 p-3 text-sm" placeholder="Nazwa RCKiK"
                                    type="text" name="name" />
                            </div>

                            <div>
                                <label class="sr-only" for="address">"Adres"</label>
                                <input class="w-full rounded-lg border-gray-200 p-3 text-sm" placeholder="Adres" type="text"
                                    name="address" />
                            </div>

                            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                                <div>
                                    <label class="sr-only" for="city">"Miasto"</label>
                                    <input class="w-full rounded-lg border-gray-200 p-3 text-sm" placeholder="Miejscowość"
                                        type="text" name="city" />
                                </div>

                                <div>
                                    <label class="sr-only" for="phone">"Telefon"</label>
                                    <input class="w-full rounded-lg border-gray-200 p-3 text-sm"
                                        placeholder="Numer telefonu" type="tel" name="phone" />
                                </div>
                            </div>

                            <div class="mt-4 flex justify-center">
                                <button type="submit"
                                    class="inline-block w-full rounded-lg bg-black px-5 py-3 font-medium text-white sm:w-auto">"Dodaj stację"</button>
                            </div>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[server(AddStation, "/api")]
pub async fn add_station(
    cx: Scope,
    name: String,
    address: String,
    city: String,
    phone: String,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;
    dbg!(&name, &address, &city, &phone);

    auth.current_user.ok_or(ServerFnError::ServerError(
        "User not logged in.".to_string(),
    ))?;

    sqlx::query(
        "INSERT INTO stations (name, address, city, phone, amount_a_plus, amount_a_minus, amount_b_plus, amount_b_minus, amount_ab_plus, amount_ab_minus, amount_o_plus, amount_o_minus)
        VALUES (?, ?, ?, ?, 0, 0, 0, 0, 0, 0, 0, 0)")
        .bind(name)
        .bind(address)
        .bind(city)
        .bind(phone)
        .execute(&pool)
        .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
}

