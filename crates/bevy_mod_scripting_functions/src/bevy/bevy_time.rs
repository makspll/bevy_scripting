// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
use bevy_mod_scripting_core::{
    AddContextInitializer, StoreDocumentation, bindings::ReflectReference,
};
use bevy_mod_scripting_functions::RegisterScriptFunction;
use crate::*;
pub struct BevyTimeScriptingPlugin;
impl bevy::app::Plugin for BevyTimeScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut world = app.world_mut();
        NamespaceBuilder::<Fixed>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::Fixed>| {
                    let output: Val<bevy::time::prelude::Fixed> = Fixed::clone(_self)
                        .into();
                    output
                },
            );
        NamespaceBuilder::<Real>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::Real>| {
                    let output: Val<bevy::time::prelude::Real> = Real::clone(_self)
                        .into();
                    output
                },
            );
        NamespaceBuilder::<Timer>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: Val<bevy::time::prelude::Timer> = Timer::clone(_self)
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: () = Timer::assert_receiver_is_total_eq(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "from_seconds",
                |duration: f32, mode: Val<bevy::time::prelude::TimerMode>| {
                    let output: Val<bevy::time::prelude::Timer> = Timer::from_seconds(
                            duration,
                            mode,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "finished",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: bool = Timer::finished(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "just_finished",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: bool = Timer::just_finished(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "elapsed_secs",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = Timer::elapsed_secs(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "elapsed_secs_f64",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f64 = Timer::elapsed_secs_f64(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "mode",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: Val<bevy::time::prelude::TimerMode> = Timer::mode(_self)
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "set_mode",
                |
                    _self: Mut<bevy::time::prelude::Timer>,
                    mode: Val<bevy::time::prelude::TimerMode>|
                {
                    let output: () = Timer::set_mode(_self, mode).into();
                    output
                },
            )
            .overwrite_script_function(
                "pause",
                |_self: Mut<bevy::time::prelude::Timer>| {
                    let output: () = Timer::pause(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "unpause",
                |_self: Mut<bevy::time::prelude::Timer>| {
                    let output: () = Timer::unpause(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "paused",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: bool = Timer::paused(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "reset",
                |_self: Mut<bevy::time::prelude::Timer>| {
                    let output: () = Timer::reset(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "fraction",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = Timer::fraction(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "fraction_remaining",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = Timer::fraction_remaining(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "remaining_secs",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = Timer::remaining_secs(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "times_finished_this_tick",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: u32 = Timer::times_finished_this_tick(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "eq",
                |
                    _self: Ref<bevy::time::prelude::Timer>,
                    other: Ref<bevy::time::prelude::Timer>|
                {
                    let output: bool = Timer::eq(_self, other).into();
                    output
                },
            );
        NamespaceBuilder::<TimerMode>::new(world)
            .overwrite_script_function(
                "eq",
                |
                    _self: Ref<bevy::time::prelude::TimerMode>,
                    other: Ref<bevy::time::prelude::TimerMode>|
                {
                    let output: bool = TimerMode::eq(_self, other).into();
                    output
                },
            )
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::TimerMode>| {
                    let output: Val<bevy::time::prelude::TimerMode> = TimerMode::clone(
                            _self,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::time::prelude::TimerMode>| {
                    let output: () = TimerMode::assert_receiver_is_total_eq(_self)
                        .into();
                    output
                },
            );
        NamespaceBuilder::<Virtual>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::Virtual>| {
                    let output: Val<bevy::time::prelude::Virtual> = Virtual::clone(_self)
                        .into();
                    output
                },
            );
        NamespaceBuilder::<Stopwatch>::new(world)
            .overwrite_script_function(
                "new",
                || {
                    let output: Val<bevy::time::Stopwatch> = Stopwatch::new().into();
                    output
                },
            )
            .overwrite_script_function(
                "elapsed_secs",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: f32 = Stopwatch::elapsed_secs(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "elapsed_secs_f64",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: f64 = Stopwatch::elapsed_secs_f64(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "pause",
                |_self: Mut<bevy::time::Stopwatch>| {
                    let output: () = Stopwatch::pause(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "unpause",
                |_self: Mut<bevy::time::Stopwatch>| {
                    let output: () = Stopwatch::unpause(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "is_paused",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: bool = Stopwatch::is_paused(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "reset",
                |_self: Mut<bevy::time::Stopwatch>| {
                    let output: () = Stopwatch::reset(_self).into();
                    output
                },
            )
            .overwrite_script_function(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: () = Stopwatch::assert_receiver_is_total_eq(_self)
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: Val<bevy::time::Stopwatch> = Stopwatch::clone(_self)
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "eq",
                |_self: Ref<bevy::time::Stopwatch>, other: Ref<bevy::time::Stopwatch>| {
                    let output: bool = Stopwatch::eq(_self, other).into();
                    output
                },
            );
    }
}
