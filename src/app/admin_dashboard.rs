use leptos::*;
use crate::app::stations_table::*;

use super::UserResource;


#[component]
pub fn AdminDashboard(cx: Scope) -> impl IntoView {
    let user = use_context::<UserResource>(cx).expect("User resource to be present");

    view! {cx,
        <Suspense fallback=move || view! { cx, <h1>"fallback"</h1> }>
        {user.read(cx).map(move |user| match user {
            Some(_) => view! {cx,
                <div class="w-full mt-96">
                    <StationsTable />
                </div>
            },
            None => view! {cx,
                <div class="flex w-full items-center justify-center">
                    <h1 class="text-black text-4xl pt-20 px-20 text-center">
                        "You are not authorized to view this page."
                    </h1>
                </div>
            }
        }).into_view(cx)}
        </Suspense>
    }
}
