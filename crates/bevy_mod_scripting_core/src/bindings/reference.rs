//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.
use super::{access_map::ReflectAccessId, WorldGuard};
use crate::{
    bindings::ReflectAllocationId,
    error::InteropError,
    prelude::ReflectAllocator,
    reflection_extensions::{PartialReflectExt, TypeIdExtensions},
    with_access_read, with_access_write,
};
use bevy::{
    ecs::{
        change_detection::MutUntyped, component::ComponentId, entity::Entity,
        world::unsafe_world_cell::UnsafeWorldCell,
    },
    prelude::{Component, ReflectDefault, Resource},
    ptr::Ptr,
    reflect::{ParsedPath, PartialReflect, Reflect, ReflectFromPtr, ReflectPath},
};
use std::{any::TypeId, fmt::Debug};

/// An accessor to a `dyn PartialReflect` struct, stores a base ID of the type and a reflection path
/// safe to build but to reflect on the value inside you need to ensure aliasing rules are upheld
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
#[reflect(Default)]
pub struct ReflectReference {
    #[reflect(ignore)]
    pub base: ReflectBaseType,
    // TODO: experiment with Fixed capacity vec, boxed array etc, compromise between heap allocation and runtime cost
    // needs benchmarks first though
    /// The path from the top level type to the actual value we want to access
    #[reflect(ignore)]
    pub reflect_path: ParsedPath,
}

impl Default for ReflectReference {
    fn default() -> Self {
        Self {
            base: ReflectBaseType {
                type_id: None::<TypeId>.or_fake_id(),
                base_id: ReflectBase::Owned(ReflectAllocationId::new(0)),
            },
            reflect_path: ParsedPath(vec![]),
        }
    }
}

/// Specifies where we should source the type id from when reflecting a ReflectReference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeIdSource {
    /// Use the type id the reference points to after walking the path
    Tail,
    /// Given the Tail referene is a container type, use the type id of the elements in the container
    Element,
    /// Givent the Tail reference is a container type, use the type id of the keys of the container
    Key,
}

impl ReflectReference {
    /// Creates a new infinite iterator. This iterator will keep returning the next element reference forever.
    pub fn into_iter_infinite(self) -> ReflectRefIter {
        ReflectRefIter::new_indexed(self)
    }

    /// If this is a reference to something with a length accessible via reflection, returns that length.
    pub fn len(&self, world: WorldGuard) -> Result<Option<usize>, InteropError> {
        self.with_reflect(world, |r| match r.reflect_ref() {
            bevy::reflect::ReflectRef::Struct(s) => Some(s.field_len()),
            bevy::reflect::ReflectRef::TupleStruct(ts) => Some(ts.field_len()),
            bevy::reflect::ReflectRef::Tuple(t) => Some(t.field_len()),
            bevy::reflect::ReflectRef::List(l) => Some(l.len()),
            bevy::reflect::ReflectRef::Array(a) => Some(a.len()),
            bevy::reflect::ReflectRef::Map(m) => Some(m.len()),
            bevy::reflect::ReflectRef::Set(s) => Some(s.len()),
            bevy::reflect::ReflectRef::Enum(e) => Some(e.field_len()),
            _ => None,
        })
    }

    pub fn new_allocated<T: PartialReflect>(
        value: T,
        allocator: &mut ReflectAllocator,
    ) -> ReflectReference {
        let type_id = value
            .get_represented_type_info()
            .map(|i| i.type_id())
            .unwrap_or_else(|| {
                panic!(
                    "Type '{}' has no represented type information to allocate with.",
                    std::any::type_name::<T>()
                )
            });
        let id = allocator.allocate(value);
        ReflectReference {
            base: ReflectBaseType {
                type_id,
                base_id: ReflectBase::Owned(id),
            },
            reflect_path: ParsedPath(Vec::default()),
        }
    }

    pub fn new_allocated_boxed(
        value: Box<dyn PartialReflect>,
        allocator: &mut ReflectAllocator,
    ) -> ReflectReference {
        let type_id = value
            .get_represented_type_info()
            .map(|i| i.type_id())
            .unwrap_or_else(|| {
                panic!(
                    "Type '{}' has no represented type information to allocate with.",
                    std::any::type_name_of_val(value.as_ref())
                )
            });
        let id = allocator.allocate_boxed(value);
        ReflectReference {
            base: ReflectBaseType {
                type_id,
                base_id: ReflectBase::Owned(id),
            },
            reflect_path: ParsedPath(Vec::default()),
        }
    }

