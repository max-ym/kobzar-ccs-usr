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
    path: Rc<Path>,

    /// ID that definitely identifies the service. This ID is guaranteed to
    /// be unique in the map of services of local network.
    id: usize,
}

/// Entry point of service. When service gets started, this entry point
/// will be executed.
pub trait ServiceEntry {

    /// Start the service.
    /// The channel transfered is connected to a thread which requested
    /// the service.
    fn start(&self, channel: &Channel);
}

/// Architecture-dependent Service part.
pub trait ServiceArchitecture {

    type SE: ServiceEntry;

    /// The handle for Service covered by this wrap.
    fn handle(&self) -> &Service;

    /// Entry point which is called to start the service.
    fn entry_point(&self) -> &Self::SE;
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
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}
