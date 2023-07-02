use bevy::{
    utils::HashMap, 
    prelude::*, 
};

#[derive(Default, Component)]
pub struct Level {
    pub tiles: Vec<Vec<u32>>,
    pub offset: Vec3,
}

impl Level {
    pub fn from_tiles(tiles: Vec<Vec<u32>>) -> Self {
        Self {
            tiles,
            offset: Vec3::ZERO,
        }
    }

    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }
}


pub fn draw_level(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    query: Query<(Entity, &Level), Changed<Level>>
) {
    let mut tiles = HashMap::new();
    tiles.insert(0, "map/block.glb");
    tiles.insert(1, "map/blockCliff.glb");
    tiles.insert(2, "map/blockCurve.glb");
    tiles.insert(3, "map/blockDirt.glb");
    tiles.insert(3, "map/blockEnd.glb");
    tiles.insert(3, "map/blockEnd.glb");

    for (level_group, level) in query.iter() {   
        for (y, row) in level.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let tilename = format!("{}#Scene0", tiles[tile]);
                
                let tile = commands.spawn((
                    Name::new(format!("Tile-{}_{}", x, y)),
                    SceneBundle { 
                        scene: asset_server.load(tilename), 
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0) + level.offset)
                            .with_rotation(Quat::from_rotation_arc(Vec3::Y, Vec3::Z)),
                        // visibility: Visibility::Hidden,
                        ..default()
                    }
                )).id();
                
                commands.entity(level_group).push_children(&[tile]);
            }
        }
    }
}