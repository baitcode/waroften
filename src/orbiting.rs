use bevy::{
    prelude::*, 
};

pub struct OrbitingCameraPlugin;

impl Plugin for OrbitingCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update_camera);
    }
}

// TODO: use a separate component for initialisation processing and exchanges itself for initialised component state
// create system initialize_orbiting_camera(InitialisingOrbitingCameraState) -> theat spawns OrbitingCameraState
// create system update_orbiting_camera(OrbitingCameraState)
// Or add Initialised struct and update query to filter using it
fn update_camera(
    mut transforms: Query<&mut Transform>,
    mut orbiting: Query<(Entity, &mut OrbitingCameraState)>
) {
    for (camera, mut orbiting) in orbiting.iter_mut() {
        if orbiting.target.is_none() { continue };

        let target = orbiting.target.unwrap();

        if !orbiting.initialized {
            initialize_orbiting_camera(&mut transforms, target, camera, &mut orbiting);
        } 
        
        update_orbiting_camera(&mut transforms, target, camera, &mut orbiting);
    }
}

fn initialize_orbiting_camera(
    transforms: &mut Query<&mut Transform>,
    target: Entity,
    camera: Entity,
    orbiting: &mut OrbitingCameraState,
) {
    let target_transform = transforms.get(target).unwrap().clone();
    let mut camera_transform = transforms.get_mut(camera).unwrap();

    let relative_to_center = camera_transform.translation - target_transform.translation;
    orbiting.radius = relative_to_center.length();
    orbiting.longitude = relative_to_center.y.atan2(relative_to_center.x);
    orbiting.latitude = (relative_to_center.z / orbiting.radius).acos();
    camera_transform.look_at(target_transform.translation, Vec3::Z);
    orbiting.initialized = true;
}

fn update_orbiting_camera(
    transforms: &mut Query<&mut Transform>,
    target: Entity,
    camera: Entity,
    orbiting: &mut OrbitingCameraState,
) {
    let target_transform = transforms.get(target).unwrap().clone();
    let mut camera_transform = transforms.get_mut(camera).unwrap();
    
    camera_transform.translation = target_transform.translation + Vec3::new(
        orbiting.radius * orbiting.longitude.sin() * orbiting.latitude.sin(),
        orbiting.radius * orbiting.longitude.cos() * orbiting.latitude.sin(),
        orbiting.radius * orbiting.latitude.cos(),
    );
    camera_transform.look_at(target_transform.translation, Vec3::Z);
}


#[derive(Component)]
pub struct OrbitingCameraState {
    initialized: bool,
    pub radius: f32,
    pub longitude: f32,
    pub latitude: f32,
    pub target: Option<Entity>,
}

impl OrbitingCameraState {
    pub const fn with_target(mut self, target: Entity) -> Self {
        self.target = Some(target);
        self
    }
}

impl Default for OrbitingCameraState {
    fn default() -> Self {
        Self {
            initialized: false,
            radius: 1.0,
            longitude: 0.0,
            latitude: 0.0,
            target: None,
        }
    }
}
