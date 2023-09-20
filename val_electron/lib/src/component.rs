use bevy::prelude::*;
use std::collections::HashMap;

use val_electron_data::data::inventory_actions::ItemActions;

#[derive(Clone, Component)]
pub struct InventoryGUI {
    pub(crate) name: String,
    pub(crate) slot_actions: HashMap<u32, ItemActions>,
}