    pub fn new_resource_ref<T: Resource>(world: WorldGuard) -> Option<Self> {
        let reflect_id = ReflectAccessId::for_resource::<T>(&world.as_unsafe_world_cell())?;
        Some(Self {
            base: ReflectBaseType {
                type_id: TypeId::of::<T>(),
                base_id: ReflectBase::Resource(reflect_id.into()),
            },
            reflect_path: ParsedPath(Vec::default()),
        })
    }

    pub fn new_component_ref<T: Component>(entity: Entity, world: WorldGuard) -> Option<Self> {
        let reflect_id = ReflectAccessId::for_component::<T>(&world.as_unsafe_world_cell())?;
        Some(Self {
            base: ReflectBaseType {
                type_id: TypeId::of::<T>(),
                base_id: ReflectBase::Component(entity, reflect_id.into()),
            },
            reflect_path: ParsedPath(Vec::default()),
        })
    }

    /// Indexes into the reflect path inside this reference.
    /// You can use [`Self::reflect`] and [`Self::reflect_mut`] to get the actual value.
    pub fn index_path<T: Into<ParsedPath>>(&mut self, index: T) {
        self.reflect_path.0.extend(index.into().0);
    }

    /// Tries to downcast to the specified type and cloning the value if successful.
    pub fn downcast<O: Clone + PartialReflect>(
        &self,
        world: WorldGuard,
    ) -> Result<O, InteropError> {
        self.with_reflect(world, |r| {
            r.try_downcast_ref::<O>()
                .cloned()
                .ok_or_else(|| InteropError::could_not_downcast(self.clone(), TypeId::of::<O>()))
        })?
    }

    /// Attempts to create a [`Box<dyn PartialReflect>`] from the reference. This is possible using a few strategies:
    /// - If the reference is to a world, a [`WorldCallbackAccess`] is created and boxed
    /// - If the reference is to an allocation with no reflection path and references to it, the value is taken as is.
    /// - If the reference has a [`bevy::reflect::ReflectFromReflect`] type data associated with it, the value is cloned using that impl.
    /// - If all above fails, [`bevy::reflect::PartialReflect::clone_value`] is used to clone the value.
    ///
    pub fn to_owned_value(
        &self,
        world: WorldGuard,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            if self.reflect_path.is_empty() && id.strong_count() == 0 {
                let allocator = world.allocator();
                let mut allocator = allocator.write();
                let arc = allocator
                    .remove(id)
                    .ok_or_else(|| InteropError::garbage_collected_allocation(self.clone()))?;

                let access_id = ReflectAccessId::for_allocation(id.clone());
                if world.claim_write_access(access_id) {
                    // Safety: we claim write access, nobody else is accessing this
                    if unsafe { &*arc.get_ptr() }.try_as_reflect().is_some() {
                        // Safety: the only accesses exist in this function
                        unsafe { world.release_access(access_id) };
                        return Ok(unsafe { arc.take() });
                    } else {
                        unsafe { world.release_access(access_id) };
                    }
                }
                allocator.insert(id.clone(), arc);
            }
        }

