#![allow(clippy::all, unused_imports, deprecated, dead_code)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
use super::bevy_input::*;
extern crate self as bevy_script_api;
use bevy_script_api::{lua::RegisterForeignLuaType, ReflectedValue};
/// An event that is sent whenever the user's cursor enters a window.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::CursorEntered",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::CursorEntered;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::CursorEntered) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct CursorEntered {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
}
/// The icon to display for a [`Window`](crate::window::Window)'s [`Cursor`](crate::window::Cursor).
/// Examples of all of these cursors can be found [here](https://www.w3schools.com/cssref/playit.php?filename=playcss_cursor&preval=crosshair).
/// This `enum` is simply a copy of a similar `enum` found in [`winit`](https://docs.rs/winit/latest/winit/window/enum.CursorIcon.html).
/// `winit`, in turn, is based upon the [CSS3 UI spec](https://www.w3.org/TR/css-ui-3/#cursor).
/// See the [`window_settings`] example for usage.
/// [`window_settings`]: https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::CursorIcon",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::CursorIcon;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &cursor::CursorIcon) -> bool;

"#]
)]
pub struct CursorIcon {}
/// An event that is sent whenever the user's cursor leaves a window.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::CursorLeft",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::CursorLeft;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::CursorLeft) -> bool;

"#]
)]
pub struct CursorLeft {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
}
/// An event reporting that the mouse cursor has moved inside a window.
/// The event is sent only if the cursor is over one of the application's windows.
/// It is the translated version of [`WindowEvent::CursorMoved`] from the `winit` crate with the addition of `delta`.
/// Not to be confused with the [`MouseMotion`] event from `bevy_input`.
/// Because the range of data is limited by the window area and it may have been transformed by the OS to implement certain effects like acceleration,
/// you should not use it for non-cursor-like behaviour such as 3D camera control. Please see [`MouseMotion`] instead.
/// [`WindowEvent::CursorMoved`]: https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.CursorMoved
/// [`MouseMotion`]: bevy_input::mouse::MouseMotion
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::CursorMoved",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::CursorMoved;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::CursorMoved) -> bool;

"#]
)]
pub struct CursorMoved {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    #[lua(output(proxy))]
    position: bevy::math::Vec2,
    delta: ReflectedValue,
}
/// Events related to files being dragged and dropped on a window.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::FileDragAndDrop",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::FileDragAndDrop) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::FileDragAndDrop;

"#]
)]
pub struct FileDragAndDrop {}
/// A Input Method Editor event.
/// This event is the translated version of the `WindowEvent::Ime` from the `winit` crate.
/// It is only sent if IME was enabled on the window with [`Window::ime_enabled`](crate::window::Window::ime_enabled).
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::Ime",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::Ime;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::Ime) -> bool;

"#]
)]
pub struct Ime {}
/// References a screen monitor.
/// Used when centering a [`Window`] on a monitor.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::MonitorSelection",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::MonitorSelection) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::MonitorSelection;

