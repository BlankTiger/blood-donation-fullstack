use crate::model::UserResource;
use leptos::*;

#[component]
pub fn AuthGuard<F, IV>(cx: Scope, view: F) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let user = use_context::<UserResource>(cx).expect("userresource to be provided");

    view! { cx,
        <Suspense fallback=move || {
            view! { cx, <div>"Loading..."</div> }
        }>
            {user
                .read(cx)
                .map(|user| match user {
                    Some(_) => view().into_view(cx),
                    None => {
                        view! { cx, <Unauthorized/> }
                            .into_view(cx)
                    }
                })}
        </Suspense>
    }
}

#[component]
pub fn AuthGuardTwoViews<F, G, IV>(cx: Scope, view_authed: F, view_unauthed: G) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    G: Fn() -> IV + 'static,
    IV: IntoView,
{
    let user = use_context::<UserResource>(cx).expect("userresource to be provided");

    view! { cx,
        <Suspense fallback=move || {
            view! { cx, <div>"Loading..."</div> }
        }>
            {user
                .read(cx)
                .map(|user| match user {
                    Some(_) => view_authed().into_view(cx),
                    None => view_unauthed().into_view(cx),
                })}
        </Suspense>
    }
}

#[component]
pub fn Unauthorized(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex w-full items-center justify-center">
            <h1 class="text-black text-4xl pt-20 px-20 text-center">
                "You are not authorized to view this page."
            </h1>
        </div>
    }
}