        self.with_reflect(world.clone(), |r| {
            <dyn PartialReflect>::from_reflect_or_clone(r, world.clone())
        })
    }

    /// The way to access the value of the reference, that is the pointed-to value.
    /// This method is safe to use as it ensures no-one else has aliasing access to the value at the same time.
    ///
    /// # Panics
    /// - if the value is aliased and the access is not allowed
    #[track_caller]
    pub fn with_reflect<O, F: FnOnce(&dyn PartialReflect) -> O>(
        &self,
        world: WorldGuard,
        f: F,
    ) -> Result<O, InteropError> {
        let access_id = ReflectAccessId::for_reference(self.base.base_id.clone())
            .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;
        with_access_read!(
            world.0.accesses,
            access_id,
            "could not access reflect reference",
            { unsafe { self.reflect_unsafe(world.clone()) }.map(f) }
        )
    }

    /// The way to access the value of the reference, that is the pointed-to value.
    /// This method is safe to use as it ensures no-one else has aliasing access to the value at the same time.
    ///
    /// # Panics
    /// - if the value is aliased and the access is not allowed
    #[track_caller]
    pub fn with_reflect_mut<O, F: FnOnce(&mut dyn PartialReflect) -> O>(
        &self,
        world: WorldGuard,
        f: F,
    ) -> Result<O, InteropError> {
        let access_id = ReflectAccessId::for_reference(self.base.base_id.clone())
            .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;
        with_access_write!(
            world.0.accesses,
            access_id,
            "Could not access reflect reference mutably",
            { unsafe { self.reflect_mut_unsafe(world.clone()) }.map(f) }
        )
    }

    pub fn tail_type_id(&self, world: WorldGuard) -> Result<Option<TypeId>, InteropError> {
        if self.reflect_path.is_empty() {
            return Ok(Some(self.base.type_id));
        }
        self.with_reflect(world, |r| {
            r.get_represented_type_info().map(|t| t.type_id())
        })
    }

    pub fn element_type_id(&self, world: WorldGuard) -> Result<Option<TypeId>, InteropError> {
        self.with_reflect(world, |r| r.element_type_id())
    }

    pub fn key_type_id(&self, world: WorldGuard) -> Result<Option<TypeId>, InteropError> {
        self.with_reflect(world, |r| r.key_type_id())
    }

    pub fn type_id_of(
        &self,
        source: TypeIdSource,
        world: WorldGuard,
    ) -> Result<Option<TypeId>, InteropError> {
        match source {
            TypeIdSource::Tail => self.tail_type_id(world),
            TypeIdSource::Element => self.element_type_id(world),
            TypeIdSource::Key => self.key_type_id(world),
        }
    }

    /// Retrieves a reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    ///
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing references to the same value exist at all at the same time
    ///
    /// To do this safely you need to use [`WorldAccessGuard::claim_read_access`] or [`WorldAccessGuard::claim_global_access`] to ensure nobody else is currently accessing the value.
    pub unsafe fn reflect_unsafe<'w>(
        &self,
        world: WorldGuard<'w>,
    ) -> Result<&'w dyn PartialReflect, InteropError> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator = world.allocator();
            let allocator = allocator.read();

            let arc = allocator
                .get(id)
                .ok_or_else(|| InteropError::garbage_collected_allocation(self.clone()))?;

            // safety: caller promises it's fine :)
            return self.walk_path(unsafe { &*arc.get_ptr() });
        }

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;

        let ptr = self
            .base
            .base_id
            .clone()
            .into_ptr(world.as_unsafe_world_cell())
            .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;

        // (Ptr) Safety: we use the same type_id to both
        // 1) retrieve the ptr
        // 2) retrieve the ReflectFromPtr type data
        // (UnsafeWorldCell) Safety:
        // we already have access to &world so no &mut world exists
        debug_assert_eq!(
            from_ptr_data.type_id(),
            self.base.type_id,
            "Safety invariant violated"
        );

        let base = unsafe { from_ptr_data.as_reflect(ptr) };

        drop(type_registry);

        self.walk_path(base.as_partial_reflect())
    }

    /// Retrieves mutable reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no other references to the same value exist at all at the same time (even if you have the correct access)
    ///
    /// To do this safely you need to use [`WorldAccessGuard::claim_write_access`] or [`WorldAccessGuard::claim_global_access`] to ensure nobody else is currently accessing the value.
    pub unsafe fn reflect_mut_unsafe<'w>(
        &self,
        world: WorldGuard<'w>,
    ) -> Result<&'w mut dyn PartialReflect, InteropError> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator = world.allocator();
            let allocator = allocator.read();
            let arc = allocator
                .get(id)
                .ok_or_else(|| InteropError::garbage_collected_allocation(self.clone()))?;

            // Safety: caller promises this is fine :)
            return self.walk_path_mut(unsafe { &mut *arc.get_ptr() });
        };

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;

        let ptr = self
            .base
            .base_id
            .clone()
            .into_ptr_mut(world.as_unsafe_world_cell())
            .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;

        // (Ptr) Safety: we use the same type_id to both
        // 1) retrieve the ptr
        // 2) retrieve the ReflectFromPtr type data
        // (UnsafeWorldCell) Safety:
        // we already have access to &world so no &mut world exists
        debug_assert_eq!(
            from_ptr_data.type_id(),
            self.base.type_id,
            "Invariant violated"
        );
        let base = unsafe { from_ptr_data.as_reflect_mut(ptr.into_inner()) };
        drop(type_registry);
        self.walk_path_mut(base.as_partial_reflect_mut())
    }

    fn walk_path<'a>(
        &self,
        root: &'a dyn PartialReflect,
    ) -> Result<&'a dyn PartialReflect, InteropError> {
        self.reflect_path
            .reflect_element(root)
            .map_err(|e| InteropError::reflection_path_error(e.to_string(), Some(self.clone())))
    }

    fn walk_path_mut<'a>(
        &self,
        root: &'a mut dyn PartialReflect,
    ) -> Result<&'a mut dyn PartialReflect, InteropError> {
        self.reflect_path
            .reflect_element_mut(root)
            .map_err(|e| InteropError::reflection_path_error(e.to_string(), Some(self.clone())))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct ReflectBaseType {
    pub type_id: TypeId,
    pub base_id: ReflectBase,
}

