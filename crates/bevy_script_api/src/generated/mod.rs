// @generated by cargo bevy-api-gen collect, modify the templates not this file
mod bevy_animation;
mod bevy_asset;
mod bevy_core;
mod bevy_core_pipeline;
mod bevy_ecs;
mod bevy_gizmos;
mod bevy_gltf;
mod bevy_hierarchy;
mod bevy_input;
mod bevy_pbr;
mod bevy_reflect;
mod bevy_render;
mod bevy_sprite;
mod bevy_text;
mod bevy_time;
mod bevy_transform;
mod bevy_ui;
mod bevy_window;
pub struct BevyAPIProvider;
impl bevy_mod_scripting::core::hosts::APIProvider for BevyAPIProvider {
    type APITarget = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = LuaDocFragment;
    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(), ScriptError> {
        bevy_ui::BevyUiAPIProvider.attach_api(ctx)?;
        bevy_sprite::BevySpriteAPIProvider.attach_api(ctx)?;
        bevy_core_pipeline::BevyCorePipelineAPIProvider.attach_api(ctx)?;
        bevy_ecs::BevyEcsAPIProvider.attach_api(ctx)?;
        bevy_transform::BevyTransformAPIProvider.attach_api(ctx)?;
        bevy_text::BevyTextAPIProvider.attach_api(ctx)?;
        bevy_input::BevyInputAPIProvider.attach_api(ctx)?;
        bevy_asset::BevyAssetAPIProvider.attach_api(ctx)?;
        bevy_core::BevyCoreAPIProvider.attach_api(ctx)?;
        bevy_gizmos::BevyGizmosAPIProvider.attach_api(ctx)?;
        bevy_time::BevyTimeAPIProvider.attach_api(ctx)?;
        bevy_pbr::BevyPbrAPIProvider.attach_api(ctx)?;
        bevy_hierarchy::BevyHierarchyAPIProvider.attach_api(ctx)?;
        bevy_gltf::BevyGltfAPIProvider.attach_api(ctx)?;
        bevy_animation::BevyAnimationAPIProvider.attach_api(ctx)?;
        bevy_window::BevyWindowAPIProvider.attach_api(ctx)?;
        bevy_render::BevyRenderAPIProvider.attach_api(ctx)?;
        bevy_reflect::BevyReflectAPIProvider.attach_api(ctx)?;
        Ok(())
    }
    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        [
            bevy_ui::BevyUiAPIProvider.get_doc_fragment(),
            bevy_sprite::BevySpriteAPIProvider.get_doc_fragment(),
            bevy_core_pipeline::BevyCorePipelineAPIProvider.get_doc_fragment(),
            bevy_ecs::BevyEcsAPIProvider.get_doc_fragment(),
            bevy_transform::BevyTransformAPIProvider.get_doc_fragment(),
            bevy_text::BevyTextAPIProvider.get_doc_fragment(),
            bevy_input::BevyInputAPIProvider.get_doc_fragment(),
            bevy_asset::BevyAssetAPIProvider.get_doc_fragment(),
            bevy_core::BevyCoreAPIProvider.get_doc_fragment(),
            bevy_gizmos::BevyGizmosAPIProvider.get_doc_fragment(),
            bevy_time::BevyTimeAPIProvider.get_doc_fragment(),
            bevy_pbr::BevyPbrAPIProvider.get_doc_fragment(),
            bevy_hierarchy::BevyHierarchyAPIProvider.get_doc_fragment(),
            bevy_gltf::BevyGltfAPIProvider.get_doc_fragment(),
            bevy_animation::BevyAnimationAPIProvider.get_doc_fragment(),
            bevy_window::BevyWindowAPIProvider.get_doc_fragment(),
            bevy_render::BevyRenderAPIProvider.get_doc_fragment(),
            bevy_reflect::BevyReflectAPIProvider.get_doc_fragment(),
        ]
        .into_iter()
        .filter_map(|a: Option<_>| a)
        .fold(None, |(a, b)| a.merge(b))
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
        bevy_ui::BevyUiAPIProvider.register_with_app(app);
        bevy_sprite::BevySpriteAPIProvider.register_with_app(app);
        bevy_core_pipeline::BevyCorePipelineAPIProvider.register_with_app(app);
        bevy_ecs::BevyEcsAPIProvider.register_with_app(app);
        bevy_transform::BevyTransformAPIProvider.register_with_app(app);
        bevy_text::BevyTextAPIProvider.register_with_app(app);
        bevy_input::BevyInputAPIProvider.register_with_app(app);
        bevy_asset::BevyAssetAPIProvider.register_with_app(app);
        bevy_core::BevyCoreAPIProvider.register_with_app(app);
        bevy_gizmos::BevyGizmosAPIProvider.register_with_app(app);
        bevy_time::BevyTimeAPIProvider.register_with_app(app);
        bevy_pbr::BevyPbrAPIProvider.register_with_app(app);
        bevy_hierarchy::BevyHierarchyAPIProvider.register_with_app(app);
        bevy_gltf::BevyGltfAPIProvider.register_with_app(app);
        bevy_animation::BevyAnimationAPIProvider.register_with_app(app);
        bevy_window::BevyWindowAPIProvider.register_with_app(app);
        bevy_render::BevyRenderAPIProvider.register_with_app(app);
        bevy_reflect::BevyReflectAPIProvider.register_with_app(app);
    }
}