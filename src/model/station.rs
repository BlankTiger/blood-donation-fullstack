use cfg_if::cfg_if;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AvailableBlood {
    pub amount_a_plus: f32,
    pub amount_a_minus: f32,
    pub amount_b_plus: f32,
    pub amount_b_minus: f32,
    pub amount_ab_plus: f32,
    pub amount_ab_minus: f32,
    pub amount_o_plus: f32,
    pub amount_o_minus: f32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Station {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub city: String,
    pub phone: String,
    pub available_blood: AvailableBlood,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::FromRow;

        #[derive(FromRow, Clone, Debug)]
        pub struct SqlStation {
            id: i32,
            name: String,
            address: String,
            city: String,
            phone: String,
            #[sqlx(flatten)]
            available_blood: SqlAvailableBlood,
        }

        #[derive(FromRow, Clone, Debug)]
        pub struct SqlAvailableBlood {
            amount_a_plus: f32,
            amount_a_minus: f32,
            amount_b_plus: f32,
            amount_b_minus: f32,
            amount_ab_plus: f32,
            amount_ab_minus: f32,
            amount_o_plus: f32,
            amount_o_minus: f32,
        }

        impl From<SqlAvailableBlood> for AvailableBlood {
            fn from(sql_available_blood: SqlAvailableBlood) -> Self {
                AvailableBlood {
                    amount_a_plus: sql_available_blood.amount_a_plus,
                    amount_a_minus: sql_available_blood.amount_a_minus,
                    amount_b_plus: sql_available_blood.amount_b_plus,
                    amount_b_minus: sql_available_blood.amount_b_minus,
                    amount_ab_plus: sql_available_blood.amount_ab_plus,
                    amount_ab_minus: sql_available_blood.amount_ab_minus,
                    amount_o_plus: sql_available_blood.amount_o_plus,
                    amount_o_minus: sql_available_blood.amount_o_minus,
                }
            }
        }

        impl From<SqlStation> for Station {
            fn from(sql_station: SqlStation) -> Self {
                Station {
                    id: sql_station.id,
                    name: sql_station.name,
                    address: sql_station.address,
                    city: sql_station.city,
                    phone: sql_station.phone,
                    available_blood: sql_station.available_blood.into(),
                }
            }
        }
    }
}
