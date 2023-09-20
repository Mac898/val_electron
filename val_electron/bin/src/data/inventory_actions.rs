use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Default, Serialize, Deserialize, Debug)]
pub struct ItemActions {
    pub on_click: Option<ItemAction>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ItemAction {
    ChangeUI(String),
    Command(String),
    ChangeServer(String),
}
