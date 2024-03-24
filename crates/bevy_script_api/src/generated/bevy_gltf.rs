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


    

#[derive(LuaProxy)]
#[proxy(
derive(clone,debug),
remote="GltfExtras",
functions[r#"
#[lua(kind = "Method", as_trait = "std::clone::Clone", output(proxy))]
    fn clone(&self) -> GltfExtras;
}
"#]
)]




pub struct LuaGltfExtras{
    
    
        #[lua(output(proxy))]value:std::string::String,


    
    
}


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

pub struct BevyGltfAPIProvider;

impl bevy_mod_scripting::core::hosts::APIProvider for BevyGltfAPIProvider {
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
        Some(LuaDocFragment::new("BevyGltfAPI", |tw| {
            tw
                .document_global_instance::<Globals>().expect("Something went wrong documenting globals")
            
                .process_type::<LuaGltfExtras>()
                
            
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
        
        app.register_foreign_lua_type::<GltfExtras>();
        
    }
}