"#]
)]
pub struct MonitorSelection {}
/// An event that is sent whenever a window receives a character from the OS or underlying system.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::ReceivedCharacter",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::ReceivedCharacter;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::ReceivedCharacter) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct ReceivedCharacter {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    #[lua(output(proxy))]
    char: smol_str::SmolStr,
}
/// The defining [`Component`] for window entities,
/// storing information about how it should appear and behave.
/// Each window corresponds to an entity, and is uniquely identified by the value of their [`Entity`].
/// When the [`Window`] component is added to an entity, a new window will be opened.
/// When it is removed or the entity is despawned, the window will close.
/// The primary window entity (and the corresponding window) is spawned by default
/// by [`WindowPlugin`](crate::WindowPlugin) and is marked with the [`PrimaryWindow`] component.
/// This component is synchronized with `winit` through `bevy_winit`:
/// it will reflect the current state of the window and can be modified to change this state.
/// # Example
/// Because this component is synchronized with `winit`, it can be used to perform
/// OS-integrated windowing operations. For example, here's a simple system
/// to change the cursor type:
/// ```
/// # use bevy_ecs::query::With;
/// # use bevy_ecs::system::Query;
/// # use bevy_window::{CursorIcon, PrimaryWindow, Window};
/// fn change_cursor(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
///     // Query returns one window typically.
///     for mut window in windows.iter_mut() {
///         window.cursor.icon = CursorIcon::Wait;
///     }
/// }
/// ```
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::Window",
    functions[r#"
/// Setting to true will attempt to maximize the window.
/// Setting to false will attempt to un-maximize the window.

    #[lua(kind = "MutatingMethod")]
    fn set_maximized(&mut self, maximized: bool) -> ();

"#,
    r#"
/// Setting to true will attempt to minimize the window.
/// Setting to false will attempt to un-minimize the window.

    #[lua(kind = "MutatingMethod")]
    fn set_minimized(&mut self, minimized: bool) -> ();

"#,
    r#"
/// The window's client area width in logical pixels.
/// See [`WindowResolution`] for an explanation about logical/physical sizes.

    #[lua(kind = "Method")]
    fn width(&self) -> f32;

"#,
    r#"
/// The window's client area height in logical pixels.
/// See [`WindowResolution`] for an explanation about logical/physical sizes.

    #[lua(kind = "Method")]
    fn height(&self) -> f32;

"#,
    r#"
/// The window's client area width in physical pixels.
/// See [`WindowResolution`] for an explanation about logical/physical sizes.

    #[lua(kind = "Method")]
    fn physical_width(&self) -> u32;

"#,
    r#"
/// The window's client area height in physical pixels.
/// See [`WindowResolution`] for an explanation about logical/physical sizes.

    #[lua(kind = "Method")]
    fn physical_height(&self) -> u32;

"#,
    r#"
/// The window's scale factor.
/// Ratio of physical size to logical size, see [`WindowResolution`].

    #[lua(kind = "Method")]
    fn scale_factor(&self) -> f32;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::Window;

"#]
)]
pub struct Window {
    #[lua(output(proxy))]
    cursor: bevy::window::Cursor,
    #[lua(output(proxy))]
    present_mode: bevy::window::PresentMode,
    #[lua(output(proxy))]
    mode: bevy::window::WindowMode,
    #[lua(output(proxy))]
    position: bevy::window::prelude::WindowPosition,
    #[lua(output(proxy))]
    resolution: bevy::window::WindowResolution,
    title: std::string::String,
    name: std::option::Option<std::string::String>,
    #[lua(output(proxy))]
    composite_alpha_mode: bevy::window::CompositeAlphaMode,
    #[lua(output(proxy))]
    resize_constraints: bevy::window::prelude::WindowResizeConstraints,
    resizable: bool,
    #[lua(output(proxy))]
    enabled_buttons: bevy::window::EnabledButtons,
    decorations: bool,
    transparent: bool,
    focused: bool,
    #[lua(output(proxy))]
    window_level: bevy::window::WindowLevel,
    canvas: std::option::Option<std::string::String>,
    prevent_default_event_handling: bool,
    #[lua(output(proxy))]
    internal: bevy::window::InternalWindowState,
    ime_enabled: bool,
    #[lua(output(proxy))]
    ime_position: bevy::math::Vec2,
    window_theme: ReflectedValue,
    visible: bool,
}
/// An event that is sent when a window is repositioned in physical pixels.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::WindowMoved",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::WindowMoved;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowMoved) -> bool;

"#]
)]
pub struct WindowMoved {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    #[lua(output(proxy))]
    position: bevy::math::IVec2,
}
/// Defines where a [`Window`] should be placed on the screen.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::WindowPosition",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::WindowPosition;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::WindowPosition) -> bool;

"#,
    r#"
/// Creates a new [`WindowPosition`] at a position.

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        position: bevy::math::IVec2,
    ) -> bevy::window::prelude::WindowPosition;

"#,
    r#"
