use super::*;
use std::rc::Rc;

/// Handle of the object. Allows to access object's data in system
/// CCS controller.
#[derive(Clone)]
pub struct Object {

    /// Path to the network which is created by this object instance.
    /// Note that there can be multiple objects with same network path.
    /// They can be distinguished by ID which always is unique among the
    /// local network. The last node of the path is the name of this
    /// object.
    path: Rc<Path>,

    /// Object ID in some network.
    id: usize,
}

/// Architecture-dependent object implementation.
pub trait ObjectArchitecture {

    /// Service iterator over all accessible for this thread local services.
    fn service_iter(&self) -> ServiceIterator;

    /// Object iterator over all accessible for current subobject.
    fn object_iter(&self) -> ObjectIterator;

    /// Set of services accessible for current thread.
    fn services(&self) -> ServiceSet;

    /// Set of objects accessible for current thread.
    fn objects(&self) -> ObjectSet;
}

impl Object {

    /// Object full path including the name of this object.
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    /// The name of this object.
    pub fn name(&self) -> &str {
        let path = self.path.as_ref();
        path.name()
    }

    /// ID that uniquely identifies this object. See
    /// [id field](struct.Object.html#structfield.id).
    pub fn id(&self) -> usize {
        self.id
    }
}