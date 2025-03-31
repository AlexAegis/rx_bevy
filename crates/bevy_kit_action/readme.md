# bevy_kit_action

TODO: Rename to bevy_socketed_actions? Or bevy_action_socket? bevy_sock?

An input mapper solution

## Features

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
