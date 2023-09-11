use dioxus::html::KeyCode::P;

pub enum InventoryTypes {
    SmallChest(SmallChestType),
    LargeChest(LargeChestType),
    Hopper(HopperType),
    Dispenser(DispenserType)
}

pub struct SmallChestType {
    rows: u32,
}
impl Default for SmallChestType {
    fn default() -> Self {
        Self {
            rows: 3,
        }
    }
}
impl SmallChestType {
    pub fn get_min_rows() -> u32 {
        1
    }
    pub fn get_max_rows() -> u32 {
        3
    }
    pub fn default_name() -> String {
        "Chest".to_string()
    }
}