# Bevy Input Mapper Plugin

Simplify Bevy game development with the user-friendly Bevy Input Mapper Plugin.

## Keywords

- **Action:** An action represents a user input trigger, such as firing a weapon, looking up, jumping, strafing, and more.

- **Input:** Input encompasses any user-activated or pressable input, including mouse movement, mouse buttons, gamepad analog sticks, and etc.

- **Binding:** Establish a direct connection between specific user input (e.g., pressing the space key, clicking the left mouse button, etc.) and a corresponding gameplay action (e.g., firing, jumping).

- **Scenario Profile:** A scenario profile is used to manage distinct bindings required for various actions, such as walking, driving, or fighting. It allows for customized input configurations in different gameplay scenarios.

## Features

- [x] Input Mapping
- [ ] Load bindings from files
- [ ] Supported Input Devices:
  - [ ] Gamepad
  - [x] Keyboard
  - [x] Mouse
    - [x] Mouse Movement
    - [x] Mouse Buttons
- [ ] Multiple scenario profiles

## Installation & Usage

You can install the plugin via Cargo by adding it to your project's dependencies:

```
cargo add bevy_input_mapper
```

For practical examples and implementation details, refer to the [examples](examples/) directory in this repository.


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Credits

This version of Bevy Input Mapper, merged after the refactoring branch, draws heavy inspiration from [Kurinji](https://github.com/PradeepKumarRajamanickam/kurinji).