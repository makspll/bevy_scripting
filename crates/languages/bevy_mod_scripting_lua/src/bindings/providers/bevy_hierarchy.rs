// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
use super::bevy_core::*;
use bevy_mod_scripting_core::{
    AddContextInitializer, StoreDocumentation, bindings::ReflectReference,
};
use crate::{
    bindings::proxy::{
        LuaReflectRefProxy, LuaReflectRefMutProxy, LuaReflectValProxy, LuaValProxy,
        LuaIdentityProxy,
    },
    RegisterLua, tealr::mlu::mlua::IntoLua,
};
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::hierarchy::prelude::Children",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"
/// Swaps the child at `a_index` with the child at `b_index`.

    #[lua()]
    fn swap(
        _self: LuaReflectRefMutProxy<bevy::hierarchy::prelude::Children>,
        a_index: usize,
        b_index: usize,
    ) -> ();

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Children();
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::hierarchy::prelude::Parent",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq")]
    fn assert_receiver_is_total_eq(
        _self: LuaReflectRefProxy<bevy::hierarchy::prelude::Parent>,
    ) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq::<bevy::hierarchy::prelude::Parent>",
        composite = "eq",
    )]
    fn eq(
        _self: LuaReflectRefProxy<bevy::hierarchy::prelude::Parent>,
        other: LuaReflectRefProxy<bevy::hierarchy::prelude::Parent>,
    ) -> bool;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct Parent();
#[derive(bevy_mod_scripting_derive::LuaProxy)]
#[proxy(
    remote = "bevy::hierarchy::HierarchyEvent",
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq")]
    fn assert_receiver_is_total_eq(
        _self: LuaReflectRefProxy<bevy::hierarchy::HierarchyEvent>,
    ) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq::<bevy::hierarchy::HierarchyEvent>",
        composite = "eq",
    )]
    fn eq(
        _self: LuaReflectRefProxy<bevy::hierarchy::HierarchyEvent>,
        other: LuaReflectRefProxy<bevy::hierarchy::HierarchyEvent>,
    ) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::clone::Clone")]
    fn clone(
        _self: LuaReflectRefProxy<bevy::hierarchy::HierarchyEvent>,
    ) -> LuaReflectValProxy<bevy::hierarchy::HierarchyEvent>;

"#,
    r#"
#[lua(metamethod="ToString")]
fn index(&self) -> String {
    format!("{:?}", _self)
}
"#]
)]
pub struct HierarchyEvent {}
#[derive(Default)]
pub(crate) struct Globals;
impl crate::tealr::mlu::ExportInstances for Globals {
    fn add_instances<'lua, T: crate::tealr::mlu::InstanceCollector<'lua>>(
        self,
        instances: &mut T,
    ) -> crate::tealr::mlu::mlua::Result<()> {
        Ok(())
    }
}
fn bevy_hierarchy_context_initializer(
    _: &bevy_mod_scripting_core::script::ScriptId,
    ctx: &mut crate::prelude::Lua,
) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
    crate::tealr::mlu::set_global_env(Globals, ctx)?;
    Ok(())
}
pub struct BevyHierarchyScriptingPlugin;
impl bevy::app::Plugin for BevyHierarchyScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_lua_proxy::<bevy::hierarchy::prelude::Children>();
        app.register_lua_proxy::<bevy::hierarchy::prelude::Parent>();
        app.register_lua_proxy::<bevy::hierarchy::HierarchyEvent>();
        app.add_context_initializer::<()>(bevy_hierarchy_context_initializer);
        app.add_documentation_fragment(
            crate::docs::LuaDocumentationFragment::new(
                "BevyHierarchyAPI",
                |tw| {
                    tw.document_global_instance::<Globals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaChildren>()
                        .process_type::<LuaParent>()
                        .process_type::<LuaHierarchyEvent>()
                },
            ),
        );
    }
}
