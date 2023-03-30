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
                        <h2 class="text-3xl font-bold sm:text-4xl">"Gdzie można oddać krew?"</h2>
                        <p class="mt-4 text-gray-600 text-justify">
                            "W Polsce pobór krwi jest zarezerwowany tylko dla placówek wchodzących w skład Publicznej Służby
                            Krwi. Do pobierania krwi i oddzielania jej składników uprawnione są wyłącznie: Regionalne Centra
                            Krwiodawstwa i Krwiolecznictwa, Wojskowe Centrum Krwiodawstwa i Krwiolecznictwa, Centrum
                            Krwiodawstwa i Krwiolecznictwa MSWiA (dawniej MSW) oraz ich oddziały i stacje terenowe. W Polsce
                            jest 21 RCKiK oraz wspomniane wyżej 2 centra resortowe. 16 z nich znajduje się w stolicach
                            województw kolejne 5 w miastach powiatowych (Słupsk, Kalisz, Radom, Wałbrzych, Racibórz). Oprócz
                            stałych punktów w postaci centrów oraz oddziałów terenowych krew pozyskiwana jest na akcjach
                            wyjazdowych (zarówno w mobilnych – krwiobusach jak i stałych punktach). Na poniższej mapie
                            zaznaczone zostały tylko stałe punkty poboru krwi. Natomiast harmonogramy akcji wyjazdowych
                            zamieściliśmy na samym dole strony."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
