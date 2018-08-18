//! Module which gives handles to access elements of the network.

use std::rc::Rc;

pub mod meta;

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

/// Channel that allows communication among services.
#[derive(Clone)]
pub struct Channel {

    /// ID of the channel registered in the system.
    id: usize,

    /// The service which originated this channel. Even though service
    /// could give up this channel, it still is identified as that which
    /// has origin from that service. The ID of the channel is related
    /// to the local network of the object of origin service.
    origin: Service,
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

impl Channel {

    /// ID that uniquely identifies this channel inside the object.
    pub fn id(&self) -> usize {
        self.id
    }

    /// See [origin service](struct.Channel.html#structfield.origin).
    pub fn origin(&self) -> &Service {
        &self.origin
    }
}
