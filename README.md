# bevy_chess_tutorial

:round_pushpin: [Init](.docs/INIT.md)ializing the project

## Getting started

- [ ] Add to `src/main.rs`

```rust
use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .run();
}
```

- [ ] Changing Window settings

* Add the `Msaa` resource (default is 4)

```rust
    // Set antialiasing to use 4 samples
    .insert_resource(Msaa::default())
```
* Change the `DefaultPlugin` to add the `WindowPlugin`:

- the import 

```rust
use bevy::{
    prelude::*,
    window::{PresentMode},
};
```

- default plugin 

```rust
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Chess!".into(),
            resolution: (1600., 1600.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
```


# References

- [ ] [Chess game in Rust using Bevy](https://caballerocoll.com/blog/bevy-chess-tutorial)
- [ ] [bevy/examples/window
/window_settings.rs](https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs)
