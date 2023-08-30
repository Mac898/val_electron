use crate::data::Inventory;
use dioxus::prelude::*;

pub fn InventorySizeControls(cx: Scope) -> Element {
    let inventory = use_shared_state::<Inventory>(cx).unwrap();

    cx.render(rsx!(
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
                "Inventory Sizes"
            }

            // Size Buttons
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
                    inventory.write().num_rows = 1;
                    inventory.write().num_cols = 9;
                },
                "9x1"
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
                    inventory.write().num_rows = 2;
                    inventory.write().num_cols = 9;
                },
                "9x2"
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
                    inventory.write().num_rows = 3;
                    inventory.write().num_cols = 9;
                },
                "9x3 (Small Chest)"
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
                    inventory.write().num_rows = 4;
                    inventory.write().num_cols = 9;
                },
                "9x4"
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
                    inventory.write().num_rows = 5;
                    inventory.write().num_cols = 9;
                },
                "9x5"
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
                    inventory.write().num_rows = 6;
                    inventory.write().num_cols = 9;
                },
                "9x6 (Large Chest)"
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
                    inventory.write().num_rows = 3;
                    inventory.write().num_cols = 3;
                },
                "3x3 (Dispenser)"
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
                    inventory.write().num_rows = 1;
                    inventory.write().num_cols = 5;
                },
                "5x1 (Hopper)"
            }
        }
    ))
}