/// Set the position to a specific point.

    #[lua(kind = "MutatingMethod")]
    fn set(&mut self, #[proxy] position: bevy::math::IVec2) -> ();

"#,
    r#"
/// Set the window to a specific monitor.

    #[lua(kind = "MutatingMethod")]
    fn center(
        &mut self,
        #[proxy]
        monitor: bevy::window::prelude::MonitorSelection,
    ) -> ();

"#]
)]
pub struct WindowPosition {}
/// The size limits on a [`Window`].
/// These values are measured in logical pixels (see [`WindowResolution`]), so the user's
/// scale factor does affect the size limits on the window.
/// Please note that if the window is resizable, then when the window is
/// maximized it may have a size outside of these limits. The functionality
/// required to disable maximizing is not yet exposed by winit.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::prelude::WindowResizeConstraints",
    functions[r#"
/// Checks if the constraints are valid.
/// Will output warnings if it isn't.

    #[lua(kind = "Method", output(proxy))]
    fn check_constraints(&self) -> bevy::window::prelude::WindowResizeConstraints;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::prelude::WindowResizeConstraints;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::WindowResizeConstraints) -> bool;

"#]
)]
pub struct WindowResizeConstraints {
    min_width: f32,
    min_height: f32,
    max_width: f32,
    max_height: f32,
}
/// A window event that is sent whenever a window's logical size has changed.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowResized",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowResized) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowResized;

"#]
)]
pub struct WindowResized {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    width: f32,
    height: f32,
}
/// An event that is sent whenever a new window is created.
/// To create a new window, spawn an entity with a [`crate::Window`] on it.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowCreated",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowCreated;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowCreated) -> bool;

"#]
)]
pub struct WindowCreated {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
}
/// An event that is sent whenever a window is closed. This will be sent when
/// the window entity loses its [`Window`](crate::window::Window) component or is despawned.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowClosed",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowClosed) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowClosed;

"#]
)]
pub struct WindowClosed {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
}
/// An event that is sent whenever the operating systems requests that a window
/// be closed. This will be sent when the close button of the window is pressed.
/// If the default [`WindowPlugin`] is used, these events are handled
/// by closing the corresponding [`Window`].
/// To disable this behavior, set `close_when_requested` on the [`WindowPlugin`]
/// to `false`.
/// [`WindowPlugin`]: crate::WindowPlugin
/// [`Window`]: crate::Window
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowCloseRequested",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowCloseRequested;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowCloseRequested) -> bool;

"#]
)]
pub struct WindowCloseRequested {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
}
/// An event that is sent whenever a window is destroyed by the underlying window system.
/// Note that if your application only has a single window, this event may be your last chance to
/// persist state before the application terminates.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowDestroyed",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowDestroyed;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowDestroyed) -> bool;

"#]
)]
pub struct WindowDestroyed {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
}
/// An event that indicates all of the application's windows should be redrawn,
/// even if their control flow is set to `Wait` and there have been no window events.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::RequestRedraw",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::RequestRedraw) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::RequestRedraw;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct RequestRedraw {}
/// An event that indicates a window has received or lost focus.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowFocused",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowFocused;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowFocused) -> bool;

"#]
)]
pub struct WindowFocused {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    focused: bool,
}
/// The window has been occluded (completely hidden from view).
/// This is different to window visibility as it depends on
/// whether the window is closed, minimised, set invisible,
/// or fully occluded by another window.
/// It is the translated version of [`WindowEvent::Occluded`] from the `winit` crate.
/// [`WindowEvent::Occluded`]: https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.Occluded
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowOccluded",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowOccluded;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowOccluded) -> bool;

"#]
)]
pub struct WindowOccluded {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    occluded: bool,
}
/// An event that indicates a window's scale factor has changed.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowScaleFactorChanged",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowScaleFactorChanged;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowScaleFactorChanged) -> bool;

"#]
)]
pub struct WindowScaleFactorChanged {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    scale_factor: f64,
}
/// An event that indicates a window's OS-reported scale factor has changed.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowBackendScaleFactorChanged",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowBackendScaleFactorChanged) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowBackendScaleFactorChanged;

"#]
)]
pub struct WindowBackendScaleFactorChanged {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    scale_factor: f64,
}
/// An event sent when the system theme changes for a window.
/// This event is only sent when the window is relying on the system theme to control its appearance.
/// i.e. It is only sent when [`Window::window_theme`](crate::window::Window::window_theme) is `None` and the system theme changes.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowThemeChanged",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::WindowThemeChanged) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowThemeChanged;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct WindowThemeChanged {
    #[lua(output(proxy))]
    window: bevy::ecs::entity::Entity,
    #[lua(output(proxy))]
    theme: bevy::window::WindowTheme,
}
/// Application lifetime events
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::ApplicationLifetime",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::ApplicationLifetime;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &event::ApplicationLifetime) -> bool;

