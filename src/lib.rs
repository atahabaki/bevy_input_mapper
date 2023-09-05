use bevy::{
    input::{mouse::MouseMotion, ButtonState},
    prelude::*,
    utils::HashMap,
};
/// Any input that can be represented as clickable/pressable.
/// Such as buttons.
pub trait Press {}
impl Press for MouseButton {}
impl Press for KeyCode {}
impl Press for GamepadButtonType {}
/// Any input change that can be represented in 2D Axis system.
/// Examples: mouse movements, and gamepad's left and right stick movements.
pub trait Move2D {}
impl Move2D for Axis2D {}
impl Move2D for GamepadAxis {}
/// Use for Mapping and Input Scanning.
/// Represents pressable inputs with ButtonState.
#[derive(Debug, Clone, Copy)]
pub struct Pressable<T: Press> {
    pub button: T,
    pub state: ButtonState,
}
/// Used for Mapping and Input Scanning.
/// Represents movable inputs such as analog sticks and mouses with prefered axis.
#[derive(Debug, Clone, Copy)]
pub struct Movable<T: Move2D> {
    pub prefered_axis: T,
}
/// Used to lock the axis to the desired.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Axis2D {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY
}
/// Represents Gamepad's Axis
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GamepadAxis {
    PositiveLeftStickX,
    NegativeLeftStickX,
    PositiveLeftStickY,
    NegativeLeftStickY,
    PositiveRightStickX,
    NegativeRightStickX,
    PositiveRightStickY,
    NegativeRightStickY,
}
/// Currently supported input types are pressable and swipable.
#[derive(Debug)]
pub enum InputActionData {
    Swipable(String, InputAction, f32),
    Pressable(String, InputAction),
}
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
    pub keyboard_button: Option<Pressable<KeyCode>>,
    /// Used to bind to a Mouse Button.
    pub mouse_button: Option<Pressable<MouseButton>>,
    /// Used to bind to a Mouse Move.
    pub mouse_axis: Option<Movable<Axis2D>>,
    /// Used to bind to a Gamepad Button.
    pub gamepad_button: Option<Pressable<GamepadButtonType>>,
    /// Used to bind to a Gamepad axis.
    pub gamepad_axis: Option<Movable<GamepadAxis>>,
}

impl InputAction {
    /// Returns [InputAction] if at least one Mapping exist.
    pub fn new(
        keyboard_button: Option<Pressable<KeyCode>>,
        mouse_button: Option<Pressable<MouseButton>>,
        mouse_axis: Option<Movable<Axis2D>>,
        gamepad_button: Option<Pressable<GamepadButtonType>>,
        gamepad_axis: Option<Movable<GamepadAxis>>,
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
            if kbd.state == ButtonState::Pressed && keyboard_input.just_pressed(kbd.button)
                || keyboard_input.pressed(kbd.button)
            {
                events.send(InputActionEvent(InputActionData::Pressable(
                    (*action.0).clone().into(),
                    *action.1,
                )));
            }
            if kbd.state == ButtonState::Released && keyboard_input.just_released(kbd.button) {
                events.send(InputActionEvent(InputActionData::Pressable(
                    (*action.0).clone().into(),
                    *action.1,
                )));
            }
        }
        if action.1.has_mouse_axis() {
            let axis = action.1.mouse_axis.unwrap().prefered_axis;
            for ev in mouse_motion.iter() {
                events.send(InputActionEvent(InputActionData::Swipable(
                    (*action.0).clone().into(),
                    *action.1,
                    if axis == Axis2D::X {
                        ev.delta.x
                    } else {
                        ev.delta.y
                    },
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

    fn ready(&self, _app: &App) -> bool {
        true
    }

    fn finish(&self, _app: &mut App) {
        // do nothing
    }

    fn cleanup(&self, _app: &mut App) {
        // do nothing
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}

/// Used to insert binding/mapping to [ConfiguredInputActions].
/// This will perhaps change
#[cfg(feature = "bind_macro")]
#[macro_export]
macro_rules! bind {
    ($action_map:ident, $action_name:expr,
        $(
            ($button:expr, $state:expr)
        ),+
    ) => {
        {
            let mut input_action = InputAction::default();
            $(
                // TODO: Get button type. Insert to appropriate slot.
            )+
            $action_map.entry($action_name).or_insert(input_action);
        }
    };
    ($action_map:ident, $action_name:expr, $($axis:expr),+) => {
            {
                let mut input_action = InputAction::default();
                $(
                    // TODO: Get Axis type. Insert to appropriate slot.
                )+
                $action_map.entry($action_name).or_insert(input_action);
            }
    }
}
