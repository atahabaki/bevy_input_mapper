use bevy::{
    input::{mouse::MouseMotion, ButtonState},
    prelude::*,
    utils::HashMap,
};

#[derive(Debug)]
pub enum InputActionData {
    Swipable(String, InputAction, Vec2),
    Pressable(String, InputAction),
}

#[derive(Debug, Clone, Copy)]
pub struct KeyboardButtonActionConfig(pub KeyCode, pub ButtonState);
#[derive(Debug, Clone, Copy)]
pub struct MouseButtonActionConfig(pub MouseButton, pub ButtonState);

#[derive(Debug, Clone, Copy)]
pub struct MouseMoveActionConfig(pub Vec2);
#[derive(Debug, Clone, Copy)]
pub struct GamepadButtonActionConfig(pub GamepadButtonType, pub ButtonState);
#[derive(Debug, Clone, Copy)]
pub struct GamepadStickMoveActionConfig(pub GamepadAxisType);

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct InputAction {
    pub keyboard_button: Option<KeyboardButtonActionConfig>,
    pub mouse_button: Option<MouseButtonActionConfig>,
    pub mouse_axis: Option<MouseMoveActionConfig>,
    pub gamepad_button: Option<GamepadButtonActionConfig>,
    pub gamepad_axis: Option<GamepadStickMoveActionConfig>,
}

impl InputAction {
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

    pub fn has_keyboard_button(&self) -> bool {
        self.keyboard_button.is_some()
    }

    pub fn has_mouse_button(&self) -> bool {
        self.mouse_button.is_some()
    }

    pub fn has_mouse_axis(&self) -> bool {
        self.mouse_axis.is_some()
    }

    pub fn has_gamepad_button(&self) -> bool {
        self.gamepad_button.is_some()
    }

    pub fn has_gamepad_axis(&self) -> bool {
        self.gamepad_axis.is_some()
    }
}

#[derive(Event)]
pub struct InputActionEvent(pub InputActionData);

#[derive(Resource)]
pub struct ConfiguredInputActions(pub HashMap<&'static str, InputAction>);

pub struct InputActionPlugin;

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
