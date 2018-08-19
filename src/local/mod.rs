//! The local application implementation of CCS.
//! Used as default in-app network when global network is not available
//! or required.
//! Module is useful to test different aspects of CCS and was designed for
//! this purpose. Also, it can be used as a tutorial for implementing your
//! own CCS architecture.
//!
//! The recomended order of trait implementation is:
//! 1. [Service entry](../meta/trait.ServiceEntry.html).
//!    Service trait wants to know how to start a service
//!    when it gets called.
//! 2. [Service architecture](../meta/trait.ServiceArchitecture.html).
//!    Object trait needs to know the type of
//!    services it stores in it's collections.
//! 3. [Object architecture](../meta/trait.ObjectArchitecture.html).
//!    Actual object with associated service type.
//!    They have dependency on each other so they can be implemented
//!    in reversed order.
//! 4. [Channel architecture](../meta/trait.ChannelArchitecture.html).
//!    Depends on service implementation.
//!    Allows to connect services and carries out data sharing.
//! 5. [Memory architecture](../meta/trait.MemoryArchitecture.html).
//!    Used by thread to access its memory.
//! 6. [Thread architecture](../meta/trait.ThreadArchitecture.html).
//!    Thread type that is created after some
//!    service gets requested. Depends on service implementation.
//! 7. [Implementer source](../master/trait.ImplementerSource.html).
//!    Source of interface implementers that is used by the master
//!    to load load them. Depends on object implementation.
//! 8. [Interface source](../master/trait.InterfaceSource.html).
//!    Source of interfaces used by the master.
//! 9. [Master](../master/trait.Master.html).
//!    Main controller that controls all CCS environment.

use super::meta::{Interface, InterfaceSet, Object};

use std::rc::Rc;

pub struct MyServiceEntry {
    start: Fn(MyChannel),
}

pub struct MyService {

    /// The entry point.
    entry: MyServiceEntry,

    /// Service handle to return when requested.
    serv: Rc<MyService>,
}

pub struct MyObject {

    /// Arch independent part.
    object: Object,
}

/// The implementer of Interface source. When application begins,
/// we add the list of all interfaces we will use so that master could
/// find all requested sources when they get needed.
pub struct MyInterfaceSource {

    /// The set of application interfaces.
    ints: InterfaceSet,
}

impl MyInterfaceSource {

    /// Append new interface.
    pub fn append(&mut self, i: Interface) {
        self.ints.insert(i);
    }
}
