use crate::data::ApplicationState;
use dioxus::prelude::*;

pub fn PropertiesControls(cx: Scope) -> Element {
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
            h2 {
                font_size: "24px",
                margin_top: "0px",
                margin_bottom: "20px",
                "Item Properties"
            }

        }
    ))
}