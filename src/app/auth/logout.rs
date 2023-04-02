use crate::auth::Logout;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Logout(cx: Scope, action: Action<Logout, Result<(), ServerFnError>>) -> impl IntoView {
    view! { cx,
        <div id="loginbox">
            <ActionForm action=action>
                <button
                    type="submit"
                    class="bg-blue-600 text-gray-200 p-2 rounded hover:bg-blue-500 hover:text-gray-100"
                >
                    "Wyloguj"
                </button>
            </ActionForm>
        </div>
    }
}
