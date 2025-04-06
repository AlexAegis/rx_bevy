# bevy_kit_action

TODO: Rename to bevy_socketed_actions? Or bevy_action_socket? bevy_sock? signals?

An input mapper solution

## Concepts

- Signal
  - Type: Anything
  - Data that is being routed around and read, changes of the signal can
    trigger events
  - Analogy: Electricity in a wire
- Action/Channel (Wire)
  - Type: Struct/Enum
  - An Action is a type that carries a signal as its value to distinguish between
    different signals.
- Socket
  - Holds data for a channel
  - Holds connections with other sockets
  - TODO: A type that defines the shape a signal
- ??? Container
  - Type: Component
  - Holds a signals value, can be used to query it.
- ??? Converter
  - An input and output socket pair where the data
    type differs, and conversion happens
  - Example: a `bool` is converted into an `f32` of `0.0` and `1.0`
- ??? Mapper
  - Maybe this is the same as a converter
- ??? Writing into an Action
  - Actions induce signal changes
  - Analogy: Flipping a light switch, which has a wire connected to it, and by
    flipping the switch, you increase the voltage in the wire.

## Cargo Features

TODO: Review and describe the remaining features

- `keyboard`
  - On by default
  - Creates a special entity representing the keyboard as an action context
- `mouse`
  - On by default
  - Creates a special entity representing the mouse as an action context
- `gamepad`
  - On by default
  - Assigns an `ActionContext` to every `GamePad` object to be used as an
    action source

## Examples

All in one example:

```sh
cargo run -p bevy_kit_action --example example --features example
```

## Ideas

Schedule:

1. Manual input of actions in `PreUpdate` from the user, and devices
2. Map each action state to its mapped target, do transformation etc
3. repeat two in order to map all layers in a single frame
4. trigger observers

## Questions

- Who to trigger
- Where to store and how mappings

## TODO

- Accumulator, what if multiple actions map to the same target action? By default I can just overwrite it with the latest, but an accumulator could combine them
- Modifiers based off the ADSR value, it modifies Signal
- Conditions, when can an action trigger based on? idk, but it should return an enum,
- Put keyboards and inputs behind features
- Action mappings should be From<Action> impl based, but maybe a registry based one would be nice too
- Chords should be order sensitive
  - Possible chords should be easily accessible, and when an action is received
    check if it starts any of the chords, in a single frame it should be possible
    to trigger multiple steps in a chord, so order matters: finding started chords could be done
    from the actions side, as that's cheaper, but then the rest of the "did it continue" checks must be done from the chords side and search all incoming actions even if they were "processed already"
- What can an action do?
  - Start/Activate
  - Ongoing/InProgress
  - End/Deactivate
- Differences between what I want and bevy_enhanced_input
  - Mappings between actions and actions to different entities. For example if an rc-car controller can be switched between 2 cars
- Extract the core of this library about envelopes into it's own crate
