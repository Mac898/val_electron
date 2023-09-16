use std::ops::{Deref, DerefMut};
use dioxus::prelude::*;
use crate::data::Inventory;

pub fn InventoryCopySaveControls(cx: Scope) -> Element {
    let inventory = use_shared_state::<Inventory>(cx).unwrap();
    let import_export_text = use_state(cx, || "".to_string());

    cx.render( rsx!(
        div {
            margin: "0 auto",
            text_align: "center",
            background_color: "#c2c2c2",
            padding: "20px",
            border_radius: "10px",
            box_shadow: "0px 2px 4px rgba(0,0,0,0.1)",

            // Title
            h2 { font_size: "24px", margin_top: "0px", margin_bottom: "20px", "Import/Export Inventory" }

            // Import / Export Button
            button {
                display: "block",
                margin: "10px auto",
                padding: "10px 20px",
                font_size: "16px",
                background_color: "#007bff",
                color: "#fff",
                border: "none",
                border_radius: "5px",
                cursor: "pointer",
                transition: "background-color 0.3s, transform 0.2s",
                width: "100%",
                onclick: move |_| {
                    let mut rinventory = inventory.write();
                    if !import_export_text.is_empty() {
                        *rinventory.deref_mut() = serde_json::from_str::<Inventory>(import_export_text).unwrap();
                    }
                },
                "Import Inventory"
            }
            button {
                display: "block",
                margin: "10px auto",
                padding: "10px 20px",
                font_size: "16px",
                background_color: "#007bff",
                color: "#fff",
                border: "none",
                border_radius: "5px",
                cursor: "pointer",
                transition: "background-color 0.3s, transform 0.2s",
                width: "100%",
                onclick: move |_| {
                    let rinventory = inventory.read();
                    if !rinventory.gui_name.is_empty() {
                        import_export_text.set(serde_json::to_string(rinventory.deref()).unwrap());
                    }
                },
                "Export Inventory"
            }
            textarea {
                placeholder: "Import/Export Text...",
                value: "{import_export_text}",
                oninput: move |event| {
                    import_export_text.set(event.value.clone());
                },
            }
        }
    ))
}