"#]
)]
pub struct ApplicationLifetime {}
/// Marker [`Component`] for the window considered the primary window.
/// Currently this is assumed to only exist on 1 entity at a time.
/// [`WindowPlugin`](crate::WindowPlugin) will spawn a [`Window`] entity
/// with this component if [`primary_window`](crate::WindowPlugin::primary_window)
/// is `Some`.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::PrimaryWindow",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::PrimaryWindow;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::PrimaryWindow) -> bool;

"#]
)]
pub struct PrimaryWindow {}
/// Cursor data for a [`Window`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::Cursor",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::Cursor;

"#]
)]
pub struct Cursor {
    #[lua(output(proxy))]
    icon: bevy::window::prelude::CursorIcon,
    visible: bool,
    #[lua(output(proxy))]
    grab_mode: bevy::window::CursorGrabMode,
    hit_test: bool,
}
/// Defines if and how the [`Cursor`] is grabbed by a [`Window`].
/// ## Platform-specific
/// - **`Windows`** doesn't support [`CursorGrabMode::Locked`]
/// - **`macOS`** doesn't support [`CursorGrabMode::Confined`]
/// - **`iOS/Android`** don't have cursors.
/// Since `Windows` and `macOS` have different [`CursorGrabMode`] support, we first try to set the grab mode that was asked for. If it doesn't work then use the alternate grab mode.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::CursorGrabMode",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::CursorGrabMode) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::CursorGrabMode;

"#]
)]
pub struct CursorGrabMode {}
/// Specifies how the alpha channel of the textures should be handled during compositing, for a [`Window`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::CompositeAlphaMode",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::CompositeAlphaMode) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::CompositeAlphaMode;

"#]
)]
pub struct CompositeAlphaMode {}
/// Controls the size of a [`Window`]
/// ## Physical, logical and requested sizes
/// There are three sizes associated with a window:
/// - the physical size,
/// which represents the actual height and width in physical pixels
/// the window occupies on the monitor,
/// - the logical size,
/// which represents the size that should be used to scale elements
/// inside the window, measured in logical pixels,
/// - the requested size,
/// measured in logical pixels, which is the value submitted
/// to the API when creating the window, or requesting that it be resized.
/// ## Scale factor
/// The reason logical size and physical size are separated and can be different
/// is to account for the cases where:
/// - several monitors have different pixel densities,
/// - the user has set up a pixel density preference in its operating system,
/// - the Bevy `App` has specified a specific scale factor between both.
/// The factor between physical size and logical size can be retrieved with
/// [`WindowResolution::scale_factor`].
/// For the first two cases, a scale factor is set automatically by the operating
/// system through the window backend. You can get it with
/// [`WindowResolution::base_scale_factor`].
/// For the third case, you can override this automatic scale factor with
/// [`WindowResolution::set_scale_factor_override`].
/// ## Requested and obtained sizes
/// The logical size should be equal to the requested size after creating/resizing,
/// when possible.
/// The reason the requested size and logical size might be different
/// is because the corresponding physical size might exceed limits (either the
/// size limits of the monitor, or limits defined in [`WindowResizeConstraints`]).
/// Note: The requested size is not kept in memory, for example requesting a size
/// too big for the screen, making the logical size different from the requested size,
/// and then setting a scale factor that makes the previous requested size within
/// the limits of the screen will not get back that previous requested size.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowResolution",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::WindowResolution) -> bool;

"#,
    r#"
/// Creates a new [`WindowResolution`].

    #[lua(kind = "Function", output(proxy))]
    fn new(logical_width: f32, logical_height: f32) -> bevy::window::WindowResolution;

"#,
    r#"
/// Builder method for adding a scale factor override to the resolution.

    #[lua(kind = "Method", output(proxy))]
    fn with_scale_factor_override(
        self,
        scale_factor_override: f32,
    ) -> bevy::window::WindowResolution;

