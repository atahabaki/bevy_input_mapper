use bevy::{
    input::{mouse::MouseMotion, ButtonState},
    prelude::*,
    utils::HashMap,
};
/// Currently supported input types are pressable and swipable.
#[derive(Debug)]
pub enum InputActionData {
    Swipable(String, InputAction, Vec2),
    Pressable(String, InputAction),
}
/// Mapping configuration for Keyboard Button/Key.
///
/// - Uses [KeyCode] to match the Keyboard Button.
/// - Uses [ButtonState] to match the desired button state.
#[derive(Debug, Clone, Copy)]
pub struct KeyboardButtonActionConfig(pub KeyCode, pub ButtonState);
/// Mapping configuration for Mouse Button
///
/// - [MouseButton] to mapped to the Action
/// - [ButtonState] to mapped to the Action
#[derive(Debug, Clone, Copy)]
pub struct MouseButtonActionConfig(pub MouseButton, pub ButtonState);
/// Mapping configuration for the Mouse movement in X,Y coordinates.
///
/// [Vec2] is used but, you should use just `Vec2::X` or `Vec2::Y`
/// This will be changed in later versions.
#[derive(Debug, Clone, Copy)]
pub struct MouseMoveActionConfig(pub Vec2);
/// Mapping configuration for the Gamepad buttons.
///
/// - [GamepadButtonType] is used to determine which button should be used.
/// (i.e. Triangle, X, Y, DPAD_UP, etc.)
/// - [ButtonState] is used to match the button is pressed/released.
#[derive(Debug, Clone, Copy)]
pub struct GamepadButtonActionConfig(pub GamepadButtonType, pub ButtonState);
/// Mapping configuration for the Gamepad axises.
///
/// - [GamepadAxisType] is used to match the desired Axis.
#[derive(Debug, Clone, Copy)]
pub struct GamepadStickMoveActionConfig(pub GamepadAxisType);

/// Action is anything that can be achieved by any user input.
/// For example looking up/down/right/left, walking up/down/righ/left,
/// jumping, anything.
///
/// A Mapping is like binding actions (jump, walk, look) to
/// configured keys/dpads/axis.
///
/// It is basically User Input -> Action.
///
/// Below is a InputAction, has fields with optional configurations.
#[derive(Debug, Default, Clone, Copy, Component)]
pub struct InputAction {
    /// Used to bind to a Keyboard Button.
    pub keyboard_button: Option<KeyboardButtonActionConfig>,
    /// Used to bind to a Mouse Button.
    pub mouse_button: Option<MouseButtonActionConfig>,
    /// Used to bind to a Mouse Move.
    pub mouse_axis: Option<MouseMoveActionConfig>,
    /// Used to bind to a Gamepad Button.
    pub gamepad_button: Option<GamepadButtonActionConfig>,
    /// Used to bind to a Gamepad axis.
    pub gamepad_axis: Option<GamepadStickMoveActionConfig>,
}

impl InputAction {
    /// Returns [InputAction] if at least one Mapping exist.
    pub fn new(
        keyboard_button: Option<KeyboardButtonActionConfig>,
        mouse_button: Option<MouseButtonActionConfig>,
        mouse_axis: Option<MouseMoveActionConfig>,
        gamepad_button: Option<GamepadButtonActionConfig>,
        gamepad_axis: Option<GamepadStickMoveActionConfig>,
    ) -> Option<Self> {
        if keyboard_button.is_some()
            || gamepad_button.is_some()
            || mouse_button.is_some()
            || mouse_axis.is_some()
            || gamepad_axis.is_some()
        {
            return Some(Self {
                keyboard_button,
                gamepad_button,
                mouse_button,
                mouse_axis,
                gamepad_axis,
            });
        }
        None
    }
    /// Returns true if there is a mapping for Keyboard Button.
    pub fn has_keyboard_button(&self) -> bool {
        self.keyboard_button.is_some()
    }
    /// Returns true if there is a mapping for Mouse Button.
    pub fn has_mouse_button(&self) -> bool {
        self.mouse_button.is_some()
    }
    /// Returns true if there is a mapping for Mouse Movement.
    pub fn has_mouse_axis(&self) -> bool {
        self.mouse_axis.is_some()
    }
    /// Returns true if there is a mapping for Gamepad Button.
    pub fn has_gamepad_button(&self) -> bool {
        self.gamepad_button.is_some()
    }
    /// Returns true if there is a mapping for Gamepad Axis.
    pub fn has_gamepad_axis(&self) -> bool {
        self.gamepad_axis.is_some()
    }
}

/// Event for Mappings.
/// Events sent to notify the user input changes.
#[derive(Event)]
pub struct InputActionEvent(pub InputActionData);

/// This is the mapping list resource.
/// Needs to be loaded. Even empty HashMap is fine, but why you would want that?
#[derive(Resource)]
pub struct ConfiguredInputActions(pub HashMap<&'static str, InputAction>);

pub struct InputActionPlugin;

/// The main logic is here. Loops through [ConfiguredInputActions], and if finds
/// any input change, sends an [InputActionEvent] with the corresponding MappingData.
fn input_action_listener(
    configured_input_actions: Res<ConfiguredInputActions>,
    mut events: EventWriter<InputActionEvent>,
    mut mouse_motion: EventReader<MouseMotion>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for action in configured_input_actions.0.iter() {
        if action.1.has_keyboard_button() {
            let kbd = action.1.keyboard_button.unwrap();
            if kbd.1 == ButtonState::Pressed && keyboard_input.just_pressed(kbd.0)
                || keyboard_input.pressed(kbd.0)
            {
                events.send(InputActionEvent(InputActionData::Pressable(
                    (*action.0).clone().into(),
                    action.1.clone(),
                )));
            }
            if kbd.1 == ButtonState::Released && keyboard_input.just_released(kbd.0) {
                events.send(InputActionEvent(InputActionData::Pressable(
                    (*action.0).clone().into(),
                    action.1.clone(),
                )));
            }
        }
        if action.1.has_mouse_axis() {
            for ev in mouse_motion.iter() {
                events.send(InputActionEvent(InputActionData::Swipable(
                    (*action.0).clone().into(),
                    action.1.clone(),
                    ev.delta,
                )))
            }
        }
    }
}

impl Plugin for InputActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputActionEvent>()
            .add_systems(Update, input_action_listener);
    }
}
