use leptos::*;
use leptos_router::*;
use crate::auth::AuthGuard;
use crate::auth::AuthGuardProps;
use crate::app::notification::*;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::app::{pool, auth};
    use crate::auth::User;
}}

#[server(UpdateAvailableBlood, "/api")]
pub async fn update_available_blood(
    cx: Scope,
    station_id: i32,
    amount_a_plus: f32,
    amount_a_minus: f32,
    amount_b_plus: f32,
    amount_b_minus: f32,
    amount_ab_plus: f32,
    amount_ab_minus: f32,
    amount_o_plus: f32,
    amount_o_minus: f32,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    if amount_a_plus < 0.0 || amount_a_minus < 0.0 || amount_b_plus < 0.0 || amount_b_minus < 0.0 || amount_ab_plus < 0.0 || amount_ab_minus < 0.0 || amount_o_plus < 0.0 || amount_o_minus < 0.0 {
        return Err(ServerFnError::ServerError("Invalid amount.".to_string()));
    }

    auth.current_user.ok_or(ServerFnError::ServerError(
        "User not logged in.".to_string(),
    ))?;

    sqlx::query(
        "update stations set amount_a_plus = ?, amount_a_minus = ?, amount_b_plus = ?, amount_b_minus = ?, amount_ab_plus = ?, amount_ab_minus = ?, amount_o_plus = ?, amount_o_minus = ? where id = ?")
        .bind(amount_a_plus)
        .bind(amount_a_minus)
        .bind(amount_b_plus)
        .bind(amount_b_minus)
        .bind(amount_ab_plus)
        .bind(amount_ab_minus)
        .bind(amount_o_plus)
        .bind(amount_o_minus)
        .bind(station_id)
        .execute(&pool)
        .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
}

#[component]
pub fn UpdateAvailableBlood(cx: Scope) -> impl IntoView {
    view! { cx,
        <AuthGuard view=move || {
            view! { cx, <Authorized/> }
        }/>
    }
}

#[component]
fn Authorized(cx: Scope) -> impl IntoView {
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
