use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::data::inventory_items::Item;
use crate::data::inventory_type::{InventoryType, SmallChestType};

pub mod interaction_movement;
pub mod inventory_type;
pub mod inventory_items;
pub mod inventory_actions;

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub slots: HashMap<u32, Item>,
    pub name: String,
    pub gui_name: String,
}
impl Default for Inventory {
    fn default() -> Self {
        let default_type = SmallChestType::default();
        Self {
            slots: HashMap::with_capacity(100),
            name: default_type.default_name(),
            gui_name: "".to_string(),
        }
    }
}
