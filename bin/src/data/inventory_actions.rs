use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemActions {
    ChangeUI(String),
    Command(String),
    ChangeServer(String)
}
