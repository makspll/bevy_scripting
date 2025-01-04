#![allow(deprecated)]
use std::sync::Mutex;

use ansi_parser::AnsiParser;
use asset::ScriptAsset;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    image::ImageSampler,
    log::LogPlugin,
    prelude::*,
    reflect::Reflect,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::{PrimaryWindow, WindowResized},
};

use bevy_console::{
    make_layer, send_log_buffer_to_console, AddConsoleCommand, ConsoleCommand,
    ConsoleConfiguration, ConsoleOpen, ConsolePlugin, PrintConsoleLine,
};
use bevy_mod_scripting::{prelude::*, ScriptFunctionsPlugin};
use bevy_mod_scripting_lua::LuaScriptingPlugin;
use bindings::script_value::ScriptValue;
use clap::Parser;
use commands::DeleteScript;
use script::ScriptComponent;

// CONSOLE SETUP

fn console_app(app: &mut App) -> &mut App {
    // forward logs to the console
    app.add_plugins((
        DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "error,game_of_life=info".to_owned(),
            custom_layer: make_layer,
        }),
        ConsolePlugin,
    ))
    .add_console_command::<GameOfLifeCommand, _>(run_script_cmd)
    .add_systems(Startup, |mut open: ResMut<ConsoleOpen>| {
        open.open = true;
    })
}

fn run_script_cmd(
    mut log: ConsoleCommand<GameOfLifeCommand>,
    mut commands: Commands,
    mut loaded_scripts: ResMut<LoadedScripts>,
) {
    if let Some(Ok(command)) = log.take() {
        match command {
            GameOfLifeCommand::Start => {
                // create an entity with the script component
                bevy::log::info!("Starting game of life!");
                commands.spawn(ScriptComponent::new(
                    vec!["scripts/game_of_life.lua".into()],
                ));
            }
            GameOfLifeCommand::Stop => {
                // we can simply drop the handle, or manually delete, I'll just drop the handle
                bevy::log::info!("Stopping game of life!");

                // I am not mapping the handles to the script names, so I'll just clear the entire list
                loaded_scripts.0.clear();

                // you could also do:
                // commands.queue(DeleteScript::<LuaScriptingPlugin>::new(
                //     "scripts/game_of_life.lua".into(),
                // ));
            }
        }
    }
}

// we use bevy-debug-console to demonstrate how this can fit in in the runtime of a game
// note that using just the entity id instead of the full Entity has issues,
// but since we aren't despawning/spawning entities this works in our case
#[derive(Parser, ConsoleCommand)]
#[command(name = "gol")]
///Runs a Lua script from the `assets/scripts` directory
pub enum GameOfLifeCommand {
    Start,
    Stop,
}

// GAME OF LIFE
fn game_of_life_app(app: &mut App) -> &mut App {
    app.insert_resource(Time::<Fixed>::from_seconds(UPDATE_FREQUENCY.into()))
        .add_plugins((
            // for FPS counters
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            // for scripting
            LuaScriptingPlugin::default(),
            ScriptFunctionsPlugin,
        ))
        .register_type::<LifeState>()
        .register_type::<Settings>()
        .init_resource::<Settings>()
        .init_resource::<LoadedScripts>()
        .add_systems(Startup, (init_game_of_life_state, load_script_assets))
        .add_systems(Update, sync_window_size)
        .add_systems(
            FixedUpdate,
            (
                update_rendered_state.after(sync_window_size),
                send_on_update.after(update_rendered_state),
                event_handler::<OnUpdate, LuaScriptingPlugin>,
            ),
        )
}

#[derive(Debug, Default, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct LifeState {
    pub cells: Vec<u8>,
}

#[derive(Debug, Resource, Default)]
pub struct LoadedScripts(pub Vec<Handle<ScriptAsset>>);

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct Settings {
    physical_grid_dimensions: (u32, u32),
    display_grid_dimensions: (u32, u32),
    border_thickness: u32,
    live_color: u8,
    dead_color: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            border_thickness: 1,
            live_color: 255u8,
            dead_color: 0u8,
            physical_grid_dimensions: (88, 50),
            display_grid_dimensions: (0, 0),
        }
    }
}

/// Prepares any scripts by loading them and storing the handles.
pub fn load_script_assets(
    asset_server: Res<AssetServer>,
    mut loaded_scripts: ResMut<LoadedScripts>,
) {
    loaded_scripts
        .0
        .push(asset_server.load("scripts/game_of_life.lua"));
}

pub fn init_game_of_life_state(
    mut commands: Commands,
    mut assets: ResMut<Assets<Image>>,
    settings: Res<Settings>,
) {
    let mut image = Image::new_fill(
        Extent3d {
            width: settings.physical_grid_dimensions.0,
            height: settings.physical_grid_dimensions.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0u8],
        TextureFormat::R8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    image.sampler = ImageSampler::nearest();

    commands.spawn(Camera2d);
    commands
        .spawn(Sprite {
            image: assets.add(image),
            custom_size: Some(Vec2::new(
                settings.display_grid_dimensions.0 as f32,
                settings.display_grid_dimensions.1 as f32,
            )),
            color: Color::srgb(1.0, 0.388, 0.278), // TOMATO
            ..Default::default()
        })
        .insert(LifeState {
            cells: vec![
                0u8;
                (settings.physical_grid_dimensions.0 * settings.physical_grid_dimensions.1)
                    as usize
            ],
        });

    bevy::log::info!("Game of life was initialized. use `gol start` to start the game!");
}

pub fn sync_window_size(
    mut resize_event: EventReader<WindowResized>,
    mut settings: ResMut<Settings>,
    mut query: Query<&mut Sprite, With<LifeState>>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(e) = resize_event
        .read()
        .filter(|e| primary_windows.get(e.window).is_ok())
        .last()
    {
        let primary_window = primary_windows.get(e.window).unwrap();
        settings.display_grid_dimensions = (
            primary_window.physical_width(),
            primary_window.physical_height(),
        );

        // resize all game's of life, retain aspect ratio and fit the entire game in the window
        for mut sprite in query.iter_mut() {
            let scale = if settings.physical_grid_dimensions.0 > settings.physical_grid_dimensions.1
            {
                // horizontal is longer
                settings.display_grid_dimensions.1 as f32
                    / settings.physical_grid_dimensions.1 as f32
            } else {
                // vertical is longer
                settings.display_grid_dimensions.0 as f32
                    / settings.physical_grid_dimensions.0 as f32
            };

            sprite.custom_size = Some(Vec2::new(
                (settings.physical_grid_dimensions.0 as f32) * scale,
                (settings.physical_grid_dimensions.1 as f32) * scale,
            ));
        }
    }
}

/// Runs after LifeState components are updated, updates their rendered representation
pub fn update_rendered_state(
    mut assets: ResMut<Assets<Image>>,
    query: Query<(&LifeState, &Sprite)>,
) {
    for (new_state, old_rendered_state) in query.iter() {
        let old_rendered_state = assets
            .get_mut(&old_rendered_state.image)
            .expect("World is not setup correctly");

        old_rendered_state.data = new_state.cells.clone();
    }
}

callback_labels!(
    OnUpdate => "on_update",
    Init => "init"
);

/// Sends events allowing scripts to drive update logic
pub fn send_on_update(mut events: EventWriter<ScriptCallbackEvent>) {
    events.send(ScriptCallbackEvent::new_for_all(
        OnUpdate,
        vec![ScriptValue::Unit],
    ));
}

const UPDATE_FREQUENCY: f32 = 1.0 / 60.0;

// MAIN

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    console_app(&mut app);
    game_of_life_app(&mut app);

    app.run();

    Ok(())
}
