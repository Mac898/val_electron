use enum_dispatch::enum_dispatch;

#[enum_dispatch]
#[derive(PartialEq, Clone)]
pub enum InventoryTypes {
    SmallChest(SmallChestType),
    LargeChest(LargeChestType),
    Hopper(HopperType),
    Dispenser(DispenserType)
}

#[enum_dispatch(InventoryTypes)]
pub trait InventoryType {
    fn get_columns(&self) -> u32;
    fn get_rows(&self) -> u32;

    fn get_min_columns(&self) -> u32;
    fn get_max_columns(&self) -> u32;

    fn get_min_rows(&self) -> u32;
    fn get_max_rows(&self) -> u32;

    fn default_name(&self) -> String;
}

#[derive(PartialEq, Clone)]
pub struct SmallChestType {
    pub rows: u32,
}
impl Default for SmallChestType {
    fn default() -> Self {
        Self {
            rows: 3,
        }
    }
}
impl InventoryType for SmallChestType {
    fn get_columns(&self) -> u32 {
        9
    }

    fn get_rows(&self) -> u32 {
        return self.rows
    }

    fn get_min_columns(&self) -> u32 {
        9
    }

    fn get_max_columns(&self) -> u32 {
        9
    }

    fn get_min_rows(&self) -> u32 {
        1
    }
    fn get_max_rows(&self) -> u32 {
        3
    }
    fn default_name(&self) -> String {
        "Chest".to_string()
    }
}

#[derive(PartialEq, Clone)]
pub struct LargeChestType {
    pub rows: u32,
}
impl Default for LargeChestType {
    fn default() -> Self {
        Self {
            rows: 6
        }
    }
}
impl InventoryType for LargeChestType {
    fn get_columns(&self) -> u32 {
        9
    }

    fn get_rows(&self) -> u32 {
        return self.rows
    }

    fn get_min_columns(&self) -> u32 {
        9
    }

    fn get_max_columns(&self) -> u32 {
        9
    }

    fn get_min_rows(&self) -> u32 {
        4
    }

    fn get_max_rows(&self) -> u32 {
        6
    }

    fn default_name(&self) -> String {
        "Large Chest".to_string()
    }
}

#[derive(PartialEq, Default, Clone)]
pub struct HopperType {}
impl InventoryType for HopperType {
    fn get_columns(&self) -> u32 {
        5
    }

    fn get_rows(&self) -> u32 {
        1
    }

    fn get_min_columns(&self) -> u32 {
        5
    }

    fn get_max_columns(&self) -> u32 {
        5
    }

    fn get_min_rows(&self) -> u32 {
        1
    }

    fn get_max_rows(&self) -> u32 {
        1
    }

    fn default_name(&self) -> String {
        "Hopper".to_string()
    }
}

#[derive(PartialEq, Default, Clone)]
pub struct DispenserType {}
impl InventoryType for DispenserType {
    fn get_columns(&self) -> u32 {
        3
    }

    fn get_rows(&self) -> u32 {
        3
    }

    fn get_min_columns(&self) -> u32 {
        3
    }

    fn get_max_columns(&self) -> u32 {
        3
    }

    fn get_min_rows(&self) -> u32 {
        3
    }

    fn get_max_rows(&self) -> u32 {
        3
    }

    fn default_name(&self) -> String {
        "Dispenser".to_string()
    }
}
