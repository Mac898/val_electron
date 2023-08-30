use std::collections::HashMap;

pub struct Item {
    id: u32,
    image_path: String,
}

pub struct Inventory {
    pub num_cols: u32,
    pub num_rows: u32,
    pub text: String,
    state: HashMap<(u32, u32), Item>,
}
impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            num_cols: 5,
            num_rows: 1,
            text: "Default Text".to_string(),
            state: Default::default(),
        }
    }
}
