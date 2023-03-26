use leptos::*;

#[component]
pub fn StationsTable(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <h1 class="text-black text-4xl pt-20 px-20 text-center">"Zarejestrowane stacje krwiodawstwa"</h1>
        <div class="flex justify-center">
            <div class="overflow-x-auto flex justify-center w-full border-gray-400 border-2 rounded-2xl mx-40 my-10">
                <table class="w-full mx-20 my-10 divide-y-2 divide-gray-200 text-sm">
                    <thead>
                        <tr>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">"RCKiK"</th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">"adres"</th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">"miejscowość"</th>
                            <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900">"telefon"</th>
                            <th class="px-4 py-2"></th>
                        </tr>
                    </thead>

                    <tbody class="divide-y divide-gray-200">
                        <tr>
                            <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900">"RCKiK przykład"</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">"ul. Wazów 42"</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">"65-046 Zielona Góra"</td>
                            <td class="whitespace-nowrap px-4 py-2 text-gray-700">"tel. (68) 329 83 60"</td>
                            <td class="whitespace-nowrap px-4 py-2">
                                <a href="przykladowa-stacja.html"
                                    class="inline-block rounded bg-indigo-600 px-4 py-2 text-xs font-medium text-white hover:bg-indigo-700">"Zobacz"</a>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
