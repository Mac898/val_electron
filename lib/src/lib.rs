use bevy::app::Plugin;
use bevy::log::*;
use valence::prelude::*;
use valence::{MINECRAFT_VERSION, PROTOCOL_VERSION};

pub struct ElectronGUIPlugin;

impl Plugin for ElectronGUIPlugin {
    fn build(&self, app: &mut App) {
        info!("Loading Minecraft Electron GUI Plugin For valence_rs by Mac"
            " on minecraft " + MINECRAFT_VERSION + " with protocol " + PROTOCOL_VERSION);

        // Load Data From Configs

        // Build Structures

        // Inject Handlers

        // Finish Up
    }

    fn name(&self) -> &str {
        return "Minecraft Electron GUI Plugin For valence_rs";
    }

    fn is_unique(&self) -> bool {
        return false;
    }
}
