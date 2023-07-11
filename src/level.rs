use bevy::{
    utils::HashMap, 
    prelude::*, 
};
use bevy_rapier3d::prelude::{
    Collider, 
    RigidBody
};

#[derive(Component)]
pub struct Level {
    pub tiles: Vec<Vec<Vec<u32>>>,
    pub offset: Vec3,
}

impl Level {
    pub fn from_tiles(tiles: Vec<Vec<u32>>) -> Self {
        Self {
            tiles: vec!(tiles),
            offset: Vec3::ZERO,
        }
    }

    pub fn add_layer(mut self, tiles: Vec<Vec<u32>>) -> Self {
        self.tiles.push(tiles);
        self
    }
}

impl Default for Level {
    fn default() -> Self {
        Self {
            tiles: vec!(vec!(vec!())),
            offset: Vec3::ZERO,
        }
    }
}

pub fn draw_level(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    query: Query<(Entity, &Level), Changed<Level>>
) {
    let mut tiles = HashMap::new();
    tiles.insert(1, "map/block.glb");
    tiles.insert(2, "map/blockCliff.glb");
    tiles.insert(3, "map/blockCurve.glb");
    tiles.insert(4, "map/blockDirt.glb");
    tiles.insert(5, "map/blockEnd.glb");
    tiles.insert(6, "map/blockEnd.glb");

    for (level_group, level) in query.iter() {   
        for (z, layer) in level.tiles.iter().enumerate() {
            for (y, row) in layer.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if *tile <= 0 {
                        continue;
                    }
                    
                    let tilename = format!("{}#Scene0", tiles[tile]);
                    
                    let tile = commands.spawn((
                        Name::new(format!("Tile-{}_{}", x, y)),
                        SceneBundle { 
                            scene: asset_server.load(tilename), 
                            transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32) + level.offset)
                                .with_rotation(Quat::from_rotation_arc(Vec3::Y, Vec3::Z)),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        RigidBody::Fixed,
                    )).with_children(|children| {
                        children.spawn((
                            Collider::cuboid(0.5, 0.5, 0.5), // TODO: Collider has to be moved 0.5 up along Y axis and scaled 0.5 along Y axis
                            Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
                        ));
                    }).id();
                    
                    commands.entity(level_group).push_children(&[tile]);
                }
            }
        }
    }
}