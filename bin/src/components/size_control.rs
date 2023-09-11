use dioxus::prelude::*;

use crate::data::ApplicationState;

use crate::data::{GenericChestInventoryKind, HopperInventoryKind, DispenserInventoryKind};

pub fn InventorySizeControls(cx: Scope) -> Element {
    let state = use_shared_state::<ApplicationState>(cx).unwrap();

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
                    let mut writable_state = state.write();
                    let reset_name = writable_state.inventory.name
                        == writable_state.inventory.kind.default_name();
                    writable_state.inventory.kind = Box::new(GenericChestInventoryKind::default());
                    writable_state.inventory.kind.set_rows(1);
                    if reset_name {
                        writable_state
                            .inventory
                            .name = writable_state.inventory.kind.default_name().to_string();
                    }
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
                    let mut writable_state = state.write();
                    let reset_name = writable_state.inventory.name
                        == writable_state.inventory.kind.default_name();
                    writable_state.inventory.kind = Box::new(GenericChestInventoryKind::default());
                    writable_state.inventory.kind.set_rows(2);
                    if reset_name {
                        writable_state
                            .inventory
                            .name = writable_state.inventory.kind.default_name().to_string();
                    }
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
                    let mut writable_state = state.write();
                    let reset_name = writable_state.inventory.name
                        == writable_state.inventory.kind.default_name();
                    writable_state.inventory.kind = Box::new(GenericChestInventoryKind::default());
                    if reset_name {
                        writable_state
                            .inventory
                            .name = writable_state.inventory.kind.default_name().to_string();
                    }
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
                    let mut writable_state = state.write();
                    let reset_name = writable_state.inventory.name
                        == writable_state.inventory.kind.default_name();
                    writable_state.inventory.kind = Box::new(GenericChestInventoryKind::default());
                    writable_state.inventory.kind.set_rows(4);
                    if reset_name {
                        writable_state
                            .inventory
                            .name = writable_state.inventory.kind.default_name().to_string();
                    }
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
                    let mut writable_state = state.write();
                    let reset_name = writable_state.inventory.name
                        == writable_state.inventory.kind.default_name();
                    writable_state.inventory.kind = Box::new(GenericChestInventoryKind::default());
                    writable_state.inventory.kind.set_rows(5);
                    if reset_name {
                        writable_state
                            .inventory
                            .name = writable_state.inventory.kind.default_name().to_string();
                    }
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
                    let mut writable_state = state.write();
                    let reset_name = writable_state.inventory.name
                        == writable_state.inventory.kind.default_name();
                    writable_state.inventory.kind = Box::new(GenericChestInventoryKind::default());
                    writable_state.inventory.kind.set_rows(6);
                    if reset_name {
                        writable_state
                            .inventory
                            .name = writable_state.inventory.kind.default_name().to_string();
                    }
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
                    let mut writable_state = state.write();
                    let reset_name = writable_state.inventory.name
                        == writable_state.inventory.kind.default_name();
                    writable_state.inventory.kind = Box::new(DispenserInventoryKind::default());
                    if reset_name {
                        writable_state
                            .inventory
                            .name = writable_state.inventory.kind.default_name().to_string();
                    }
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
                    let mut writable_state = state.write();
                    let reset_name = writable_state.inventory.name
                        == writable_state.inventory.kind.default_name();
                    writable_state.inventory.kind = Box::new(HopperInventoryKind::default());
                    if reset_name {
                        writable_state
                            .inventory
                            .name = writable_state.inventory.kind.default_name().to_string();
                    }
                },
                "5x1 (Hopper)"
            }
        }
    ))
}
