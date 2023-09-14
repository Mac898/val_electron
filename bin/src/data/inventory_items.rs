use super::inventory_actions::ItemActions;

use minecraft_data_rs::models::item::Item as ItemMC;

#[derive(Clone, Default)]
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
