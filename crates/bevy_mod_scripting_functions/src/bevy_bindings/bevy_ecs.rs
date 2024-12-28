// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_reflect::*;
use bevy_mod_scripting_core::{
    AddContextInitializer, StoreDocumentation,
    bindings::{ReflectReference, function::from::{Ref, Mut, Val}},
};
use crate::*;
pub struct BevyEcsScriptingPlugin;
impl ::bevy::app::Plugin for BevyEcsScriptingPlugin {
    fn build(&self, app: &mut ::bevy::prelude::App) {
        let mut world = app.world_mut();
        NamespaceBuilder::<::bevy::ecs::entity::Entity>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::ecs::entity::Entity>| {
                    let output: Val<bevy::ecs::entity::Entity> = ::bevy::ecs::entity::Entity::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "eq",
                |
                    _self: Ref<bevy::ecs::entity::Entity>,
                    other: Ref<bevy::ecs::entity::Entity>|
                {
                    let output: bool = ::bevy::ecs::entity::Entity::eq(&_self, &other)
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "from_raw",
                |index: u32| {
                    let output: Val<bevy::ecs::entity::Entity> = ::bevy::ecs::entity::Entity::from_raw(
                            index,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "to_bits",
                |_self: Val<bevy::ecs::entity::Entity>| {
                    let output: u64 = ::bevy::ecs::entity::Entity::to_bits(
                            _self.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "from_bits",
                |bits: u64| {
                    let output: Val<bevy::ecs::entity::Entity> = ::bevy::ecs::entity::Entity::from_bits(
                            bits,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "index",
                |_self: Val<bevy::ecs::entity::Entity>| {
                    let output: u32 = ::bevy::ecs::entity::Entity::index(
                            _self.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "generation",
                |_self: Val<bevy::ecs::entity::Entity>| {
                    let output: u32 = ::bevy::ecs::entity::Entity::generation(
                            _self.into_inner(),
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::ecs::world::OnAdd>::new(world);
        NamespaceBuilder::<::bevy::ecs::world::OnInsert>::new(world);
        NamespaceBuilder::<::bevy::ecs::world::OnRemove>::new(world);
        NamespaceBuilder::<::bevy::ecs::world::OnReplace>::new(world);
        NamespaceBuilder::<::bevy::ecs::component::ComponentId>::new(world)
            .overwrite_script_function(
                "eq",
                |
                    _self: Ref<bevy::ecs::component::ComponentId>,
                    other: Ref<bevy::ecs::component::ComponentId>|
                {
                    let output: bool = ::bevy::ecs::component::ComponentId::eq(
                            &_self,
                            &other,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::ecs::component::ComponentId>| {
                    let output: Val<bevy::ecs::component::ComponentId> = ::bevy::ecs::component::ComponentId::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::ecs::component::ComponentId>| {
                    let output: () = ::bevy::ecs::component::ComponentId::assert_receiver_is_total_eq(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "new",
                |index: usize| {
                    let output: Val<bevy::ecs::component::ComponentId> = ::bevy::ecs::component::ComponentId::new(
                            index,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "index",
                |_self: Val<bevy::ecs::component::ComponentId>| {
                    let output: usize = ::bevy::ecs::component::ComponentId::index(
                            _self.into_inner(),
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::ecs::component::Tick>::new(world)
            .overwrite_script_function(
                "new",
                |tick: u32| {
                    let output: Val<bevy::ecs::component::Tick> = ::bevy::ecs::component::Tick::new(
                            tick,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "get",
                |_self: Val<bevy::ecs::component::Tick>| {
                    let output: u32 = ::bevy::ecs::component::Tick::get(
                            _self.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "set",
                |mut _self: Mut<bevy::ecs::component::Tick>, tick: u32| {
                    let output: () = ::bevy::ecs::component::Tick::set(&mut _self, tick)
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "is_newer_than",
                |
                    _self: Val<bevy::ecs::component::Tick>,
                    last_run: Val<bevy::ecs::component::Tick>,
                    this_run: Val<bevy::ecs::component::Tick>|
                {
                    let output: bool = ::bevy::ecs::component::Tick::is_newer_than(
                            _self.into_inner(),
                            last_run.into_inner(),
                            this_run.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "eq",
                |
                    _self: Ref<bevy::ecs::component::Tick>,
                    other: Ref<bevy::ecs::component::Tick>|
                {
                    let output: bool = ::bevy::ecs::component::Tick::eq(&_self, &other)
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::ecs::component::Tick>| {
                    let output: () = ::bevy::ecs::component::Tick::assert_receiver_is_total_eq(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::ecs::component::Tick>| {
                    let output: Val<bevy::ecs::component::Tick> = ::bevy::ecs::component::Tick::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::ecs::component::ComponentTicks>::new(world)
            .overwrite_script_function(
                "is_added",
                |
                    _self: Ref<bevy::ecs::component::ComponentTicks>,
                    last_run: Val<bevy::ecs::component::Tick>,
                    this_run: Val<bevy::ecs::component::Tick>|
                {
                    let output: bool = ::bevy::ecs::component::ComponentTicks::is_added(
                            &_self,
                            last_run.into_inner(),
                            this_run.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "is_changed",
                |
                    _self: Ref<bevy::ecs::component::ComponentTicks>,
                    last_run: Val<bevy::ecs::component::Tick>,
                    this_run: Val<bevy::ecs::component::Tick>|
                {
                    let output: bool = ::bevy::ecs::component::ComponentTicks::is_changed(
                            &_self,
                            last_run.into_inner(),
                            this_run.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "new",
                |change_tick: Val<bevy::ecs::component::Tick>| {
                    let output: Val<bevy::ecs::component::ComponentTicks> = ::bevy::ecs::component::ComponentTicks::new(
                            change_tick.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "set_changed",
                |
                    mut _self: Mut<bevy::ecs::component::ComponentTicks>,
                    change_tick: Val<bevy::ecs::component::Tick>|
                {
                    let output: () = ::bevy::ecs::component::ComponentTicks::set_changed(
                            &mut _self,
                            change_tick.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::ecs::component::ComponentTicks>| {
                    let output: Val<bevy::ecs::component::ComponentTicks> = ::bevy::ecs::component::ComponentTicks::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::ecs::identifier::Identifier>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::ecs::identifier::Identifier>| {
                    let output: Val<bevy::ecs::identifier::Identifier> = ::bevy::ecs::identifier::Identifier::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "low",
                |_self: Val<bevy::ecs::identifier::Identifier>| {
                    let output: u32 = ::bevy::ecs::identifier::Identifier::low(
                            _self.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "masked_high",
                |_self: Val<bevy::ecs::identifier::Identifier>| {
                    let output: u32 = ::bevy::ecs::identifier::Identifier::masked_high(
                            _self.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "to_bits",
                |_self: Val<bevy::ecs::identifier::Identifier>| {
                    let output: u64 = ::bevy::ecs::identifier::Identifier::to_bits(
                            _self.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "from_bits",
                |value: u64| {
                    let output: Val<bevy::ecs::identifier::Identifier> = ::bevy::ecs::identifier::Identifier::from_bits(
                            value,
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "eq",
                |
                    _self: Ref<bevy::ecs::identifier::Identifier>,
                    other: Ref<bevy::ecs::identifier::Identifier>|
                {
                    let output: bool = ::bevy::ecs::identifier::Identifier::eq(
                            &_self,
                            &other,
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::ecs::entity::EntityHash>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::ecs::entity::EntityHash>| {
                    let output: Val<bevy::ecs::entity::EntityHash> = ::bevy::ecs::entity::EntityHash::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<
            ::bevy::ecs::removal_detection::RemovedComponentEntity,
        >::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::ecs::removal_detection::RemovedComponentEntity>| {
                    let output: Val<
                        bevy::ecs::removal_detection::RemovedComponentEntity,
                    > = ::bevy::ecs::removal_detection::RemovedComponentEntity::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::ecs::system::SystemIdMarker>::new(world);
    }
}
