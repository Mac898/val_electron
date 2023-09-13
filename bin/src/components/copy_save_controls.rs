use dioxus::prelude::*;

pub fn InventoryCopySaveControls(cx: Scope) -> Element {
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
                onclick: move |_| {},
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
                onclick: move |_| {},
                "Export Inventory"
            }
        }
    ))
}