"#,
    r#"
/// The window's client area width in logical pixels.

    #[lua(kind = "Method")]
    fn width(&self) -> f32;

"#,
    r#"
/// The window's client area height in logical pixels.

    #[lua(kind = "Method")]
    fn height(&self) -> f32;

"#,
    r#"
/// The window's client area width in physical pixels.

    #[lua(kind = "Method")]
    fn physical_width(&self) -> u32;

"#,
    r#"
/// The window's client area height in physical pixels.

    #[lua(kind = "Method")]
    fn physical_height(&self) -> u32;

"#,
    r#"
/// The ratio of physical pixels to logical pixels.
/// `physical_pixels = logical_pixels * scale_factor`

    #[lua(kind = "Method")]
    fn scale_factor(&self) -> f32;

"#,
    r#"
/// The window scale factor as reported by the window backend.
/// This value is unaffected by [`WindowResolution::scale_factor_override`].

    #[lua(kind = "Method")]
    fn base_scale_factor(&self) -> f32;

"#,
    r#"
/// The scale factor set with [`WindowResolution::set_scale_factor_override`].
/// This value may be different from the scale factor reported by the window backend.

    #[lua(kind = "Method")]
    fn scale_factor_override(&self) -> std::option::Option<f32>;

"#,
    r#"
/// Set the window's logical resolution.

    #[lua(kind = "MutatingMethod")]
    fn set(&mut self, width: f32, height: f32) -> ();

"#,
    r#"
/// Set the window's physical resolution.
/// This will ignore the scale factor setting, so most of the time you should
/// prefer to use [`WindowResolution::set`].

    #[lua(kind = "MutatingMethod")]
    fn set_physical_resolution(&mut self, width: u32, height: u32) -> ();

"#,
    r#"
/// Set the window's scale factor, this may get overridden by the backend.

    #[lua(kind = "MutatingMethod")]
    fn set_scale_factor(&mut self, scale_factor: f32) -> ();

"#,
    r#"
/// Set the window's scale factor, this will be used over what the backend decides.
/// This can change the logical and physical sizes if the resulting physical
/// size is not within the limits.

    #[lua(kind = "MutatingMethod")]
    fn set_scale_factor_override(
        &mut self,
        scale_factor_override: std::option::Option<f32>,
    ) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowResolution;

"#]
)]
pub struct WindowResolution {}
/// Defines the way a [`Window`] is displayed.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowMode",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::WindowMode) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowMode;

"#]
)]
pub struct WindowMode {}
/// Specifies where a [`Window`] should appear relative to other overlapping windows (on top or under) .
/// Levels are groups of windows with respect to their z-position.
/// The relative ordering between windows in different window levels is fixed.
/// The z-order of windows within the same window level may change dynamically on user interaction.
/// ## Platform-specific
/// - **iOS / Android / Web / Wayland:** Unsupported.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowLevel",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowLevel;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::WindowLevel) -> bool;

"#]
)]
pub struct WindowLevel {}
/// Presentation mode for a [`Window`].
/// The presentation mode specifies when a frame is presented to the window. The [`Fifo`]
/// option corresponds to a traditional `VSync`, where the framerate is capped by the
/// display refresh rate. Both [`Immediate`] and [`Mailbox`] are low-latency and are not
/// capped by the refresh rate, but may not be available on all platforms. Tearing
/// may be observed with [`Immediate`] mode, but will not be observed with [`Mailbox`] or
/// [`Fifo`].
/// [`AutoVsync`] or [`AutoNoVsync`] will gracefully fallback to [`Fifo`] when unavailable.
/// [`Immediate`] or [`Mailbox`] will panic if not supported by the platform.
/// [`Fifo`]: PresentMode::Fifo
/// [`FifoRelaxed`]: PresentMode::FifoRelaxed
/// [`Immediate`]: PresentMode::Immediate
/// [`Mailbox`]: PresentMode::Mailbox
/// [`AutoVsync`]: PresentMode::AutoVsync
/// [`AutoNoVsync`]: PresentMode::AutoNoVsync
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::PresentMode",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::PresentMode) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::PresentMode;

