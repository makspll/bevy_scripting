use std::collections::HashMap;

use cargo_metadata::camino::Utf8PathBuf;
use indexmap::IndexMap;
use log::debug;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{AdtDef, TyCtxt};
use serde::Serialize;

use crate::{MetaLoader, TemplateContext};

pub(crate) struct BevyCtxt<'tcx> {
    pub(crate) tcx: TyCtxt<'tcx>,
    pub(crate) meta_loader: MetaLoader,
    pub(crate) reflect_types: IndexMap<DefId, ReflectType<'tcx>>,
    pub(crate) cached_traits: CachedTraits,

    /// the template context used for generating code
    pub(crate) template_context: Option<TemplateContext>,
}

impl<'tcx> BevyCtxt<'tcx> {
    /// Creates a new context with the provided TyCtxt and meta directories
    pub(crate) fn new(tcx: TyCtxt<'tcx>, meta_dirs: Vec<Utf8PathBuf>) -> Self {
        Self {
            tcx,
            reflect_types: Default::default(),
            cached_traits: Default::default(),
            meta_loader: MetaLoader::new(meta_dirs),
            template_context: Default::default(),
        }
    }

    /// Clears all data structures in the context
    pub(crate) fn clear(&mut self) {
        debug!("Clearing all context");
        *self = Self::new(self.tcx, self.meta_loader.meta_dirs.clone());
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct ReflectType<'tcx> {
    /// Trait implementations for the reflect type (from a selection)
    pub(crate) trait_impls: Option<Vec<DefId>>,
    /// Information about the ADT structure, fields, and variants
    pub(crate) variant_data: Option<AdtDef<'tcx>>,
    /// Functions passing criteria to be proxied
    pub(crate) valid_functions: Option<Vec<FunctionContext>>,

    /// Mapping from fields to the reflection strategy
    field_reflection_types: IndexMap<DefId, ReflectionStrategy>,
}

impl ReflectType<'_> {
    pub(crate) fn set_field_reflection_strategies<
        I: Iterator<Item = (DefId, ReflectionStrategy)>,
    >(
        &mut self,
        field_strats: I,
    ) {
        self.field_reflection_types = field_strats.collect();
    }

    pub(crate) fn get_field_reflection_strat(&self, field: DefId) -> Option<&ReflectionStrategy> {
        self.field_reflection_types.get(&field)
    }
}

pub(crate) const DEF_PATHS_FROM_LUA: [&str; 2] = ["value::FromLuaMulti", "mlua::FromLuaMulti"];
pub(crate) const DEF_PATHS_TO_LUA: [&str; 2] = ["value::ToLuaMulti", "mlua::ToLuaMulti"];
pub(crate) const DEF_PATHS_REFLECT: [&str; 2] = ["bevy_reflect::Reflect", "reflect::Reflect"];
pub(crate) const FN_SOURCE_TRAITS: [&str; 12] = [
    // PRINTING
    "std::string::ToString",
    // OWNERSHIP
    "std::clone::Clone",
    // OPERATORS
    "std::ops::Neg",
    "std::ops::Mul",
    "std::ops::Add",
    "std::ops::Sub",
    "std::ops::Div",
    "std::ops::Rem",
    "std::cmp::Eq",
    "std::cmp::PartialEq",
    "std::ord::Ord", // we don't use these fully cuz of the output types not being lua primitives, but keeping it for the future
    "std::ord::PartialOrd",
];

/// A collection of common traits stored for quick access.
#[derive(Default)]
pub(crate) struct CachedTraits {
    pub(crate) mlua_from_lua_multi: Option<DefId>,
    pub(crate) mlua_to_lua_multi: Option<DefId>,
    pub(crate) bevy_reflect_reflect: Option<DefId>,
    /// Traits whose methods can be included in the generated code
    pub(crate) fn_source_traits: HashMap<String, DefId>,
}

impl CachedTraits {
    pub(crate) fn has_all_mlua_traits(&self) -> bool {
        self.mlua_from_lua_multi.is_some() && self.mlua_to_lua_multi.is_some()
    }

    pub(crate) fn has_all_bevy_traits(&self) -> bool {
        self.bevy_reflect_reflect.is_some()
    }

    pub(crate) fn has_all_fn_source_traits(&self) -> bool {
        self.fn_source_traits
            .iter()
            .all(|(k, _)| FN_SOURCE_TRAITS.contains(&k.as_str()))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FunctionContext {
    pub(crate) def_id: DefId,
    pub(crate) has_self: bool,
    pub(crate) trait_did: Option<DefId>,
    /// strategies for input and output (last element is the output)
    pub(crate) reflection_strategies: Vec<ReflectionStrategy>,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Debug)]
pub(crate) enum ReflectionStrategy {
    /// The will have a known wrapper we can use
    Proxy,
    /// The type is a primitive with the right traits to be used directly in arguments and return values
    Primitive,
    /// Use a reflection primitive i.e. 'ReflectedValue', dynamic runtime reflection
    Reflection,
    /// Either ignored via 'reflect(ignore)' or not visible
    Filtered,
}