//! Master module that allows allocating new channels, services and objects.

use std::rc::Rc;

use super::*;
use super::meta;

mod interface_source;
pub use self::interface_source::*;

mod implementer_source;
pub use self::implementer_source::*;

/// Master controls all allocations, lists, sets, maps, trees and other stuff
/// of CCS.
pub trait Master {

    /// Add interface source to this master. If it is already added
    /// then nothing is done. The reference to interface source
    /// that was passed gets returned.
    fn add_interface_source(&mut self, isrc: InterfaceSource)
            -> &InterfaceSource;

    /// Add implementer source to this master. If it is already added
    /// then nothing is done. The reference to implementer source
    /// that was passed gets returned.
    fn add_implementer_source(&mut self, isrc: ImplementerSource)
            -> &ImplementerSource;
}
