use bevy::{
    prelude::*, 
    input::mouse::MouseMotion,
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


pub fn rotate_camera_using_mouse(
    mut motion_evr: EventReader<MouseMotion>,
    mut q1: Query<&mut OrbitingCameraState, With<Camera3d>>,
    time: Res<Time>,
) {
    for ev in motion_evr.iter() {        
        for mut camera in q1.iter_mut() {
            if ev.delta.length() < 0.1 {
                continue;
            }

            camera.longitude += ev.delta.x * time.delta_seconds() * 0.1;
            camera.latitude -= ev.delta.y * time.delta_seconds() * 0.1;
        }
    }
}


#[derive(Component,Default)]
pub struct Target {
}

pub fn spawn_camera(
    mut commands: Commands, 
    mut added_player: Query<Entity, Added<Target>>
 ) {
    for id in added_player.iter_mut() {
        commands.spawn((
            Name::new("CameraBundle"),
            Camera3dBundle {
                transform: Transform::default()
                    .with_translation(Vec3::new(10.0, 10.0, 10.0)),
                ..Default::default()
            },
            OrbitingCameraState::default()
                .with_target(id),
        ));
    }
}

