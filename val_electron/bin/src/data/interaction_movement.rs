use crate::data::inventory_items::Item;

#[derive(Default, Clone)]
pub struct ItemMovementState {
    pub selected_item: Option<u32>,
    pub dragged_item: Option<Item>,
    pub dragged_from_inventory: bool,
    pub dragged_from_slot: u32,
}
