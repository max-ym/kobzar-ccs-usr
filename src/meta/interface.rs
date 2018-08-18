use super::*;
use std::rc::Rc;
use std::collections::BTreeSet;
use std::cmp::Ordering;

/// Interface defines some service that must be implemented by the
/// object to support some group of functionality.
pub struct Interface {

    /// Vendor of this interface.
    vendor_path: Rc<Path>,

    /// Version of this interface.
    version: Version,

    /// Set of service names this interface requires to be implemented.
    services: BTreeSet<String>,

    /// Interfaces that must be implemented in order to implement
    /// this interface.
    dependencies: BTreeSet<Rc<Interface>>,
}

impl Interface {

    /// Vendor of this interface.
    pub fn vendor(&self) -> &Path {
        self.vendor_path.as_ref()
    }

    /// Version of this interface definition.
    pub fn version(&self) -> &Version {
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

impl PartialEq for Interface {

    fn eq(&self, other: &Interface) -> bool {
        // Check versions first - its a quick check.
        if self.version() != other.version() {
            return false;
        }

        // Check paths.
        if self.vendor() != other.vendor() {
            return false;
        }

        // Check dependencies.
        if self.dependencies() != other.dependencies() {
            return false;
        }

        // Check services.
        if self.services() == other.services() {
            return false;
        }

        true
    }
}

impl Eq for Interface {}

impl PartialOrd for Interface {

    fn partial_cmp(&self, other: &Interface) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interface {

    fn cmp(&self, other: &Interface) -> Ordering {
        use self::Ordering::*;

        // Compare vendor paths.
        let vendor_cmp = self.vendor().cmp(other.vendor());
        if vendor_cmp != Equal {
            return vendor_cmp;
        }

        // Compare versions.
        let version_cmp = self.version().cmp(other.version());
        if version_cmp != Equal {
            return version_cmp;
        }

        // All other things don't matter.
        Equal
    }
}
