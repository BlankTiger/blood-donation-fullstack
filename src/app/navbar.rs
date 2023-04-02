mod authorized_navbar;
mod search_dialog;
mod stations_list;
mod unauthorized_navbar;

use self::authorized_navbar::*;
use self::search_dialog::*;
use self::unauthorized_navbar::*;
use crate::auth::Logout;
use leptos::html::Dialog;
use leptos::*;

use crate::model::UserResource;

#[component]
pub fn Navbar(
    cx: Scope,
    logout_action: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    let user = use_context::<UserResource>(cx).expect("User resource to be present");
    let my_dialog = create_node_ref::<Dialog>(cx);
    let on_click = move |_| {
        if let Some(dialog) = my_dialog.get() {
            let _ = dialog.show_modal();
        }
    };

    view! { cx,
        <nav
            id="header"
            class="w-full z-30 py-1 bg-white shadow-lg border-b border-blue-400 fixed top-0"
        >
            <Transition fallback=move || {
                view! { cx, <span>"Loading..."</span> }
            }>
                <SearchDialog reference=my_dialog/>
                {user
                    .read(cx)
                    .map(|user| match user {
                        Some(user) => {
                            view! { cx, <AuthorizedNavbar user=user on_click=Box::new(on_click) logout_action=logout_action/> }
                                .into_view(cx)
                        }
                        None => {
                            view! { cx, <UnauthorizedNavbar on_click=Box::new(on_click)/> }
                                .into_view(cx)
                        }
                    })}
            </Transition>
        </nav>
    }
}
