use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemActions {
    pub on_click: Option<ItemAction>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemAction {
    ChangeUI(String),
    Command(String),
    ChangeServer(String),
}
