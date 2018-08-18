use super::*;
use std::rc::Rc;
use std::collections::BTreeSet;

/// Interface defines some service that must be implemented by the
/// object to support some group of functionality.
pub struct Interface {

    /// Vendor of this interface.
    vendor_path: Rc<meta::Path>,

    /// Version of this interface.
    version: meta::Version,

    /// Set of service names this interface requires to be implemented.
    services: BTreeSet<String>,

    /// Interfaces that must be implemented in order to implement
    /// this interface.
    dependencies: BTreeSet<Rc<Interface>>,
}

impl Interface {

    /// Vendor of this interface.
    pub fn vendor(&self) -> &meta::Path {
        self.vendor_path.as_ref()
    }

    /// Version of this interface definition.
    pub fn version(&self) -> &meta::Version {
        &self.version
    }

    /// Service names that must be implemented to support this
    /// interface.
    pub fn services(&self) -> &BTreeSet<String> {
        &self.services
    }

    /// Interfaces that must be implemented by the object first in order to
    /// implement this interface.
    pub fn dependencies(&self) -> &BTreeSet<Rc<Interface>> {
        &self.dependencies
    }
}
