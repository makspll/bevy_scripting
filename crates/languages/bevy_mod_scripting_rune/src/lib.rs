use std::sync::Arc;

use bevy_mod_scripting_core::{asset::Language, IntoScriptPluginParams, ScriptingPlugin};
use rune::{
    runtime::{Args, RuntimeContext},
    Unit, Vm,
};

pub trait RuneEventArg: Args + Clone + Send + Sync + 'static {}
impl<T: Args + Clone + Send + Sync + 'static> RuneEventArg for T {}

pub struct RuneScriptContext {
    pub unit: Arc<Unit>,
    pub runtime_context: Arc<RuntimeContext>,
}

pub type RuneRuntime = Vm;

impl IntoScriptPluginParams for RuneScriptingPlugin {
    type C = RuneScriptContext;
    type R = RuneRuntime;

    const LANGUAGE: Language = Language::Rune;

    fn build_runtime() -> Self::R {
        todo!()
    }
}

pub struct RuneScriptingPlugin {
    pub scripting_plugin: ScriptingPlugin<Self>,
}

impl Default for RuneScriptingPlugin {
    fn default() -> Self {
        todo!()
        // Self {
        //     scripting_plugin: ScriptingPlugin {
        //         runtime_builder: todo!(),
        //         runtime_settings: todo!(),
        //         callback_handler: todo!(),
        //         context_builder: todo!(),
        //         context_assigner: todo!(),
        //     },
        // }
    }
}

// use std::{marker::PhantomData, sync::Arc};

// use bevy::prelude::*;
// use bevy_mod_scripting_core::{
//     prelude::*,
//     systems::{self, CachedScriptState},
// };
// use prelude::{RuneDocFragment, RuneFile, RuneLoader};
// use rune::{
//     runtime::{Args, RuntimeContext, VmError, VmResult},
//     Context, Diagnostics, Source, Sources, Unit, Vm,
// };

// mod assets;
// mod docs;

// pub mod prelude {
//     pub use crate::{
//         assets::{RuneFile, RuneLoader},
//         docs::RuneDocFragment,
//         RuneArgs, RuneEvent, RuneScriptContext, RuneScriptHost,
//     };
//     pub use rune::{self, runtime::Args, Context};
// }

// /// Super trait adding additional bounds to Rune's `Args` trait.
// /// It's gets automatically implemented for any type that implments `Args`,
// /// so you should never have to manually implement it.
// pub trait RuneArgs: Args + Clone + Send + Sync + 'static {}

// impl<T: Args + Clone + Send + Sync + 'static> RuneArgs for T {}

// /// A Rune script hook.
// #[derive(Debug, Clone, Event)]
// pub struct RuneEvent<A: RuneArgs> {
//     /// The name of the Rune function to call.
//     pub hook_name: String,
//     /// The arguments to supply the function being invoked. If you
//     /// don't need any arguments, `()` is a good default value.
//     pub args: A,
//     /// The target set of scripts that should handle this event.
//     pub recipients: Recipients,
// }

// impl<A: RuneArgs> ScriptEvent for RuneEvent<A> {
//     fn recipients(&self) -> &Recipients {
//         &self.recipients
//     }
// }

// /// A cached Rune Vm used to execute units.
// struct RuneVm(Vm);

// impl Default for RuneVm {
//     fn default() -> Self {
//         Self(Vm::new(
//             Arc::new(RuntimeContext::default()),
//             Arc::new(Unit::default()),
//         ))
//     }
// }

// /// Script context for a rune script.
// pub struct RuneScriptContext {
//     pub unit: Arc<Unit>,
//     pub runtime_context: Arc<RuntimeContext>,
// }

// #[derive(Resource)]
// /// Rune script host. Enables Rune scripting.
// pub struct RuneScriptHost<A: RuneArgs> {
//     _ph: PhantomData<A>,
// }

// impl<A: RuneArgs> Default for RuneScriptHost<A> {
//     fn default() -> Self {
//         Self {
//             _ph: Default::default(),
//         }
//     }
// }

// impl<A: RuneArgs> RuneScriptHost<A> {
//     /// Helper function to handle errors from a Rune virtual machine.
//     ///
//     #[cold]
//     fn handle_rune_error(world: &mut World, error: VmError, script_data: &ScriptData<'_>) {
//         let mut state: CachedScriptState<Self> = world.remove_resource().unwrap();

//         let (_, mut error_wrt, _) = state.event_state.get_mut(world);

//         let error = ScriptError::RuntimeError {
//             script: script_data.name.to_owned(),
//             msg: error.to_string(),
//         };

//         error!("{}", error);

//         error_wrt.send(ScriptErrorEvent { error });
//         world.insert_resource(state);
//     }
// }

// impl<A: RuneArgs> ScriptHost for RuneScriptHost<A> {
//     type ScriptContext = RuneScriptContext;

//     type ScriptEvent = RuneEvent<A>;

//     type ScriptAsset = RuneFile;

