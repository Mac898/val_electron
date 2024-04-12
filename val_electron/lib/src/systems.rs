use super::component::*;
use std::borrow::Cow;

use val_electron_data::data::inventory_type::InventoryTypes;
use val_electron_data::data::Inventory;

use std::fs;

use valence::inventory::ClientInventoryState;
use valence::log;
use valence::prelude::*;
use valence::protocol::packets::play::open_screen_s2c::WindowType;
use valence::protocol::packets::play::{CloseScreenS2c, InventoryS2c, OpenScreenS2c};
use valence::protocol::VarInt;

pub fn init(mut commands: Commands) {
    // Make sure directories are present!
    fs::create_dir_all("./config/val_electron/guis").unwrap();

    // Read files from guis path.
    let guis = fs::read_dir("./config/val_electron/guis").unwrap();
    for gui in guis.flatten() {
        let gui_data = fs::read_to_string(gui.path()).unwrap();
        let gui_inventory = serde_json::from_str::<Inventory>(&gui_data).unwrap();

        // Name - No translation needed.

        // Inventory Kind
        let kind = match gui_inventory.kind {
            InventoryTypes::SmallChest(ref chest) => match chest.rows {
                1 => InventoryKind::Generic9x1,
                2 => InventoryKind::Generic9x2,
                3 => InventoryKind::Generic9x3,
                _ => {
                    panic!("Unsupported chest size")
                }
            },
            InventoryTypes::LargeChest(ref chest) => match chest.rows {
                4 => InventoryKind::Generic9x4,
                5 => InventoryKind::Generic9x5,
                6 => InventoryKind::Generic9x6,
                _ => {
                    panic!("Unsupported chest size")
                }
            },
            InventoryTypes::Hopper(_) => InventoryKind::Hopper,
            InventoryTypes::Dispenser(_) => InventoryKind::Generic3x3,
        };

        // Slots!

        // Construct the final data type.
        let gui_runnable_inventory = InventoryGUI {
            name: gui_inventory.gui_name,
            title: gui_inventory.name.into_text(),
            kind,
            items: gui_inventory.slots,
        };
        commands.spawn(gui_runnable_inventory);
        log::info!("Successfully created new InventoryGUI");
    }
}

pub fn packet_gui_update(
    mut commands: Commands,
    mut clients: Query<(
        Entity,
        &mut Client,
        &mut ClientInventoryState,
        &mut OpenInventory,
        Ref<CursorItem>,
    )>,
    mut inventories: Query<&mut InventoryGUI>,
) {
    for (client_entity, mut client, mut inv_state, mut open_inventory, cursor_item) in &mut clients
    {
        let Ok(mut gui) = inventories.get_mut(open_inventory.entity) else {
            // Delete open inventory as the GUI is nonexistent.
            commands.entity(client_entity).remove::<OpenInventory>();

            client.write_packet(&CloseScreenS2c {
                window_id: inv_state.window_id,
            });

            continue;
        };

        // Send GUI initialization packets.
        if open_inventory.is_added() {
            inv_state.window_id = inv_state.window_id % 100 + 1;
            open_inventory.client_changed = 0;

            client.write_packet(&OpenScreenS2c {
                window_id: VarInt(inv_state.window_id.into()),
                window_type: WindowType::from(gui.kind),
                window_title: Cow::Borrowed(&gui.title),
            });

            client.write_packet(&InventoryS2c {
                window_id: inv_state.window_id,
                state_id: VarInt(inv_state.state_id.0),
                slots: Cow::Borrowed(gui.items.iter().map(|s| {}).collect()),
                carried_item: Cow::Borrowed(&cursor_item.0),
            });
        }
    }
}
