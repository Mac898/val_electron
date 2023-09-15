use dioxus::prelude::*;
use crate::data::interaction_movement::ItemMovementState;
use crate::data::Inventory;
use crate::data::inventory_actions::ItemActions;

pub fn PropertiesControls(cx: Scope) -> Element {
    let item_movement = use_shared_state::<ItemMovementState>(cx).unwrap();
    let inventory = use_shared_state::<Inventory>(cx).unwrap();

    let properties_panel = if item_movement.read().selected_item.is_some() {
        let item_slot = item_movement.read().selected_item.unwrap();
        rsx!(
            ItemActionProperty {
                name: "On Click",
                item_slot: item_slot
            }
        )
    } else {
        rsx!(
            h5 {
                "No Item Selected"
            }
        )
    };

    cx.render(rsx!(
        div {
            margin: "0 auto",
            text_align: "center",
            background_color: "#c2c2c2",
            padding: "20px",
            border_radius: "10px",
            box_shadow: "0px 2px 4px rgba(0,0,0,0.1)",

            // Title
            h2 { font_size: "24px", margin_top: "0px", margin_bottom: "20px", "Item Properties" }
        }
    ))
}

#[derive(PartialEq, Props)]
pub struct ItemOnClickPropertyProp {
    item_slot: u32,
}
pub fn ItemOnClickProperty(cx: Scope<ItemOnClickPropertyProp>) -> Element {
    let inventory = use_shared_state::<Inventory>(cx).unwrap();

    cx.render(rsx!(
        h5 {
            "On Click Action"
        }
        select {
            onchange: move |event| {
                let item = inventory.write().slots.get_mut(&cx.props.item_slot).unwrap();
                match event.value.as_str() {
                    "ChangeUI" => {
                        item.on_click = Some(ItemActions::ChangeUI("".to_string()));
                    },
                    "Command" => {

                    },
                    "ChangeServer" => {

                    }
                    _ => {},
                }
            },
            option {
                value: "ChangeUI",
                "Change UI"
            },
            option {
                value: "Command",
                "Chat Command"
            },
            option {
                value: "ChangeServer",
                "Change Server (Bungee/Velocity Network Only)"
            }
        }
    ))
}
