use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, EnumVariantNames};
// ====================================================
// Location
// https://serde.rs/enum-representations.html
// ====================================================
#[derive(
    EnumVariantNames, EnumString, EnumIter, Display, Debug, Serialize, Deserialize, Clone, PartialEq,
)]
#[serde(tag = "location")]
pub enum Location {
    #[strum(serialize = "Living Room")]
    LivingRoom,
    #[strum(serialize = "Kitchen")]
    Kitchen,
    #[strum(serialize = "Dining Room")]
    DiningRoom,
    #[strum(serialize = "Bedroom")]
    Bedroom,
}
