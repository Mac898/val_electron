use crate::data::interaction_movement::ItemMovementState;
use crate::data::Inventory;
use crate::data::inventory_actions::ItemActions;
use dioxus::prelude::*;

pub fn PropertiesControls(cx: Scope) -> Element {
    let item_movement = use_shared_state::<ItemMovementState>(cx).unwrap();

    let properties_panel = if item_movement.read().selected_item.is_some() {
        let item_slot = item_movement.read().selected_item.unwrap();
        rsx!( ItemOnClickProperty { item_slot: item_slot } )
    } else {
        rsx!( h1 { "No Item Selected" } )
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
            h1 { font_size: "24px", margin_top: "0px", margin_bottom: "20px", "Item Properties" }
            properties_panel
        }
    ))
}

#[derive(PartialEq, Props)]
pub struct ItemOnClickPropertyProp {
    item_slot: u32,
}
pub fn ItemOnClickProperty(cx: Scope<ItemOnClickPropertyProp>) -> Element {
    let inventory = use_shared_state::<Inventory>(cx).unwrap();

    // Fields according to the item
    let inventory_read = inventory.read();
    let on_click_field = if let Some(item) = inventory_read.slots.get(&cx.props.item_slot) {
        if let Some(on_click) = &item.on_click {
            match on_click {
                ItemActions::ChangeUI(txt) => {
                    rsx!(
                        input {
                            placeholder: "UI Name",
                            display: "block",
                            margin: "10px auto",
                            padding: "10px 20px",
                            font_size: "16px",
                            color: "#333",
                            border: "2px solid #007bff",
                            cursor: "pointer",
                            transition: "background-color 0.3s, transform 0.2s",
                            width: "100%",
                            value: "{txt}",
                            oninput: move |evt| {
                                if let Some(item) = inventory.write().slots.get_mut(&cx.props.item_slot) {
                                    if let Some(on_click) = &mut item.on_click {
                                        *on_click = ItemActions::ChangeUI(evt.value.clone());
                                    }
                                }
                            }
                        }
                    )
                },
                ItemActions::Command(txt) => {
                    rsx!(
                        input {
                            placeholder: "Command",
                            display: "block",
                            margin: "10px auto",
                            padding: "10px 20px",
                            font_size: "16px",
                            color: "#333",
                            border: "2px solid #007bff",
                            cursor: "pointer",
                            transition: "background-color 0.3s, transform 0.2s",
                            width: "100%",
                            value: "{txt}",
                            oninput: move |evt| {
                                if let Some(item) = inventory.write().slots.get_mut(&cx.props.item_slot) {
                                    if let Some(on_click) = &mut item.on_click {
                                        *on_click = ItemActions::Command(evt.value.clone());
                                    }
                                }
                            }
                        }
                    )
                },
                ItemActions::ChangeServer(txt) => {
                    rsx!(
                        input {
                            placeholder: "Server Name",
                            display: "block",
                            margin: "10px auto",
                            padding: "10px 20px",
                            font_size: "16px",
                            color: "#333",
                            border: "2px solid #007bff",
                            cursor: "pointer",
                            transition: "background-color 0.3s, transform 0.2s",
                            width: "100%",
                            value: "{txt}",
                            oninput: move |evt| {
                                if let Some(item) = inventory.write().slots.get_mut(&cx.props.item_slot) {
                                    if let Some(on_click) = &mut item.on_click {
                                        *on_click = ItemActions::ChangeServer(evt.value.clone());
                                    }
                                }
                            }
                        }
                    )
                }
            }
        } else {rsx!("")}
    } else {rsx!("")};

    cx.render(rsx!(
        h1 { "On Click Action" }
        select {
            onchange: move |event| {
                let mut winventory = inventory.write();
                let item = winventory.slots.get_mut(&cx.props.item_slot).unwrap();
                match event.value.as_str() {
                    "ChangeUI" => {
                        item.on_click = Some(ItemActions::ChangeUI("".to_string()));
                    }
                    "Command" => {
                        item.on_click = Some(ItemActions::Command("".to_string()));
                    }
                    "ChangeServer" => {
                        item.on_click = Some(ItemActions::ChangeServer("".to_string()));
                    }
                    _ => {}
                }
            },
            option {
                value: "Select",
                selected: inventory.read().slots.get(&cx.props.item_slot).unwrap().on_click.is_none(),
                "Select Option",
            }
            option {
                value: "ChangeUI",
                selected: matches!(inventory.read().slots.get(&cx.props.item_slot).unwrap().on_click, Some(ItemActions::ChangeUI(_))),
                "Change UI"
            }
            option {
                value: "Command",
                selected: matches!(inventory.read().slots.get(&cx.props.item_slot).unwrap().on_click, Some(ItemActions::Command(_))),
                "Chat Command"
            }
            option {
                value: "ChangeServer",
                selected: matches!(inventory.read().slots.get(&cx.props.item_slot).unwrap().on_click, Some(ItemActions::ChangeServer(_))),
                "Change Server (Bungee/Velocity Network Only)"
            }
        }
        on_click_field
    ))
}
