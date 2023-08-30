use crate::data::Inventory;
use dioxus::prelude::*;

pub fn InventoryGUI(cx: Scope) -> Element {
    let inventory = use_shared_state::<Inventory>(cx).unwrap();

    cx.render(rsx!(
        div {
            position: "relative",
            left: "50%",
            top: "50%",
            transform: "translate(-50%, 0%)",
            margin: "0 auto",
            background_color: "#c2c2c2",
            border: "5px solid #ffffff",
            padding_left: "12px",
            padding_right: "12px",
            padding_top: "42px",
            padding_bottom: "12px",
            display: "inline-grid",
            grid_template_columns: "repeat({inventory.read().num_cols}, 54px)",
            gap: "0px",
            h5 {
                position: "absolute",
                top: "-29px",
                left: "16px",
                font_family: "minecraftregular",
                color: "#414141",
                font_size: "18pt",
                "{inventory.read().text}"
            }
            for _ in 0..inventory.read().num_cols {
                for _ in 0..inventory.read().num_rows {
                    div {
                        width: "48px",
                        height: "48px",
                        border_left: "3px solid #333333",
                        border_top: "3px solid #333333",
                        border_bottom: "3px solid #ffffff",
                        border_right: "3px solid #ffffff",
                        background_color: "#8b8b8b",
                        display: "flex",
                        justify_content: "center",
                        align_items: "center",
                        font_size: "18px",
                        color: "#8b8b8b",
                    }
                }
            }
        }
    ))
}
