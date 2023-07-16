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
            .add_event::<Move>()
            .add_event::<Jump>()
            .add_event::<Strike>()
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

pub struct Move {
    pub direction: Vec3,
    pub speed: f32,
}
pub struct Jump {}
pub struct Strike {}

// fn emit_event(time: Res<Time>, mut event_writer: EventWriter<MyEvent>) {
//     if time.seconds_since_startup() > 1.0 {
//         event_writer.send(MyEvent {
//             message: "One second has passed!".to_string(),
//         });
//     }
// }

fn enum_to_string(value: &GamepadButtonType) -> &'static str {
    match value {
        GamepadButtonType::DPadDown => "DPadDown",
        GamepadButtonType::DPadUp => "DPadUp",
        GamepadButtonType::DPadLeft => "DPadLeft",
        GamepadButtonType::DPadRight => "DPadRight",
        
        GamepadButtonType::C => "C",
        GamepadButtonType::Z => "Z",

        GamepadButtonType::LeftTrigger => "LeftTrigger",
        GamepadButtonType::LeftTrigger2 => "LeftTrigger2",
        GamepadButtonType::RightTrigger => "RightTrigger",
        GamepadButtonType::RightTrigger2 => "RightTrigger2",

        GamepadButtonType::LeftThumb => "LeftThumb",
        GamepadButtonType::RightThumb => "RightThumb",

        GamepadButtonType::Select => "Select",
        GamepadButtonType::Start => "Start",
        GamepadButtonType::Mode => "Mode",

        GamepadButtonType::South => "South",
        GamepadButtonType::East => "East",
        GamepadButtonType::North => "North",
        GamepadButtonType::West => "West",

        GamepadButtonType::Other(_) => "Other",
    }
}

pub fn gamepad_2_user_input(
    mut gamepad_evr: EventReader<GamepadEvent>,
    mut user_input: ResMut<UserInput>,
    mut movement: EventWriter<Move>,
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
                movement.send(Move {
                    direction: user_input.direction,
                    speed: 1.0,
                });
            }
            GamepadEvent::Button(event) => {
                println!("{} {}", event.value, enum_to_string(&event.button_type));
            }
            GamepadEvent::Connection(_event) => {
                // InputDeviceList update
            }
        }
    }

    let direction = if user_input.direction.length_squared() >= 0.1 { user_input.direction } else { Vec3::ZERO };
        
    movement.send(Move {
        direction,
        speed: 1.0,
    });
}