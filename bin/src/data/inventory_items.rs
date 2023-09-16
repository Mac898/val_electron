use super::inventory_actions::ItemActions;

use minecraft_data_rs::models::item::Item as ItemMC;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub on_click: Option<ItemActions>,
}
impl From<ItemMC> for Item {
    fn from(item: ItemMC) -> Self {
        Self {
            id: item.name,
            .. Default::default()
        }
    }
}
