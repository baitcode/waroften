use bevy::{
    prelude::*,
    input::gamepad::{GamepadEvent, GamepadAxisType},
};


#[derive(Default)]
pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
    fn build(&self, app: &mut App) {
        println!("UserInputPlugin::build");
        app
            .register_type::<UserInput>()
            .init_resource::<UserInput>()
            .add_system(gamepad_2_user_input);
    }
}

#[derive(Resource,Reflect)]
#[reflect(Resource)]
pub struct UserInput {
    pub direction: Vec3,
}

impl Default for UserInput {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
        }
    }
}

pub fn gamepad_2_user_input(
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut user_input: ResMut<UserInput>,
) {
    // TODO(baitcode): check if controller is configured as an input
    for ev in gamepad_evr.iter() {
        match ev {
            GamepadEvent::Axis(event) => {
                match event.axis_type {
                    GamepadAxisType::LeftStickX => user_input.direction.x = event.value,
                    GamepadAxisType::LeftStickY => user_input.direction.y = event.value,
                    _ => {},
                }
            }
            GamepadEvent::Button(_event) => {
            }
            GamepadEvent::Connection(_event) => {
                // InputDeviceList update
            }
        }
    }
}