use super::*;
use std::rc::Rc;

/// Handle of the service. Allows to access some service in
/// the network of selected object.
#[derive(Clone)]
pub struct Service {

    /// The object in which this service lives.
    object: Object,

    /// Path to this service. Should extend the path of origin object.
    /// This is a hard error not to extend the object path.
    path: Rc<meta::Path>,

    /// ID that definitely identifies the service. This ID is guaranteed to
    /// be unique in the map of services of local network.
    id: usize,
}

impl Service {

    /// Object where this service is located.
    pub fn object(&self) -> &Object {
        &self.object
    }

    /// The name of this service.
    pub fn name(&self) -> &str {
        self.path().name()
    }

    /// ID that uniquely identifies this service inside the object.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Path to this service.
    pub fn path(&self) -> &meta::Path {
        self.path.as_ref()
    }
}
