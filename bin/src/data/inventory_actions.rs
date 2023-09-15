
#[derive(Clone, PartialEq)]
pub enum ItemActions {
    ChangeUI(String),
    Command(String),
    ChangeServer(String)
}
