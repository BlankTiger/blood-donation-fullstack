use super::stations_list::*;
use leptos::ev::MouseEvent;
use leptos::html::*;
use leptos::*;

#[component]
pub fn SearchDialog(cx: Scope, reference: NodeRef<Dialog>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    let div_ref = create_node_ref::<Div>(cx);
    let on_dialog_click = move |_| {
        if let Some(dialog) = reference() {
            dialog.close();
        }
    };
    let on_div_click = move |e: MouseEvent| {
        if div_ref().is_some() {
            e.stop_propagation();
        }
    };
    let (query, set_query) = create_signal(cx, "".to_string());
    let on_change = move |_| {
        set_query(input_ref().unwrap().value());
    };

    view! { cx,
        <dialog _ref=reference on:click=on_dialog_click class="bg-transparent w-fit">
            <div class="w-lg mx-auto" on:click=on_div_click _ref=div_ref>
                <div class="relative flex mb-2 items-center w-full h-12 rounded-lg bg-white focus-within:shadow-lg overflow-hidden">
                    <div class="grid place-items-center h-full w-12 text-gray-300">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                            ></path>
                        </svg>
                    </div>
                    <input
                        on:change=on_change
                        _ref=input_ref
                        class="peer h-full w-full outline-none text-sm text-gray-700 pr-2"
                        type="text"
                        id="search"
                        placeholder="Search something.."
                    />
                </div>
                <div class="h-50 w-full bg-black-100">
                    <StationsList query_signal=query/>
                </div>
            </div>
        </dialog>
    }
}
