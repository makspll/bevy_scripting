use crate::{
    asset::ScriptAsset,
    context::{ContextLoadingSettings, ScriptContexts},
    event::{IntoCallbackLabel, OnScriptLoaded, OnScriptUnloaded},
    handler::CallbackSettings,
    prelude::RuntimeContainer,
    script::{Script, ScriptId, Scripts},
    systems::handle_script_errors,
    IntoScriptPluginParams,
};
use bevy::{asset::Handle, ecs::world::Mut, log::debug, prelude::Command};
use std::{any::type_name, marker::PhantomData};

pub struct DeleteScript<P: IntoScriptPluginParams> {
    pub id: ScriptId,
    // hack to make this Send, C does not need to be Send since it is not stored in the command
    pub _ph: PhantomData<fn(P::C, P::R)>,
}

impl<P: IntoScriptPluginParams> DeleteScript<P> {
    pub fn new(id: ScriptId) -> Self {
        Self {
            id,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Command for DeleteScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        let settings = world
            .get_resource::<ContextLoadingSettings<P>>()
            .expect("No ScriptLoadingSettings resource found")
            .clone();

        let runner = world
            .get_resource::<CallbackSettings<P>>()
            .expect("No CallbackSettings resource found")
            .callback_handler
            .expect("No callback handler set");

        let mut ctxts = world
            .remove_non_send_resource::<ScriptContexts<P>>()
            .unwrap();

        let mut runtime_container = world
            .remove_non_send_resource::<RuntimeContainer<P>>()
            .unwrap();

        world.resource_scope(|world, mut scripts: Mut<Scripts>| {
            if let Some(script) = scripts.scripts.remove(&self.id) {
                debug!("Deleting script with id: {}", self.id);

                // first let the script uninstall itself
                match (runner)(
                    vec![],
                    bevy::ecs::entity::Entity::from_raw(0),
                    &self.id,
                    &OnScriptUnloaded::into_callback_label(),
                    ctxts.get_mut(script.context_id).unwrap(),
                    &settings.context_pre_handling_initializers,
                    &mut runtime_container.runtime,
                    world,
                ) {
                    Ok(_) => {},
                    Err(e) => {
                        handle_script_errors(world, [e.with_context(format!("Running unload hook for script with id: {}. Runtime type: {}, Context type: {}", self.id, type_name::<P::R>(), type_name::<P::C>()))].into_iter());
                    }
                }

                let assigner = settings
                    .assigner
                    .as_ref()
                    .expect("Could not find context assigner in settings");
                debug!("Removing script with id: {}", self.id);
                (assigner.remove)(script.context_id, &script, &mut ctxts)
            } else {
                bevy::log::error!(
                    "Attempted to delete script with id: {} but it does not exist, doing nothing!",
                    self.id
                );
            }
        });

        world.insert_resource(settings);
        world.insert_non_send_resource(ctxts);
        world.insert_non_send_resource(runtime_container);
    }
}

/// Creates new script with the given ID, if a script with the given ID already exists, this is treated as an update
///
/// If script comes from an asset, expects it to be loaded, otherwise this command will fail to process the script.
pub struct CreateOrUpdateScript<P: IntoScriptPluginParams> {
    id: ScriptId,
    content: Box<[u8]>,
    asset: Option<Handle<ScriptAsset>>,
    // Hack to make this Send, C does not need to be Send since it is not stored in the command
    _ph: std::marker::PhantomData<fn(P::R, P::C)>,
}

impl<P: IntoScriptPluginParams> CreateOrUpdateScript<P> {
    pub fn new(id: ScriptId, content: Box<[u8]>, asset: Option<Handle<ScriptAsset>>) -> Self {
        Self {
            id,
            content,
            asset,
            _ph: std::marker::PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Command for CreateOrUpdateScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        debug!(
            "CreateOrUpdateScript command applying to script_id: {}",
            self.id
        );
        let settings = world
            .get_resource::<ContextLoadingSettings<P>>()
            .unwrap()
            .clone();
        let mut contexts = world
            .remove_non_send_resource::<ScriptContexts<P>>()
            .unwrap();
        let mut runtime = world
            .remove_non_send_resource::<RuntimeContainer<P>>()
            .unwrap();

        let runner = world.get_resource::<CallbackSettings<P>>().unwrap();
        // assign context
        let assigner = settings.assigner.clone().expect("No context assigner set");
        let builder = settings.loader.clone().expect("No context loader set");
        let runner = runner.callback_handler.expect("No callback handler set");

        world.resource_scope(|world, mut scripts: Mut<Scripts>| {

            // check if script already exists

            let mut script = scripts.scripts.get_mut(&self.id);
            let previous_context_id = script.as_ref().map(|s| s.context_id);
            debug!(
                "CreateOrUpdateScript command applying with to (script_id: {}, previous_context_id: {:?})",
                self.id, previous_context_id
            );

            // If None assign new context ID, otherwise assign the old one
            // If re-loading and different from the previous one, the old one will be removed
            let current_context_id = (assigner.assign)(script.as_deref(), &self.id, &self.content, &mut contexts);
            debug!("Context assigned: {:?}", current_context_id);

            let current_context_id = if let Some(id) = current_context_id {
                // reload existing context
                let current_context = contexts.get_mut(id).unwrap();
                match (builder.reload)(&self.id, &self.content, current_context, &settings.context_initializers, &settings.context_pre_handling_initializers, world, &mut runtime.runtime) {
                    Ok(_) => {},
                    Err(e) => {
                        handle_script_errors(world, [e.with_context(format!("Reloading script with id: {}. Runtime type: {}, Context type: {}", self.id, type_name::<P::R>(), type_name::<P::C>()))].into_iter());
                        return;
                    }
                };
                id
            } else {
                let ctxt = (builder.load)(&self.id, &self.content, &settings.context_initializers, &settings.context_pre_handling_initializers, world, &mut runtime.runtime);
                match ctxt {
                    Ok(ctxt) => contexts.insert(ctxt),
                    Err(e) => {
                        handle_script_errors(world, [e.with_context(format!("Loading script with id: {}. Runtime type: {}, Context type: {}", self.id, type_name::<P::R>(), type_name::<P::C>()))].into_iter());
                        return;
                    }
                }
            };


            if let Some(previous) = previous_context_id {
                if previous != current_context_id {
                    debug!(
                        "Script is being moved to a new context with id: {}, removing up old context.",
                        current_context_id
                    );
                    script.as_deref_mut().unwrap().context_id = current_context_id;
                    (assigner.remove)(previous, script.unwrap(), &mut contexts);
                }
            }

            let context = contexts.get_mut(current_context_id).expect("Context not found");
            match (runner)(vec![], bevy::ecs::entity::Entity::from_raw(0), &self.id, &OnScriptLoaded::into_callback_label(), context, &settings.context_pre_handling_initializers, &mut runtime.runtime, world) {
                Ok(_) => {},
                Err(e) => {
                    handle_script_errors(world, [e.with_context(format!("Running initialization hook for script with id: {}. Runtime type: {}, Context type: {}", self.id, type_name::<P::R>(), type_name::<P::C>()))].into_iter());
                },
            }

            // now we can insert the actual script
            scripts.scripts.insert(
                self.id.clone(),
                Script {
                    id: self.id,
                    asset: self.asset,
                    context_id: current_context_id,
                },
            );

            // finally we trigger on_script_loaded
        });
        world.insert_resource(settings);
        world.insert_non_send_resource(runtime);
        world.insert_non_send_resource(contexts);
    }
}