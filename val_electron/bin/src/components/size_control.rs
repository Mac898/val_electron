use crate::data::inventory_type::{SmallChestType, LargeChestType, DispenserType, HopperType, InventoryTypes};

use dioxus::prelude::*;

pub fn InventorySizeControls(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            margin: "0 auto",
            text_align: "center",
            background_color: "#c2c2c2",
            padding: "20px",
            border_radius: "10px",
            box_shadow: "0px 2px 4px rgba(0,0,0,0.1)",

            // Title
            h2 { font_size: "24px", margin_top: "0px", margin_bottom: "20px", "Inventory Sizes" }

            // Size Buttons
            SizeChangeButton {
                name: "9x1",
                new_inventory_kind: InventoryTypes::SmallChest(SmallChestType::default()),
                rows: 1
            }
            SizeChangeButton {
                name: "9x2",
                new_inventory_kind: InventoryTypes::SmallChest(SmallChestType::default()),
                rows: 2
            }
            SizeChangeButton { name: "9x3", new_inventory_kind: InventoryTypes::SmallChest(SmallChestType::default()) }
            SizeChangeButton {
                name: "9x4",
                new_inventory_kind: InventoryTypes::LargeChest(LargeChestType::default()),
                rows: 4
            }
            SizeChangeButton {
                name: "9x5",
                new_inventory_kind: InventoryTypes::LargeChest(LargeChestType::default()),
                rows: 5
            }
            SizeChangeButton { name: "9x6", new_inventory_kind: InventoryTypes::LargeChest(LargeChestType::default()) }
            SizeChangeButton { name: "5x1 (Hopper)", new_inventory_kind: InventoryTypes::Hopper(HopperType::default()) }
            SizeChangeButton {
                name: "3x3 (Dispenser)",
                new_inventory_kind: InventoryTypes::Dispenser(DispenserType::default())
            }
        }
    ))
}

#[derive(PartialEq, Props)]
pub struct SizeChangeButtonProps {
    name: &'static str,
    new_inventory_kind: InventoryTypes,
    rows: Option<u32>,
}

pub fn SizeChangeButton(cx: Scope<SizeChangeButtonProps>) -> Element {
    let inventory_type = use_shared_state::<InventoryTypes>(cx).unwrap();

    cx.render(rsx!(
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
                let mut new_inventory_type = cx.props.new_inventory_kind.clone();
                match new_inventory_type {
                    InventoryTypes::SmallChest(ref mut chest) => {
                        if let Some(row_count) = cx.props.rows {
                            chest.rows = row_count;
                        }
                    }
                    InventoryTypes::LargeChest(ref mut chest) => {
                        if let Some(row_count) = cx.props.rows {
                            chest.rows = row_count;
                        }
                    }
                    _ => {}
                }
                *inventory_type.write() = new_inventory_type;
            },
            "{cx.props.name}"
        }
    ))
}
