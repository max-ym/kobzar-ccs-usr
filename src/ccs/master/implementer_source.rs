use super::*;
use super::meta;

/// Source of interface implementers. When some implementation is required
/// Master uses these sources to load object that implements the interfaces.
pub trait ImplementerSource {

    type O: meta::ObjectArchitecture;

    fn lookup(&mut self, requirements: ImplementerRequirements)
            -> meta::ObjectArchSet<Self::O>;
}


/// Requirements to the interface that must be obeyed.
pub struct ImplementerRequirements {

    /// Interfaces that must be implemented by the object.
    interfaces: Rc<meta::InterfaceSet>,
}
