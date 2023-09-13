use crate::data::Inventory;
use crate::data::interaction_movement::ItemMovementState;
use crate::data::inventory_type::{InventoryTypes, InventoryType};
use crate::data::inventory_items::Item;

use dioxus::prelude::*;

pub fn InventoryGUI(cx: Scope) -> Element {
    let kind = use_shared_state::<InventoryTypes>(cx).unwrap();

    let num_cols = kind.read().get_columns();
    let num_rows = kind.read().get_rows();

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
            grid_template_columns: "repeat({kind.read().get_columns()}, 54px)",
            gap: "0px",
            h5 {
                position: "absolute",
                top: "-29px",
                left: "16px",
                font_family: "minecraftregular",
                color: "#414141",
                font_size: "18pt",
                "{kind.read().default_name()}"
            }
            for rownum in 0..num_rows {
                for colnum in 0..num_cols {
                    InventoryCell { slot_position: num_cols * rownum + colnum }
                }
            }
        }
    ))
}

#[derive(PartialEq, Props)]
struct InventoryCellProps {
    slot_position: u32,
}

fn InventoryCell(cx: Scope<InventoryCellProps>) -> Element {
    let drag_data = use_shared_state::<ItemMovementState>(cx).unwrap();
    let inventory_data = use_shared_state::<Inventory>(cx).unwrap();

    let slot_num = cx.props.slot_position;

    let img_if_needed;
    {
        let w_inv_data = inventory_data.read();
        if let Some(item) = w_inv_data.slots.get(&slot_num) {
            let item_name = item.id.clone();
            img_if_needed = rsx!(
                img { src: "resources/assets/minecraft/textures/item/{item_name}.png", width: "46px", height: "46px" }
            )
        } else {
            img_if_needed = rsx!("")
        };
    }

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
            prevent_default: "dragover dragenter",
            ondrop: move |event| {
                log::info!("Drop Event Caught");
                event.stop_propagation();
                let inventory = &mut inventory_data.write();
                let draggedData = drag_data.write();
                match inventory.slots.get_mut(&slot_num) {
                    Some(item) => {
                        item.id = draggedData.dragged_item.clone().unwrap();
                    }
                    None => {
                        inventory.slots.insert(
                                slot_num,
                                Item {
                                    id: draggedData.dragged_item.clone().unwrap(),
                                    on_click: None,
                                },
                            );
                    }
                }
            },
            img_if_needed
        }
    ))
}
