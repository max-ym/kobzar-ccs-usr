//! Module which gives handles to access elements of the network.

use std::rc::Rc;

pub mod meta;

/// Handle of the object. Allows to access object's data in system
/// CCS controller.
pub struct Object {

    /// Path to the network which is created by this object instance.
    /// Note that there can be multiple objects with same network path.
    /// They can be distinguished by ID which always is unique among the
    /// local network.
    path: Rc<meta::Path>,

    /// Object ID in some network.
    id: usize,
}

/// Handle of the service. Allows to access some service in
/// the network of selected object.
pub struct Service {

    /// The object in which this service lives.
    object: Object,

    /// Name that identifies the service. It still can be dublicated
    /// in the local network. ID value of the service is the only
    /// identifier that is guaranteed to be unique.
    name: String, // TODO - delete

    /// ID that definitely identifies the service. This ID is guaranteed to
    /// be unique in the map of services of local network.
    id: usize,
}

/// Channel that allows communication among services.
pub struct Channel {

    /// ID of the channel registered in the system.
    id: usize,

    /// The service which originated this channel. Even though service
    /// could give up this channel, it still is identified as that which
    /// has origin from that service. The ID of the channel is related
    /// to the local network of the object of origin service.
    origin: Service,
}
