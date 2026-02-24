use std::any::{Any, TypeId};
use std::collections::HashMap;

/// A simple type-mapped resource manager for storing global state.
pub struct Resources {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Insert a resource of type `T`. Overwrites any existing resource of the same type.
    pub fn insert<T: Any>(&mut self, resource: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(resource));
    }

    /// Remove a resource of type `T`. Returns the resource if it existed.
    pub fn remove<T: Any>(&mut self) -> Option<T> {
        self.map
            .remove(&TypeId::of::<T>())
            .map(|boxed| *boxed.downcast::<T>().unwrap())
    }

    /// Get an immutable reference to a resource of type `T`.
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .map(|boxed| boxed.downcast_ref::<T>().unwrap())
    }

    /// Get a mutable reference to a resource of type `T`.
    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .map(|boxed| boxed.downcast_mut::<T>().unwrap())
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}
