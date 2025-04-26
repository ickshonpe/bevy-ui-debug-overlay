# bevy-ui-debug-overlay

No longer necessary as the changes have been upstreamed to Bevy 0.16:

https://bevyengine.org/learn/migration-guides/0-15-to-0-16/#draw-the-ui-debug-overlay-using-the-ui-renderer

#

Improved UI debug overlay for Bevy 0.15. 
Simpler, more efficient, and easier to use than the built-in `bevy_dev_tools::ui_debug_overlay`.
* Supports multiple windows and UI rendered to texture.
* Supports UI scale factor correctly.
* Draws rounded debug rects for rounded UI nodes.

![Debug Overlay](debug_overlay.png)
### Usage

Add the dependency to your project's `cargo.toml`:
```
bevy-ui-debug-overlay = "0.2"
```

Add the plugin to your bevy project :
```
// From the included `debug_overlay.rs` example
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiDebugOverlayPlugin::start_enabled().with_line_width(2.),
        ))
        .add_systems(Startup, (setup, debug_overlay_setup))
        .add_systems(Update, (update_scroll_position, toggle_debug_overlay))
        .run();
}
```

The debug overlay is controlled using the `UiDebugOverlay` resource:
```
fn toggle_debug_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut debug_overlay: ResMut<UiDebugOverlay>,
    mut root_node_query: Query<&mut Visibility, (With<Node>, Without<Parent>)>,
) {
    if input.just_pressed(KeyCode::Space) {
        // The toggle method will enable the debug overlay if disabled and disable if enabled
        debug_overlay.toggle();
    }

    if input.just_pressed(KeyCode::KeyS) {
        // Toggle debug outlines for nodes with `ViewVisibility` set to false.
        debug_overlay.show_hidden = !debug_overlay.show_hidden;
    }

    if input.just_pressed(KeyCode::KeyC) {
        // Toggle outlines for clipped UI nodes.
        debug_overlay.show_clipped = !debug_overlay.show_clipped;
    }

    if input.just_pressed(KeyCode::KeyV) {
        for mut visibility in root_node_query.iter_mut() {
            // Toggle the UI root node's visibility
            visibility.toggle_inherited_hidden();
        }
    }
}
```

You can also use `UiDebugOverlay` to set the overlay's line width:
```
    debug_overlay.line_width = 2.;
```

### Example
To run the example use:
```
cargo run --example debug_overlay
```
The keys to control the example's debug overlay are on listed on the left side panel.



