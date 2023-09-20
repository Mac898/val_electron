use crate::component::InventoryGUI;

use val_electron_data::data::inventory_type::{InventoryType, InventoryTypes};
use val_electron_data::data::Inventory;

use std::collections::HashMap;
use std::fs;

use valence::inventory::ClickSlotEvent;
use valence::prelude::*;
use valence::ItemKind;

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

        let mut internal_inventory =
            valence::inventory::Inventory::with_title(kind, gui_inventory.name);

        // Slots!
        for (slot, item) in &gui_inventory.slots {
            internal_inventory.set_slot(
                *slot as u16,
                ItemStack::new(ItemKind::from_str(&item.id).unwrap(), 1, None),
            );
        }
        internal_inventory.set_slot(0, ItemStack::new(ItemKind::Redstone, 1, None));

        // And finally the slot actions.
        let mut actions = HashMap::with_capacity(
            (gui_inventory.kind.get_columns() * gui_inventory.kind.get_rows()) as usize,
        );

        for (slot, item) in &gui_inventory.slots {
            actions.insert(*slot, item.actions.clone());
        }

        // Construct the final data type.
        let gui_runnable_inventory = InventoryGUI {
            name: gui_inventory.gui_name,
            slot_actions: actions,
            inv_backup: internal_inventory.clone(),
        };
        commands.spawn((gui_runnable_inventory, internal_inventory));
    }
}

pub fn handle_item_click(
    mut commands: Commands,
    mut inventories: Query<
        (Entity, &mut valence::inventory::Inventory, &InventoryGUI),
        (With<InventoryGUI>, With<valence::inventory::Inventory>),
    >,
    mut events: EventReader<ClickSlotEvent>,
) {
    for event in events.iter() {
        // Restore Inventory If Item Removed
        if event.carried_item.item != ItemKind::Air {
            for (_, mut ii, ig) in inventories.iter_mut() {
                *ii = ig.inv_backup.clone();

                // Perform Action
                println!("Event: {:#?}", event);
                if let Some(action) = ig.slot_actions.get(&(event.slot_id as u32)) {
                    println!("Action triggered: {:?}", action);
                }
            }
            let player = &mut commands.entity(event.client);
            player.insert(CursorItem {
                0: ItemStack::EMPTY,
            });
        }
    }
}
