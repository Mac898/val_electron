use super::inventory_actions::ItemActions;

pub struct Item {
    pub id: String,
    pub on_click: Option<ItemActions>,
}

