use super::blood_card::*;
use crate::model::station::Station;
use leptos::*;

#[component]
pub fn BloodCards(cx: Scope, station: Station, rhd_plus: bool) -> impl IntoView {
    let title = if rhd_plus {
        "Krew RhD (+) plus"
    } else {
        "Krew RhD (-) minus"
    };

    view! { cx,
        <div class="flex flex-col justify-center items-center w-1/2 m-20 p-5">
            <h2 class="text-left font-semibold text-dark text-xl sm:text-[22px] md:text-xl lg:text-[22px] xl:text-xl 2xl:text-[22px] mb-4 block hover:text-primary">
                {title}
            </h2>
            <div class="flex flex-row w-full space-x-4">
                <BloodCard
                    title="A".to_string()
                    value=if rhd_plus {
                        station.available_blood.amount_a_plus
                    } else {
                        station.available_blood.amount_a_minus
                    }
                />
                <BloodCard
                    title="B".to_string()
                    value=if rhd_plus {
                        station.available_blood.amount_b_plus
                    } else {
                        station.available_blood.amount_b_minus
                    }
                />
                <BloodCard
                    title="AB".to_string()
                    value=if rhd_plus {
                        station.available_blood.amount_ab_plus
                    } else {
                        station.available_blood.amount_ab_minus
                    }
                />
                <BloodCard
                    title="O".to_string()
                    value=if rhd_plus {
                        station.available_blood.amount_o_plus
                    } else {
                        station.available_blood.amount_o_minus
                    }
                />
            </div>
        </div>
    }
}