"#]
)]
pub struct PresentMode {}
/// Stores internal [`Window`] state that isn't directly accessible.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::InternalWindowState",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::InternalWindowState) -> bool;

"#,
    r#"
/// Consumes the current maximize request, if it exists. This should only be called by window backends.

    #[lua(kind = "MutatingMethod")]
    fn take_maximize_request(&mut self) -> std::option::Option<bool>;

"#,
    r#"
/// Consumes the current minimize request, if it exists. This should only be called by window backends.

    #[lua(kind = "MutatingMethod")]
    fn take_minimize_request(&mut self) -> std::option::Option<bool>;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::InternalWindowState;

"#]
)]
pub struct InternalWindowState {}
/// The [`Window`] theme variant to use.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowTheme",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowTheme;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::WindowTheme) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct WindowTheme {}
/// Specifies which [`Window`] control buttons should be enabled.
/// ## Platform-specific
/// **`iOS`**, **`Android`**, and the **`Web`** do not have window control buttons.
/// On some **`Linux`** environments these values have no effect.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::EnabledButtons",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::EnabledButtons) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::EnabledButtons;

"#]
)]
pub struct EnabledButtons {
    minimize: bool,
    maximize: bool,
    close: bool,
}
/// Reference to a [`Window`], whether it be a direct link to a specific entity or
/// a more vague defaulting choice.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::WindowRef",
    functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::WindowRef;

"#]
)]
pub struct WindowRef {}
/// A flattened representation of a window reference for equality/hashing purposes.
/// For most purposes you probably want to use the unnormalized version [`WindowRef`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::window::NormalizedWindowRef",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Fetch the entity of this window reference

    #[lua(kind = "Method", output(proxy))]
    fn entity(&self) -> bevy::ecs::entity::Entity;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &window::NormalizedWindowRef) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::window::NormalizedWindowRef;

