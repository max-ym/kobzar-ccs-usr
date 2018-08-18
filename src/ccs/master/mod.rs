//! Master module that allows allocating new channels, services and objects.

use super::handles::meta;
use super::handles;

/// Source of interfaces. When some interface is requested Master
/// uses it's interface sources to find required interface.
pub trait InterfaceSource {

    /// Try to find interfaces that apply to the requirements.
    fn lookup(&mut self, requirements: InterfaceRequirements)
            -> handles::InterfaceSet;
}

/// Source of interface implementers. When some implementation is required
/// Master uses these sources to load object that implements the interfaces.
pub trait ImplementerSource {
    // TODO
}

/// Rule which interface version must be loaded.
pub enum VersionRule {

    /// Interface with exact version.
    Equal(meta::Version),

    /// Interface version newer or equal to the version provided.
    /// Note that this applies only to interface versions with
    /// equal major versions. Unequal majors discards the interfaces.
    NewerOrEqual(meta::Version),
}

/// Requirements to the interface that must be obeyed.
pub struct InterfaceRequirements {

    /// Vendor and name of the interface.
    vendor: meta::Path,

    /// Version rules.
    version: VersionRule,
}