//     type APITarget = Context;

//     type DocTarget = RuneDocFragment;

//     fn register_with_app_in_set(
//         app: &mut App,
//         schedule: impl bevy::ecs::schedule::ScheduleLabel,
//         set: impl SystemSet,
//     ) {
//         app.add_priority_event::<Self::ScriptEvent>()
//             .init_asset::<RuneFile>()
//             .init_asset_loader::<RuneLoader>()
//             .init_resource::<CachedScriptState<Self>>()
//             .init_resource::<ScriptContexts<Self::ScriptContext>>()
//             .init_resource::<APIProviders<Self>>()
//             .register_type::<ScriptCollection<Self::ScriptAsset>>()
//             .register_type::<Script<Self::ScriptAsset>>()
//             .register_type::<Handle<RuneFile>>()
//             // Add a cached Vm as a non-send resource.
//             .insert_non_send_resource(RuneVm::default())
//             // handle script insertions removal first
//             // then update their contexts later on script asset changes
//             .add_systems(
//                 schedule,
//                 (
//                     systems::script_add_synchronizer::<Self>,
//                     systems::script_remove_synchronizer::<Self>,
//                     systems::script_hot_reload_handler::<Self>,
//                 )
//                     .chain()
//                     .in_set(set),
//             );
//     }

//     fn load_script(
//         &mut self,
//         script: &[u8],
//         script_data: &ScriptData,
//         providers: &mut APIProviders<Self>,
//     ) -> Result<Self::ScriptContext, ScriptError> {
//         let mut context = rune_modules::default_context().map_err(ScriptError::new_other)?;

//         // Rune requires that we tell it what modules and types we'll be using before
//         // it compiles a file.
//         providers.attach_all(&mut context).unwrap();

//         let mut diagnostics = Diagnostics::new();

//         let mut sources = Sources::new();
//         sources
//             .insert(
//                 Source::new(
//                     script_data.name,
//                     std::str::from_utf8(script).expect("Slice is not UTF-8"),
//                 )
//                 .map_err(|msg| ScriptError::FailedToLoad {
//                     script: script_data.name.into(),
//                     msg: msg.to_string(),
//                 })?,
//             )
//             .map_err(|msg| ScriptError::FailedToLoad {
//                 script: script_data.name.into(),
//                 msg: msg.to_string(),
//             })?;

//         let result = rune::prepare(&mut sources)
//             .with_context(&context)
//             .with_diagnostics(&mut diagnostics)
//             .build();

//         if !diagnostics.is_empty() {
//             let mut writer = rune::termcolor::Buffer::no_color();

//             diagnostics
//                 .emit(&mut writer, &sources)
//                 .expect("Failed to write diagnostics to buffer");

//             return Err(ScriptError::SyntaxError {
//                 script: script_data.name.into(),
//                 msg: std::str::from_utf8(writer.as_slice())
//                     .expect("Slice was not UTF-8")
//                     .to_owned(),
//             });
//         }

//         let unit = result.expect("Failed to build Rune unit.");

//         let runtime_ctx = context
//             .runtime()
//             .expect("Failed to create Rune runtime context.");

//         Ok(RuneScriptContext {
//             unit: Arc::new(unit),
//             runtime_context: Arc::new(runtime_ctx),
//         })
//     }

//     fn setup_script(
//         &mut self,
//         script_data: &ScriptData,
//         ctx: &mut Self::ScriptContext,
//         providers: &mut APIProviders<Self>,
//     ) -> Result<(), ScriptError> {
//         providers.setup_all(script_data, ctx)
//     }

//     fn handle_events<'a>(
//         &mut self,
//         world: &mut World,
//         events: &[Self::ScriptEvent],
//         ctxs: impl Iterator<Item = (ScriptData<'a>, &'a mut Self::ScriptContext)>,
//         _providers: &mut APIProviders<Self>,
//     ) {
//         // Grab the cached Vm.
//         let RuneVm(mut vm) = world.remove_non_send_resource::<RuneVm>().unwrap(/* invariant */);

//         {
//             ctxs.for_each(|(script_data, ctx)| {
//                 for event in events {
//                     if !event.recipients().is_recipient(&script_data) {
//                         continue;
//                     }

//                     // Swap out the old context and old unit with the new ones.
//                     *vm.context_mut() = Arc::clone(&ctx.runtime_context);
//                     *vm.unit_mut() = Arc::clone(&ctx.unit);

//                     let mut exec = match vm.execute([event.hook_name.as_str()], event.args.clone())
//                     {
//                         Ok(exec) => exec,
//                         Err(error) => {
//                             Self::handle_rune_error(world, error, &script_data);
//                             continue;
//                         }
//                     };

//                     if let VmResult::Err(error) = exec.complete() {
//                         Self::handle_rune_error(world, error, &script_data);
//                     }
//                 }
//             });
//         }

//         world.insert_non_send_resource(RuneVm(vm));
//     }
// }
