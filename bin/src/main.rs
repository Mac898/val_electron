#![allow(non_snake_case)]

mod components;
mod data;

use dioxus::prelude::*;

use components::inventory::InventoryGUI;
use components::size_control::InventorySizeControls;
use components::text_control::InventoryTextControls;
use components::copy_save_controls::InventoryCopySaveControls;
use components::items::Items;
use components::properties::PropertiesControls;

use data::Inventory;
use data::ApplicationState;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || ApplicationState::default());

    cx.render(rsx!(
        table {
            width: "100%",
            tr {
                td {
                    width: "30%",
                    padding_right: "30px",
                    InventorySizeControls {}
                    br {}
                    InventoryTextControls {}
                    br {}
                    InventoryCopySaveControls {}
                }
                td {
                    min_width: "50%",
                    InventoryGUI {}
                }
                td {
                    width: "304px",
                    padding_left: "30px",
                    PropertiesControls {}
                    br {}
                    Items {}
                }
            }
        }
    ))
}
