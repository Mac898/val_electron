use crate::data::Inventory;
use dioxus::prelude::*;
use minecraft_data_rs::Api;

pub fn Items(cx: Scope) -> Element {
    let items = Api::latest().unwrap().items.items_array().unwrap();

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
            div {
                h5 {
                    position: "absolute",
                    top: "-29px",
                    left: "16px",
                    font_family: "minecraftregular",
                    color: "#414141",
                    font_size: "18pt",
                    "Items"
                }
            }
            div {
                display: "block",
                width: "240px",
                margin: "0px auto",
                // Search Box
                input {
                    placeholder: "Search...",
                    display: "block",
                    margin: "0px auto",
                    padding: "10px 20px",
                    font_size: "16px",
                    color: "#333",
                    border: "2px solid #007bff",
                    cursor: "pointer",
                    transition: "background-color 0.3s, transform 0.2s",
                    width: "100%",
                    oninput: move | evt | {}
                }
            }
            br {}
            div {
                display: "inline-grid",
                grid_template_columns: "repeat(5, 54px)",
                gap: "0px",
                for item in items {
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
                        img {
                            src: "resources/assets/minecraft/textures/item/{item.name}.png",
                            width: "46px",
                            height: "46px",
                        }
                    }
                }
            }
        }
    ))
}
