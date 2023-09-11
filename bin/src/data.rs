pub mod interaction_movement;
pub mod inventory_type;
pub mod inventory_items;
pub mod inventory_actions;

pub struct GenericChestInventoryKind {
    pub rows: u32,
    pub cols: u32,
}
impl Default for GenericChestInventoryKind {
    fn default() -> Self {
        GenericChestInventoryKind {
            rows: 3,
            cols: 9,
        }
    }
}
impl InventoryKind for GenericChestInventoryKind {
    fn max_cols(&self) -> u32 {
        9
    }

    fn max_rows(&self) -> u32 {
        6
    }

    fn num_rows(&self) -> u32 {
        return self.rows.clone()
    }

    fn num_cols(&self) -> u32 {
        return self.cols.clone()
    }

    fn set_rows(&mut self, rows: u32) {
        if rows > 0 && rows <= self.max_rows() {
           self.rows = rows;
        }
    }

    fn set_cols(&mut self, cols: u32) {
        if cols > 0 && cols <= self.max_cols() {
            self.cols = cols;
        }
    }

    fn default_name(&self) -> &'static str {
        "Chest"
    }
}

#[derive(Default)]
pub struct HopperInventoryKind;
impl InventoryKind for HopperInventoryKind {
    fn max_cols(&self) -> u32 {
        5
    }

    fn max_rows(&self) -> u32 {
        1
    }

    fn num_rows(&self) -> u32 {
        1
    }

    fn num_cols(&self) -> u32 {
        5
    }

    fn set_rows(&mut self, _: u32) {
        ()
    }

    fn set_cols(&mut self, _: u32) {
        ()
    }

    fn default_name(&self) -> &'static str {
        "Hopper"
    }
}

#[derive(Default)]
pub struct DispenserInventoryKind;
impl InventoryKind for DispenserInventoryKind {
    fn max_cols(&self) -> u32 {
        3
    }

    fn max_rows(&self) -> u32 {
        3
    }

    fn num_rows(&self) -> u32 {
        3
    }

    fn num_cols(&self) -> u32 {
        3
    }

    fn set_rows(&mut self, _: u32) {
        ()
    }

    fn set_cols(&mut self, _: u32) {
        ()
    }

    fn default_name(&self) -> &'static str {
        "Dispenser"
    }
}
