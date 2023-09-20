use val_electron_gui::component::InventoryGUI;
use val_electron_gui::ElectronGUIPlugin;
use valence::interact_block::InteractBlockEvent;
use valence::log::*;
use valence::prelude::*;

fn main() {
    info!("Starting Valence");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ElectronGUIPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (init_clients, open_chest, despawn_disconnected_clients),
        )
        .run();
    info!("Valence has shut down");
}

// Example Code Based On chest.rs example.
const SPAWN_Y: i32 = 64;
const CHEST_POS: [i32; 3] = [0, SPAWN_Y + 1, 3];

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -25..25 {
        for x in -25..25 {
            layer
                .chunk
                .set_block([x, SPAWN_Y, z], BlockState::GRASS_BLOCK);
        }
    }

    layer.chunk.set_block(CHEST_POS, BlockState::CHEST);

    commands.spawn(layer);
}

fn init_clients(
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.0, SPAWN_Y as f64 + 1.0, 0.0]);
        *game_mode = GameMode::Creative;
    }
}

fn open_chest(
    mut commands: Commands,
    inventories: Query<Entity, With<InventoryGUI>>,
    mut events: EventReader<InteractBlockEvent>,
) {
    for event in events.iter() {
        if event.position != CHEST_POS.into() {
            continue;
        }
        let open_inventory = OpenInventory::new(inventories.single());
        commands.entity(event.client).insert(open_inventory);
    }
}
