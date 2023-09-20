use valence::prelude::*;
use std::collections::HashMap;

use val_electron_data::data::inventory_items::Item;

#[derive(Clone, Component)]
pub struct InventoryGUI {
    pub(crate) name: String,
    pub(crate) title: Text,
    pub(crate) kind: InventoryKind,
    pub(crate) items: HashMap<u32, Item>,
}
