//! Master module that allows allocating new channels, services and objects.

use std::rc::Rc;

use super::meta;

mod interface_source;
pub use self::interface_source::*;

mod implementer_source;
pub use self::implementer_source::*;

/// Master controls all allocations, lists, sets, maps, trees and other stuff
/// of CCS.
pub trait Master {

    /// Channel type that is used in this master.
    type Chan: meta::ChannelArchitecture;

    /// Interface source type.
    type IntSrc: InterfaceSource;

    /// Interface implementer source type.
    type ImpSrc: ImplementerSource;

    /// Object type of this master.
    type Obj: meta::ObjectArchitecture;

    /// Service type of this master.
    type Srv: meta::ServiceArchitecture;

    /// The type of thread.
    type Thr: meta::ThreadArchitecture;

    /// Add interface source to this master. If it is already added
    /// then nothing is done. The reference to interface source
    /// that was passed gets returned.
    fn add_interface_source(&mut self, isrc: Self::IntSrc)
            -> &Self::IntSrc;

    /// Add implementer source to this master. If it is already added
    /// then nothing is done. The reference to implementer source
    /// that was passed gets returned.
    fn add_implementer_source(&mut self, isrc: Self::ImpSrc)
            -> &Self::ImpSrc;
}
