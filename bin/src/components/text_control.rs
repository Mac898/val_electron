use crate::data::ApplicationState;
use dioxus::prelude::*;

pub fn InventoryTextControls(cx: Scope) -> Element {
    let state = use_shared_state::<ApplicationState>(cx).unwrap();

    let inventory_name = state.read().inventory.kind.default_name().to_string();

    cx.render( rsx!(
        div {
            margin: "0 auto",
            text_align: "center",
            background_color: "#c2c2c2",
            padding: "20px",
            border_radius: "10px",
            box_shadow: "0px 2px 4px rgba(0,0,0,0.1)",

            // Title
            h2 {
                font_size: "24px",
                margin_top: "0px",
                margin_bottom: "20px",
                "Inventory Text"
            }

            // Text Box
            input {
                placeholder: "{inventory_name}",
                display: "block",
                margin: "10px auto",
                padding: "10px 20px",
                font_size: "16px",
                color: "#333",
                border: "2px solid #007bff",
                cursor: "pointer",
                transition: "background-color 0.3s, transform 0.2s",
                width: "100%",
                oninput: move | evt | state.write().inventory.name = (evt.value.clone())
            }
        }
    ))
}