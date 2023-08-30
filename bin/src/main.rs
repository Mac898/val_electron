#![allow(non_snake_case)]

mod components;
mod data;

use dioxus::prelude::*;

use components::inventory::InventoryGUI;
use components::size_control::InventorySizeControls;
use data::Inventory;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Inventory::default());
    cx.render(rsx!(
        table {
            tr {
                td {
                    width: "30%",
                    padding_right: "30px",
                    InventorySizeControls {}
                }
                td {
                    width: "50%",
                    InventoryGUI {}
                }
                td {
                    width: "20%",
                    padding_left: "30px",
                    h1 {
                        "TEMP"
                    }
                }
            }
        }
    ))
}
