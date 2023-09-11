use crate::data::{ApplicationState, Item};
use dioxus::prelude::*;

pub fn InventoryGUI(cx: Scope) -> Element {
    let state = use_shared_state::<ApplicationState>(cx).unwrap();

    let num_cols = state.read().inventory.kind.num_cols();
    let num_rows = state.read().inventory.kind.num_rows();

    let items = &state.read().inventory.slots;

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
            grid_template_columns: "repeat({num_cols}, 54px)",
            gap: "0px",
            h5 {
                position: "absolute",
                top: "-29px",
                left: "16px",
                font_family: "minecraftregular",
                color: "#414141",
                font_size: "18pt",
                "{state.read().inventory.name}"
            }
            for colpos in 0..num_cols {
                for rowpos in 0..num_rows {}
            }
        }
    ))
}

fn InventoryCell(cx: Scope) -> Element {
    cx.render(rsx!(
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
            prevent_default: "ondragover",
            prevent_default: "ondrop",
            ondrop: move |_| {
                let mut wstate = state.write();
                let slotnum = rowpos * wstate.inventory.kind.num_cols() + colpos;
                let draggedDataCell = wstate.draggedData.clone();
                let inventory = &mut wstate.inventory;
                let draggedData = draggedDataCell.borrow().clone().unwrap();
                match inventory.slots.get_mut(&slotnum) {
                    Some(item) => {
                        item.id = draggedData;
                    }
                    None => {
                        inventory
                            .slots
                            .insert(
                                slotnum,
                                Item {
                                    id: draggedData,
                                    on_click: None,
                                },
                            );
                    }
                }
            },
            if let Some(item) = items.get(&(rowpos * num_cols + colpos)) {
                let src_text = "resources/assets/minecraft/textures/item/{item.id}.png";
            } else {
                let src_text = "";
            }
            img { src: src_text, width: "46px", height: "46px" }
        }
    ))
}