"#]
)]
pub struct NormalizedWindowRef();
#[derive(Default)]
pub(crate) struct Globals;
impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for Globals {
    fn add_instances<
        'lua,
        T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>,
    >(self, instances: &mut T) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        instances
            .add_instance(
                "WindowPosition",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<
                    LuaWindowPosition,
                >::new,
            )?;
        instances
            .add_instance(
                "WindowResolution",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<
                    LuaWindowResolution,
                >::new,
            )?;
        Ok(())
    }
}
pub struct BevyWindowAPIProvider;
impl bevy_mod_scripting_core::hosts::APIProvider for BevyWindowAPIProvider {
    type APITarget = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = bevy_mod_scripting_lua::docs::LuaDocFragment;
    fn attach_api(
        &mut self,
        ctx: &mut Self::APITarget,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        let ctx = ctx.get_mut().expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(Globals, ctx)
            .map_err(|e| bevy_mod_scripting_core::error::ScriptError::Other(
                e.to_string(),
            ))
    }
    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(
            bevy_mod_scripting_lua::docs::LuaDocFragment::new(
                "BevyWindowAPI",
                |tw| {
                    tw.document_global_instance::<Globals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaCursorEntered>()
                        .process_type::<LuaCursorIcon>()
                        .process_type::<LuaCursorLeft>()
                        .process_type::<LuaCursorMoved>()
                        .process_type::<LuaFileDragAndDrop>()
                        .process_type::<LuaIme>()
                        .process_type::<LuaMonitorSelection>()
                        .process_type::<LuaReceivedCharacter>()
                        .process_type::<LuaWindow>()
                        .process_type::<LuaWindowMoved>()
                        .process_type::<LuaWindowPosition>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaWindowPosition,
                            >,
                        >()
                        .process_type::<LuaWindowResizeConstraints>()
                        .process_type::<LuaWindowResized>()
                        .process_type::<LuaWindowCreated>()
                        .process_type::<LuaWindowClosed>()
                        .process_type::<LuaWindowCloseRequested>()
                        .process_type::<LuaWindowDestroyed>()
                        .process_type::<LuaRequestRedraw>()
                        .process_type::<LuaWindowFocused>()
                        .process_type::<LuaWindowOccluded>()
                        .process_type::<LuaWindowScaleFactorChanged>()
                        .process_type::<LuaWindowBackendScaleFactorChanged>()
                        .process_type::<LuaWindowThemeChanged>()
                        .process_type::<LuaApplicationLifetime>()
                        .process_type::<LuaPrimaryWindow>()
                        .process_type::<LuaCursor>()
                        .process_type::<LuaCursorGrabMode>()
                        .process_type::<LuaCompositeAlphaMode>()
                        .process_type::<LuaWindowResolution>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaWindowResolution,
                            >,
                        >()
                        .process_type::<LuaWindowMode>()
                        .process_type::<LuaWindowLevel>()
                        .process_type::<LuaPresentMode>()
                        .process_type::<LuaInternalWindowState>()
                        .process_type::<LuaWindowTheme>()
                        .process_type::<LuaEnabledButtons>()
                        .process_type::<LuaWindowRef>()
                        .process_type::<LuaNormalizedWindowRef>()
                },
            ),
        )
    }
    fn setup_script(
        &mut self,
        script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }
    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }
    fn register_with_app(&self, app: &mut bevy::app::App) {
        app.register_foreign_lua_type::<bevy::window::prelude::CursorEntered>();
        app.register_foreign_lua_type::<bevy::window::prelude::CursorIcon>();
        app.register_foreign_lua_type::<bevy::window::prelude::CursorLeft>();
        app.register_foreign_lua_type::<bevy::window::prelude::CursorMoved>();
        app.register_foreign_lua_type::<bevy::window::prelude::FileDragAndDrop>();
        app.register_foreign_lua_type::<bevy::window::prelude::Ime>();
        app.register_foreign_lua_type::<bevy::window::prelude::MonitorSelection>();
        app.register_foreign_lua_type::<bevy::window::prelude::ReceivedCharacter>();
        app.register_foreign_lua_type::<bevy::window::prelude::Window>();
        app.register_foreign_lua_type::<bevy::window::prelude::WindowMoved>();
        app.register_foreign_lua_type::<bevy::window::prelude::WindowPosition>();
        app.register_foreign_lua_type::<
                bevy::window::prelude::WindowResizeConstraints,
            >();
        app.register_foreign_lua_type::<bevy::window::WindowResized>();
        app.register_foreign_lua_type::<bevy::window::WindowCreated>();
        app.register_foreign_lua_type::<bevy::window::WindowClosed>();
        app.register_foreign_lua_type::<bevy::window::WindowCloseRequested>();
        app.register_foreign_lua_type::<bevy::window::WindowDestroyed>();
        app.register_foreign_lua_type::<bevy::window::RequestRedraw>();
        app.register_foreign_lua_type::<bevy::window::WindowFocused>();
        app.register_foreign_lua_type::<bevy::window::WindowOccluded>();
        app.register_foreign_lua_type::<bevy::window::WindowScaleFactorChanged>();
        app.register_foreign_lua_type::<bevy::window::WindowBackendScaleFactorChanged>();
        app.register_foreign_lua_type::<bevy::window::WindowThemeChanged>();
        app.register_foreign_lua_type::<bevy::window::ApplicationLifetime>();
        app.register_foreign_lua_type::<bevy::window::PrimaryWindow>();
        app.register_foreign_lua_type::<bevy::window::Cursor>();
        app.register_foreign_lua_type::<bevy::window::CursorGrabMode>();
        app.register_foreign_lua_type::<bevy::window::CompositeAlphaMode>();
        app.register_foreign_lua_type::<bevy::window::WindowResolution>();
        app.register_foreign_lua_type::<bevy::window::WindowMode>();
        app.register_foreign_lua_type::<bevy::window::WindowLevel>();
        app.register_foreign_lua_type::<bevy::window::PresentMode>();
        app.register_foreign_lua_type::<bevy::window::InternalWindowState>();
        app.register_foreign_lua_type::<bevy::window::WindowTheme>();
        app.register_foreign_lua_type::<bevy::window::EnabledButtons>();
        app.register_foreign_lua_type::<bevy::window::WindowRef>();
        app.register_foreign_lua_type::<bevy::window::NormalizedWindowRef>();
    }
}
