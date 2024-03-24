// @generated by cargo bevy-api-gen generate, modify the templates not this file

#![allow(clippy::all, unused_imports, deprecated, dead_code)]
// This file is generatedchange the template not this file
extern crate self as bevy_script_api;

use std::sync::Mutex;
use bevy::prelude::App;
use bevy_mod_scripting_core::prelude::*;

#[cfg(feature = "lua")]
use {
    crate::{lua::RegisterForeignLuaType, ReflectedValue},
    bevy_mod_scripting_lua::docs::LuaDocFragment,
    bevy_mod_scripting_lua_derive::LuaProxy,
};


    
/// Contains references to the child entities of this entity.

/// See [`HierarchyQueryExt`] for hierarchy related methods on [`Query`].

/// [`HierarchyQueryExt`]: crate::query_extension::HierarchyQueryExt

/// [`Query`]: bevy_ecs::system::Query


#[derive(LuaProxy)]
#[proxy(
derive(clone,debug),
remote="components::children::Children",
functions[r#"
/// Swaps the child at `a_index` with the child at `b_index`.
#[lua(kind = "Method")]
    fn swap(&mut self, a_index: usize, b_index: usize) -> ();
}
"#]
)]



pub struct LuaChildren(
    
    
        smallvec::SmallVec<[bevy_ecs::entity::Entity; 8]>,


    
    
);

    
/// Holds a reference to the parent entity of this entity.

/// This component should only be present on entities that actually have a parent entity.

/// See [`HierarchyQueryExt`] for hierarchy related methods on [`Query`].

/// [`HierarchyQueryExt`]: crate::query_extension::HierarchyQueryExt

/// [`Query`]: bevy_ecs::system::Query


#[derive(LuaProxy)]
#[proxy(
derive(clone,debug),
remote="components::parent::Parent",
functions[r#"
/// Gets the [`Entity`] ID of the parent.
#[lua(kind = "Method", output(proxy))]
    fn get(&self) -> bevy_ecs::entity::Entity;
}
"#,
			r#"
#[lua(
        kind = "Method",
        as_trait = "std::cmp::PartialEq",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &components::parent::Parent) -> bool;
}
"#,
			r#"
#[lua(kind = "Method", as_trait = "std::cmp::Eq")]
    fn assert_receiver_is_total_eq(&self) -> ();
}
"#]
)]



pub struct LuaParent(
    
    
        #[lua(output(proxy))]bevy_ecs::entity::Entity,


    
    
);


bevy_script_api::util::impl_tealr_generic!(pub(crate) struct T);

#[derive(Default)]
pub(crate) struct Globals;

impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for Globals {
    fn add_instances<'lua, T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>>(
        self,
        instances: &mut T,
    ) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
         
            
         
            
        
        Ok(())
    }
}

pub struct BevyHierarchyAPIProvider;

impl bevy_mod_scripting::core::hosts::APIProvider for BevyHierarchyAPIProvider {
        type APITarget = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
        type ScriptContext = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
        type DocTarget = LuaDocFragment;

    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(), ScriptError> {
        let ctx = ctx
            .get_mut()
            .expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(Globals, ctx)
            .map_err(|e| ScriptError::Other(e.to_string()))
    }

    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(LuaDocFragment::new("BevyHierarchyAPI", |tw| {
            tw
                .document_global_instance::<Globals>().expect("Something went wrong documenting globals")
            
                .process_type::<LuaChildren>()
                
            
                .process_type::<LuaParent>()
                
            
            }
        ))
    }

    fn setup_script(
        &mut self,
        script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        Ok(())
    }

    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        Ok(())
    }

    fn register_with_app(&self, app: &mut App) {
        
        app.register_foreign_lua_type::<components::children::Children>();
        
        app.register_foreign_lua_type::<components::parent::Parent>();
        
    }
}