/// The Id of the kind of reflection base being pointed to
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum ReflectBase {
    Component(Entity, ComponentId),
    Resource(ComponentId),
    Owned(ReflectAllocationId),
}

impl ReflectBase {
    /// Retrieves the pointer to the underlying `dyn PartialReflect` object valid for the 'w lifteime of the world cell
    ///
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mutable references to the same value exist at the same time
    pub unsafe fn into_ptr(self, world: UnsafeWorldCell<'_>) -> Option<Ptr<'_>> {
        match self {
            ReflectBase::Component(entity, component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_entity(entity)?.get_by_id(component_id)
            }
            ReflectBase::Resource(component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_resource_by_id(component_id)
            }
            _ => None,
        }
    }

    /// Retrieves the pointer to the underlying `dyn PartialReflect` object valid for the 'w lifteime of the world cell
    ///
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing references to the same value exist at all at the same time
    pub unsafe fn into_ptr_mut(self, world: UnsafeWorldCell<'_>) -> Option<MutUntyped<'_>> {
        match self {
            ReflectBase::Component(entity, component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_entity(entity)?.get_mut_by_id(component_id)
            }
            ReflectBase::Resource(component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_resource_mut_by_id(component_id)
            }
            _ => None,
        }
    }
}

pub trait ReflectionPathExt {
    fn convert_to_0_indexed(&mut self);

    fn is_empty(&self) -> bool;

    fn iter(&self) -> impl Iterator<Item = &bevy::reflect::OffsetAccess>;
}

impl ReflectionPathExt for ParsedPath {
    /// Assumes the accesses are 1 indexed and converts them to 0 indexed
    fn convert_to_0_indexed(&mut self) {
        self.0.iter_mut().for_each(|a| match a.access {
            bevy::reflect::Access::FieldIndex(ref mut i) => *i -= 1,
            bevy::reflect::Access::TupleIndex(ref mut i) => *i -= 1,
            bevy::reflect::Access::ListIndex(ref mut i) => *i -= 1,
            _ => {}
        });
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = &bevy::reflect::OffsetAccess> {
        self.0.iter()
    }
}

/// A generic iterator over any reflected value.
/// Unlike a normal iterator, this one does not have a halting condition, it will keep returning elements forever.
/// The iterator does not try to access the value, it just works out the next index/key to access.
/// You will know you've reached the end when you get an error when trying to access the next element.
#[derive(Debug, Clone)]
pub struct ReflectRefIter {
    pub(crate) base: ReflectReference,
    // TODO: support maps etc
    pub(crate) index: IterationKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IterationKey {
    Index(usize),
}

impl ReflectRefIter {
    pub fn new_indexed(base: ReflectReference) -> Self {
        Self {
            base,
            index: IterationKey::Index(0),
        }
    }

    pub fn index(&self) -> IterationKey {
        self.index.clone()
    }

    /// Returns the next element in the iterator, it does not have a halting condition
    pub fn next_ref(&mut self) -> (ReflectReference, IterationKey) {
        let index = self.index();
        let next = match &mut self.index {
            IterationKey::Index(i) => {
                let mut next = self.base.clone();
                let parsed_path =
                    ParsedPath::parse(&format!("[{}]", *i)).expect("invariant violated");
                next.index_path(parsed_path);
                *i += 1;
                next
            }
        };
        (next, index)
    }
}

impl Iterator for ReflectRefIter {
    type Item = Result<ReflectReference, InteropError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Result<_, _> = {
            match &mut self.index {
                IterationKey::Index(i) => {
                    let mut next = self.base.clone();
                    let parsed_path = ParsedPath::parse(&i.to_string()).unwrap();
                    next.index_path(parsed_path);
                    *i += 1;
                    Ok(next)
                }
            }
        };

        Some(result)
    }
}