use std::{error::Error, fmt::Display};

use clap::Arg;
use indexmap::IndexMap;
use rustdoc_types::{Crate, Item, ItemEnum, Type};
use sailfish::TemplateOnce;

use crate::{ArgType, Config, ItemData, TypeMeta};

/// Struct representing an argument to a function, or a field declaration, Anything with a name and a type reall
#[derive(Debug)]
pub struct NameType {
    pub name: String,
    pub type_: ArgType,
    pub is_proxied: bool,
}

impl NameType {
    pub fn try_new(
        name: String,
        type_: Type,
        config: &Config,
        assoc_types: &[&Item],
    ) -> Result<Self, Box<dyn Error>> {
        let mut type_: ArgType = ArgType::try_new(name == "self", &type_)?;
        if type_.is_associated_type() {
            log::trace!(
                "Type `{type_:?}` contains associated type, matching up with `{assoc_types:?}`"
            );
            type_ = type_.map_associated_types(&|on_type, name| {
                if on_type.is_contextual() {
                    assoc_types
                        .iter()
                        .find(|assoc| {
                            assoc
                                .name
                                .as_ref()
                                .is_some_and(|assoc_name| assoc_name == &name)
                        })
                        .and_then(|assoc| {
                            if let ItemEnum::AssocType { default, .. } = &assoc.inner {
                                let a = ArgType::try_new(false, default.as_ref()?);
                                Some(a.ok()?)
                            } else {
                                log::info!(
                                    "Found matching assoc type but of wrong type {:?}",
                                    assoc
                                );
                                None
                            }
                        })
                } else {
                    None
                }
            });
        }

        let is_primitive = type_
            .base_ident()
            .is_some_and(|ident| config.primitives.contains(ident));

        let is_proxied = type_.is_contextual()
            || (!is_primitive
                && type_
                    .base_ident()
                    .is_some_and(|ident| config.types.contains_key(ident)));

        if !is_primitive && !is_proxied {
            return Err(format!(
                "Type is neither a wrapped type in the config or an allowed primitive: `{type_:?}`"
            )
            .into());
        }

        match &type_ {
            ArgType::Ref { ref_, .. } if ref_.has_outer_ref() => {
                return Err(format!("Type is a double reference: `{type_:?}`").into())
            }
            _ => (),
        }

        Ok(Self {
            name,
            type_,
            is_proxied,
        })
    }
}

#[derive(Clone)]
pub struct ImportPath {
    pub components: Vec<String>,
}

impl From<Vec<String>> for ImportPath {
    fn from(value: Vec<String>) -> Self {
        Self { components: value }
    }
}

impl Display for ImportPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.components.join("::").as_str())
    }
}

impl ImportPath {
    pub fn replace_prefix(&self, prefix: &str, replacement: &str) -> Self {
        let mut components = self.components.clone();
        if let Some(first) = components.first_mut() {
            if let Some(stripped) = first.strip_prefix(prefix) {
                *first = replacement.to_owned() + stripped;
            }
        }
        Self { components }
    }
}

#[derive(TemplateOnce)]
#[template(path = "main.stpl", escape = false)]
pub struct TemplateData {
    pub items: IndexMap<String, ItemData>,
}
