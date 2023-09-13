#[derive(Default, Clone)]
pub struct ItemMovementState {
    pub selected_item: Option<u32>,
    pub dragged_item: Option<String>,
}
