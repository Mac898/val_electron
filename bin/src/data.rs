use std::cell::RefCell;
use std::collections::HashMap;

pub struct ApplicationState {
    pub selected_item: Option<u32>,
    pub inventory: Inventory,
    pub draggedData: RefCell<Option<String>>,
}

impl Default for ApplicationState {
    fn default() -> Self {
        ApplicationState {
            selected_item: None,
            inventory: Default::default(),
            draggedData: RefCell::from(None),
        }
    }
}

pub struct Inventory {
    pub name: String,
    pub slots: HashMap<u32, Item>,
    pub kind: Box<dyn InventoryKind>,
}
impl Default for Inventory {
    fn default() -> Self {
        let kind = GenericChestInventoryKind::default();
        Inventory {
            name: kind.default_name().to_string(),
            slots: Default::default(),
            kind: Box::new(kind),
        }
    }
}

pub struct Item {
    pub id: String,
    pub on_click: Option<String>,
}

pub trait InventoryKind {
    fn max_cols(&self) -> u32;
    fn max_rows(&self) -> u32;
    fn num_rows(&self) -> u32;
    fn num_cols(&self) -> u32;
    fn set_rows(&mut self, rows: u32);
    fn set_cols(&mut self, cols: u32);
    fn default_name(&self) -> &'static str;
}

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
