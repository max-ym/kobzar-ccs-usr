//! The local application implementation of CCS.
//! Used as default in-app network when global network is not available
//! or required.
//! Module is useful to test different aspects of CCS and was designed for
//! this purpose. Also, it can be used as a tutorial for implementing your
//! own CCS architecture.
//!
//! The recomended order of trait implementation by is:
//! 1. Service entry. Service trait wants to know how to start a service
//!    when it gets called.
//! 2. Service architecture. Object trait needs to know the type of
//!    services it stores in it's collections.
//! 3. Object architecture. Actual object with associated service type.
//!    They have dependency on each other so they can be implemented
//!    in reversed order.
//! 4. Channel architecture. Depends on service implementation.
//!    Allows to connect services and carries out data sharing.
//! 5. Memory architecture. Used by thread to access its memory.
//! 6. Thread architecture. Thread type that is created after some
//!    service gets requested. Depends on service implementation.
//! 7. Implementer source.
//!    Source of interface implementers that is used by the master
//!    to load load them. Depends on object implementation.
//! 8. Interface source.
//!    Source of interfaces used by the master.
//! 9. Master.
//!    Main controller that controls all CCS environment.

use super::meta::{Interface, InterfaceSet, Object};

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

pub struct MyObject {

    /// Arch independent part.
    object: Object,
}
