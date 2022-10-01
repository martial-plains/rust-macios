use std::{collections::HashMap, ffi::CString, sync::RwLock};

use objc::{
    declare::ClassDecl,
    runtime::{objc_getClass, Class, Object, BOOL, NO, YES},
};

use crate::objective_c_runtime::{id, nil, traits::FromId};

/// A helper function to convert an Objective-C bool to a Rust bool.
#[inline(always)]
pub fn to_bool(result: BOOL) -> bool {
    match result {
        YES => true,
        NO => false,

        #[cfg(not(target_arch = "aarch64"))]
        _ => {
            std::unreachable!();
        }
    }
}

#[inline(always)]
pub fn to_optional<T>(ptr: id) -> Option<T>
where
    T: Sized + FromId,
{
    unsafe {
        if ptr != nil {
            Some(T::from_id(ptr))
        } else {
            None
        }
    }
}

lazy_static::lazy_static! {
    static ref CLASSES: ClassMap = ClassMap::new();
}

struct ClassMap(RwLock<HashMap<&'static str, HashMap<&'static str, usize>>>);

impl ClassMap {
    /// Returns a new ClassMap.
    pub fn new() -> Self {
        ClassMap(RwLock::new({
            let mut map = HashMap::new();

            // Top-level classes, like `NSView`, we cache here. The reasoning is that if a subclass
            // is being created, we can avoid querying the runtime for the superclass - i.e, many
            // subclasses will have `NSView` as their superclass.
            let _ = map.insert("_supers", HashMap::new());

            map
        }))
    }

    /// Attempts to load a previously registered subclass.
    pub fn load_subclass(
        &self,
        subclass_name: &'static str,
        superclass_name: &'static str,
    ) -> Option<*const Class> {
        let reader = self.0.read().unwrap();

        if let Some(inner) = (*reader).get(subclass_name) {
            if let Some(class) = inner.get(superclass_name) {
                return Some(*class as *const Class);
            }
        }

        None
    }

    /// Store a newly created subclass type.
    pub fn store_subclass(
        &self,
        subclass_name: &'static str,
        superclass_name: &'static str,
        class: *const Class,
    ) {
        let mut writer = self.0.write().unwrap();

        if let Some(map) = (*writer).get_mut(subclass_name) {
            let _ = map.insert(superclass_name, class as usize);
        } else {
            let mut map = HashMap::new();
            let _ = map.insert(superclass_name, class as usize);
            let _ = (*writer).insert(subclass_name, map);
        }
    }

    /// Attempts to load a Superclass. This first checks for the cached pointer; if not present, it
    /// will load the superclass from the Objective-C runtime and cache it for future lookup. This
    /// assumes that the class is one that should *already* and *always* exist in the runtime, and
    /// by design will panic if it can't load the correct superclass, as that would lead to very
    /// invalid behavior.
    pub fn load_superclass(&self, name: &'static str) -> Option<*const Class> {
        {
            let reader = self.0.read().unwrap();
            if let Some(superclass) = (*reader)["_supers"].get(name) {
                return Some(*superclass as *const Class);
            }
        }

        let objc_superclass_name = CString::new(name).unwrap();
        let superclass = unsafe { objc_getClass(objc_superclass_name.as_ptr()) };

        // This should not happen, for our use-cases, but it's conceivable that this could actually
        // be expected, so just return None and let the caller panic if so desired.
        if superclass.is_null() {
            return None;
        }

        {
            let mut writer = self.0.write().unwrap();
            if let Some(supers) = (*writer).get_mut("_supers") {
                let _ = supers.insert(name, superclass as usize);
            }
        }

        Some(superclass)
    }
}

#[inline(always)]
pub fn load_or_register_class<F>(
    superclass_name: &'static str,
    subclass_name: &'static str,
    config: F,
) -> *const Class
where
    F: Fn(&mut ClassDecl) + 'static,
{
    if let Some(subclass) = CLASSES.load_subclass(subclass_name, superclass_name) {
        return subclass;
    }

    if let Some(superclass) = CLASSES.load_superclass(superclass_name) {
        let objc_subclass_name = format!("{}_{}", subclass_name, superclass_name);

        match ClassDecl::new(&objc_subclass_name, unsafe { &*superclass }) {
            Some(mut decl) => {
                config(&mut decl);

                let class = decl.register();
                CLASSES.store_subclass(subclass_name, superclass_name, class);
                return class;
            }

            None => {
                panic!(
                    "Subclass of type {}_{} could not be allocated.",
                    subclass_name, superclass_name
                );
            }
        }
    }

    panic!(
        "Attempted to create subclass for {}, but unable to load superclass of type {}.",
        subclass_name, superclass_name
    );
}

/// Getting the instance variable of an object.
pub fn get_variable<'a, T>(this: &'a Object, ptr_name: &str) -> &'a T {
    unsafe {
        let ptr: usize = *this.get_ivar(ptr_name);
        let obj = ptr as *const T;
        &*obj
    }
}
