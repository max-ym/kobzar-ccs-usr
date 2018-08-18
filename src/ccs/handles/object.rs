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
    path: Rc<meta::Path>,

    /// Object ID in some network.
    id: usize,
}

/// Architecture-dependent object implementation.
pub trait ObjectArchitecture {
}

impl Object {

    /// Object full path including the name of this object.
    pub fn path(&self) -> &meta::Path {
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
