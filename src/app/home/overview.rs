use leptos::*;

#[component]
pub fn Overview(cx: Scope) -> impl IntoView {
    view! { cx,
        <section>
            <div class="mx-auto max-w-screen-xl px-4 py-8 sm:py-12 sm:px-6 lg:py-16 lg:px-8">
                <div class="grid grid-cols-1 gap-8 lg:grid-cols-2 lg:gap-16">
                    <div class="relative h-64 overflow-hidden rounded-lg sm:h-80 lg:order-last lg:h-full">
                        <img
                            alt="blood donation hero image"
                            src="/blood_donation_2.jpg"
                            class="absolute inset-0 h-full w-full object-cover"
                        />
                    </div>
                    <div class="lg:py-24">
                        <h2 class="text-3xl font-bold sm:text-4xl">"Where can I donate blood?"</h2>
                        <p class="mt-4 text-gray-600 text-justify">
                             "In Poland, blood collection is reserved only for facilities that are part of the Public Blood
                            Service. Blood Service. Only the following are authorized to collect blood and separate its
                            components: Regional Centers Blood Donation and Hemotherapy, the Military Center for Blood Donation
                            and Hemotherapy, the Center for Blood Donation and Hemotherapy of the Ministry of Internal Affairs
                            and Administration (formerly the Ministry of the Interior) and their branches and field stations.
                            In Poland there are 21 RCKiKs and the aforementioned 2 departmental centers. 16 of them are located
                            in the capitals of provinces another 5 in district cities (Slupsk, Kalisz, Radom, Walbrzych,
                            Raciborz). In addition to permanent points in the form of centers and field branches, blood is
                            obtained at campaigns away (both in mobile - bloodbuses and permanent points). On the map below
                            only the permanent blood collection points are marked. However, the schedules of the exit actions
                            have been posted at the very bottom of the page."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
