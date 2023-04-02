pub mod station;
pub mod user;
use crate::model::station::Station;
use crate::model::user::User;
use leptos::Resource;

pub type UserResource = Resource<(usize, usize, usize), Option<User>>;
pub type StationsResource = Resource<(), Option<Vec<Station>>>;
