use super::form::{Form, FormProps};
use leptos::*;

#[component]
pub fn Authorized(cx: Scope) -> impl IntoView {
    view! { cx,
        <title>"Dodaj stację"</title>
        <section class="w-full h-full bg-gray-100">
            <div class="mx-auto max-w-screen-xl bg-gray-100">
                <div class="flex justify-center bg-gray-100">
                    <div class="w-1/2 mt-24 rounded-lg bg-white p-8 shadow-lg lg:p-12">
                        <h1 class="text-black text-4xl text-center">"Dodaj stację RCKiK"</h1>
                        <img class="my-10 rounded-2xl" src="blood_donation.jpg"/>
                        <Form/>
                    </div>
                </div>
            </div>
        </section>
    }
}
