pub mod component;
pub mod systems;

use valence::prelude::*;
use valence::{MINECRAFT_VERSION, PROTOCOL_VERSION};
use valence::log::*;

use systems::*;

pub struct ElectronGUIPlugin;

impl Plugin for ElectronGUIPlugin {
    fn build(&self, app: &mut App) {
        info!("Loading Minecraft Electron GUI Plugin For valence_rs by Mac on minecraft {} with protocol {}", MINECRAFT_VERSION, PROTOCOL_VERSION);

        // Load Data From Configs & Build Structures
        app.add_systems(Startup, init);

        // Inject Handlers
        app.add_systems(Update, packet_gui_update);

        // Finish Up
    }

    fn name(&self) -> &str {
        "Minecraft Electron GUI Plugin For valence_rs"
    }

    fn is_unique(&self) -> bool {
        false
    }
}
